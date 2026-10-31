use regex::Regex;

use crate::{DeviceFacts, ManagerError, Result};

/// Serializes device facts while removing common device and network identifiers.
///
/// # Errors
///
/// Returns an error if serialization or the built-in redaction expressions fail.
pub fn export_redacted_report(facts: &DeviceFacts) -> Result<String> {
    let mut redacted = facts.clone();
    let serial = redacted.identity.serial.clone();
    redacted.identity.serial = "REDACTED".into();
    let mut text = serde_json::to_string_pretty(&redacted)
        .map_err(|error| ManagerError::InvalidData(error.to_string()))?;
    if !serial.is_empty() {
        text = text.replace(&serial, "REDACTED");
    }
    for pattern in [
        r"\b(?:\d{1,3}\.){3}\d{1,3}\b",
        r"(?i)\b(?:[0-9a-f]{2}:){5}[0-9a-f]{2}\b",
    ] {
        text = Regex::new(pattern)
            .map_err(|error| ManagerError::InvalidData(error.to_string()))?
            .replace_all(&text, "REDACTED")
            .into_owned();
    }
    Ok(text)
}
