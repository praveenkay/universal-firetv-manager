use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{AdbTransport, DeviceFacts, DeviceIdentity, ManagerError, Result};

#[derive(Clone, Debug, Default)]
pub struct BackupOptions {
    pub full: bool,
    pub settings: Vec<(String, String)>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BackupManifest {
    pub schema_version: u32,
    pub captured_unix_seconds: u64,
    pub identity: DeviceIdentity,
    pub settings: BTreeMap<String, Option<String>>,
    pub files: BTreeMap<String, String>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct VerifiedBackup {
    pub path: PathBuf,
    pub manifest: BackupManifest,
}

fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-'))
}

fn hash_file(path: &Path) -> Result<String> {
    let data = fs::read(path)?;
    Ok(hex::encode(Sha256::digest(data)))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    fs::write(
        path,
        serde_json::to_vec_pretty(value)
            .map_err(|error| ManagerError::InvalidData(error.to_string()))?,
    )?;
    Ok(())
}

/// Captures an atomic, device-bound baseline backup.
///
/// # Errors
///
/// Returns an error when metadata/settings cannot be captured or the destination cannot be written.
pub async fn capture_backup(
    transport: &dyn AdbTransport,
    facts: &DeviceFacts,
    destination: &Path,
    options: &BackupOptions,
) -> Result<BackupManifest> {
    let incomplete = destination.with_extension("incomplete");
    if incomplete.exists() {
        fs::remove_dir_all(&incomplete)?;
    }
    fs::create_dir_all(&incomplete)?;
    let result = capture_into(transport, facts, &incomplete, options).await;
    match result {
        Ok(manifest) => {
            if destination.exists() {
                fs::remove_dir_all(destination)?;
            }
            fs::rename(&incomplete, destination)?;
            Ok(manifest)
        }
        Err(error) => {
            let _ = fs::remove_dir_all(&incomplete);
            Err(error)
        }
    }
}

async fn capture_into(
    transport: &dyn AdbTransport,
    facts: &DeviceFacts,
    directory: &Path,
    options: &BackupOptions,
) -> Result<BackupManifest> {
    write_json(&directory.join("device-facts.json"), facts)?;
    fs::write(
        directory.join("BACKUP-LIMITS.md"),
        "# Backup limits\n\nRetail non-root ADB cannot read private app data, logins, DRM keys, Amazon account secrets, or a flashable Fire OS image.\n",
    )?;
    let mut settings = BTreeMap::new();
    for (namespace, key) in &options.settings {
        if !valid_identifier(namespace) || !valid_identifier(key) {
            return Err(ManagerError::InvalidData(
                "invalid setting identifier".into(),
            ));
        }
        let output = transport
            .shell(&format!("settings get {namespace} {key}"))
            .await?;
        let value = output.stdout.trim();
        settings.insert(
            format!("{namespace}/{key}"),
            (value != "null" && !value.is_empty()).then(|| value.to_owned()),
        );
    }
    write_json(&directory.join("settings.json"), &settings)?;
    let mut files = BTreeMap::new();
    for name in ["device-facts.json", "settings.json", "BACKUP-LIMITS.md"] {
        files.insert(name.into(), hash_file(&directory.join(name))?);
    }
    let warnings = options
        .full
        .then(|| "Full APK/shared-storage acquisition is transport-dependent; unavailable files are recorded, not treated as private-data backup.".into())
        .into_iter()
        .collect();
    let manifest = BackupManifest {
        schema_version: 1,
        captured_unix_seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| ManagerError::InvalidData(error.to_string()))?
            .as_secs(),
        identity: facts.identity.clone(),
        settings,
        files,
        warnings,
    };
    write_json(&directory.join("manifest.json"), &manifest)?;
    Ok(manifest)
}

/// Verifies backup identity and every manifest hash.
///
/// # Errors
///
/// Returns an error for malformed manifests, device mismatch, missing files, or hash mismatch.
pub fn verify_backup(path: &Path, identity: &DeviceIdentity) -> Result<VerifiedBackup> {
    let manifest: BackupManifest = serde_json::from_slice(&fs::read(path.join("manifest.json"))?)
        .map_err(|error| ManagerError::InvalidData(error.to_string()))?;
    if &manifest.identity != identity {
        return Err(ManagerError::DeviceMismatch);
    }
    for (name, expected) in &manifest.files {
        let actual = hash_file(&path.join(name))?;
        if &actual != expected {
            return Err(ManagerError::Integrity(format!("hash mismatch for {name}")));
        }
    }
    Ok(VerifiedBackup {
        path: path.to_owned(),
        manifest,
    })
}
