use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{DeviceFacts, ManagerError, Result};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LauncherStatus {
    Recommended,
    Supported,
    Experimental,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LauncherEntry {
    pub id: String,
    pub name: String,
    pub source_url: String,
    pub license: String,
    pub open_source: bool,
    pub user_supplied: bool,
    pub status: LauncherStatus,
    pub min_sdk: u32,
    pub max_sdk: Option<u32>,
    pub abis: Vec<String>,
    pub requires_validated_profile: bool,
    pub warning: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LauncherCatalog {
    pub schema_version: u32,
    pub launchers: Vec<LauncherEntry>,
}

impl LauncherCatalog {
    /// Loads the launcher metadata bundled with this build.
    ///
    /// # Errors
    ///
    /// Returns an error if the bundled catalog is invalid.
    pub fn bundled() -> Result<Self> {
        serde_json::from_str(include_str!("../../packages/catalog/launchers.json"))
            .map_err(|error| ManagerError::InvalidData(error.to_string()))
    }

    #[must_use]
    pub fn compatible_launchers(&self, facts: &DeviceFacts) -> Vec<LauncherEntry> {
        self.launchers
            .iter()
            .filter(|entry| {
                facts.identity.android_sdk >= entry.min_sdk
                    && entry
                        .max_sdk
                        .is_none_or(|maximum| facts.identity.android_sdk <= maximum)
                    && (entry.abis.is_empty()
                        || entry
                            .abis
                            .iter()
                            .any(|abi| facts.identity.abis.contains(abi)))
            })
            .cloned()
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApkIdentity {
    pub package: String,
    pub signer_sha256: String,
    pub min_sdk: u32,
    pub abis: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApkRequirement {
    pub package: String,
    pub signer_sha256: String,
    pub sha256: Option<String>,
    pub min_sdk: u32,
    pub abis: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedApk {
    pub sha256: String,
    pub identity: ApkIdentity,
}

/// Verifies an APK after a platform client has extracted its signed identity.
///
/// # Errors
///
/// Returns an error for a hash, package, signer, SDK, or ABI mismatch.
pub fn verify_apk(
    path: &Path,
    observed: &ApkIdentity,
    requirement: &ApkRequirement,
    facts: &DeviceFacts,
) -> Result<VerifiedApk> {
    if observed.package != requirement.package {
        return Err(ManagerError::Integrity("APK package mismatch".into()));
    }
    if !observed
        .signer_sha256
        .eq_ignore_ascii_case(&requirement.signer_sha256)
    {
        return Err(ManagerError::Integrity("APK signer mismatch".into()));
    }
    if observed.min_sdk != requirement.min_sdk || observed.min_sdk > facts.identity.android_sdk {
        return Err(ManagerError::Unsupported(format!(
            "APK requires Android SDK {}",
            observed.min_sdk
        )));
    }
    if observed.abis != requirement.abis
        || (!observed.abis.is_empty()
            && !observed
                .abis
                .iter()
                .any(|abi| facts.identity.abis.contains(abi)))
    {
        return Err(ManagerError::Unsupported("APK ABI is incompatible".into()));
    }

    let digest = hex::encode(Sha256::digest(fs::read(path)?));
    if requirement
        .sha256
        .as_ref()
        .is_some_and(|expected| !digest.eq_ignore_ascii_case(expected))
    {
        return Err(ManagerError::Integrity("APK hash mismatch".into()));
    }

    Ok(VerifiedApk {
        sha256: digest,
        identity: observed.clone(),
    })
}
