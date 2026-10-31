use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::{DeviceFacts, ManagerError, Result, SupportLevel};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProfileStatus {
    BuiltIn,
    Experimental,
    Candidate,
    Validated,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Profile {
    pub schema_version: u32,
    pub id: String,
    pub status: ProfileStatus,
    #[serde(default)]
    pub fingerprints: Vec<String>,
    pub min_sdk: u32,
    pub max_sdk: u32,
    pub abis: Vec<String>,
    #[serde(default)]
    pub protected_packages: Vec<String>,
    #[serde(default)]
    pub optional_disable_packages: Vec<String>,
    #[serde(default)]
    pub launcher_ids: Vec<String>,
}

impl Profile {
    #[must_use]
    pub fn base_safe() -> Self {
        Self {
            schema_version: 1,
            id: "base-safe".into(),
            status: ProfileStatus::BuiltIn,
            fingerprints: Vec::new(),
            min_sdk: 25,
            max_sdk: 40,
            abis: vec!["armeabi-v7a".into(), "arm64-v8a".into()],
            protected_packages: vec![
                "com.amazon.tv.launcher".into(),
                "com.amazon.tv.settings.v2".into(),
                "com.android.bluetooth".into(),
                "com.amazon.tv.ime".into(),
                "com.amazon.venezia".into(),
            ],
            optional_disable_packages: Vec::new(),
            launcher_ids: vec![
                "projectivy".into(),
                "dumbtv-home".into(),
                "ltv-launcher".into(),
                "user-supplied".into(),
            ],
        }
    }

    fn supports(&self, facts: &DeviceFacts) -> bool {
        facts.identity.android_sdk >= self.min_sdk
            && facts.identity.android_sdk <= self.max_sdk
            && facts
                .identity
                .abis
                .iter()
                .any(|abi| self.abis.iter().any(|supported| supported == abi))
    }
}

#[derive(Clone, Debug)]
pub struct ProfileStore {
    base: Profile,
    generation: Vec<Profile>,
    exact: Vec<Profile>,
}

#[derive(Clone, Debug)]
pub struct Selection {
    pub level: SupportLevel,
    pub base: Profile,
    pub generation_profiles: Vec<Profile>,
    pub exact_profile: Option<Profile>,
    pub reason: String,
}

impl ProfileStore {
    /// Loads profiles compiled into the application release.
    ///
    /// # Errors
    ///
    /// Returns an error if a bundled JSON profile is malformed.
    pub fn bundled() -> Result<Self> {
        let parse = |json: &str| {
            serde_json::from_str(json).map_err(|error| ManagerError::InvalidData(error.to_string()))
        };
        Ok(Self {
            base: parse(include_str!("../../profiles/base-safe.json"))?,
            generation: vec![parse(include_str!("../../profiles/fire-os-7.json"))?],
            exact: vec![parse(include_str!(
                "../../profiles/experimental/raven.json"
            ))?],
        })
    }

    /// Verifies and parses a remotely supplied profile.
    ///
    /// # Errors
    ///
    /// Returns an integrity error for an invalid Ed25519 signature or malformed JSON.
    pub fn load_signed(json: &[u8], signature: [u8; 64], key: VerifyingKey) -> Result<Profile> {
        key.verify(json, &Signature::from_bytes(&signature))
            .map_err(|error| ManagerError::Integrity(error.to_string()))?;
        serde_json::from_slice(json).map_err(|error| ManagerError::InvalidData(error.to_string()))
    }

    #[must_use]
    pub fn select(&self, facts: &DeviceFacts) -> Selection {
        if !self.base.supports(facts) {
            return Selection {
                level: SupportLevel::Blocked,
                base: self.base.clone(),
                generation_profiles: Vec::new(),
                exact_profile: None,
                reason: "device SDK or ABI is outside the supported base range".into(),
            };
        }
        let generation_profiles: Vec<_> = self
            .generation
            .iter()
            .filter(|profile| profile.supports(facts))
            .cloned()
            .collect();
        let exact_profile = self
            .exact
            .iter()
            .find(|profile| {
                profile.supports(facts)
                    && profile
                        .fingerprints
                        .iter()
                        .any(|fingerprint| fingerprint == &facts.identity.fingerprint)
            })
            .cloned();
        let level = exact_profile
            .as_ref()
            .map_or(SupportLevel::BaseOnly, |profile| {
                if profile.status == ProfileStatus::Validated {
                    SupportLevel::Supported
                } else {
                    SupportLevel::BaseOnly
                }
            });
        Selection {
            level,
            base: self.base.clone(),
            generation_profiles,
            exact_profile,
            reason: "safe base selected; exact-build mutations require validated status".into(),
        }
    }
}
