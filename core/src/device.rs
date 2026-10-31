use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeviceIdentity {
    pub serial: String,
    pub model: String,
    pub codename: String,
    pub fingerprint: String,
    pub fire_os: Option<String>,
    pub android_sdk: u32,
    pub abis: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupportLevel {
    Supported,
    BaseOnly,
    Blocked,
}

impl fmt::Display for SupportLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Supported => formatter.write_str("supported"),
            Self::BaseOnly => formatter.write_str("base-only"),
            Self::Blocked => formatter.write_str("blocked"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PackageState {
    Enabled,
    Disabled,
    RemovedForUser,
    Absent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeviceFacts {
    pub identity: DeviceIdentity,
    pub packages: BTreeMap<String, PackageState>,
    pub home_component: Option<String>,
    pub settings_components: BTreeSet<String>,
    pub tv_input_packages: BTreeSet<String>,
    pub input_method_package: Option<String>,
    pub accessibility_services: BTreeSet<String>,
    pub capabilities: BTreeSet<String>,
    pub diagnostics: BTreeMap<String, String>,
    pub storage_available_bytes: Option<u64>,
}
