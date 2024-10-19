use serde::Deserialize;
use shuru::{
    error::Error,
    tools::version_manager::{
        NodeVersionManager, PythonVersionManager, ShuruVersionManager, VersionValidator,
    },
};
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum VersionedCommand {
    Node,
    Python,
}

impl VersionedCommand {
    pub fn get_command_name(&self) -> &'static str {
        match self {
            VersionedCommand::Node => "node",
            VersionedCommand::Python => "python",
        }
    }

    pub fn get_version_manager(
        &self,
        version_info: &VersionInfo,
    ) -> Result<ShuruVersionManager, Error> {
        let version = version_info.get_version();

        match self {
            VersionedCommand::Node => {
                NodeVersionManager::validate_version(version)?;
                Ok(ShuruVersionManager::Node(
                    NodeVersionManager::with_version_info(version_info),
                ))
            }
            VersionedCommand::Python => {
                PythonVersionManager::validate_version(version)?;
                Ok(ShuruVersionManager::Python(
                    PythonVersionManager::with_version_info(version_info),
                ))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VersionInfo {
    Simple(String),
    Complex { version: String, platform: String },
}

impl VersionInfo {
    pub fn get_version(&self) -> &str {
        match self {
            VersionInfo::Simple(version) => version,
            VersionInfo::Complex { version, .. } => version,
        }
    }
}

pub fn deserialize_versions<'de, D>(
    deserializer: D,
) -> Result<HashMap<VersionedCommand, VersionInfo>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: HashMap<String, VersionInfo> = HashMap::deserialize(deserializer)?;

    let mut result = HashMap::new();

    for (key, value) in map {
        if value.get_version().is_empty() {
            return Err(serde::de::Error::custom(format!(
                "Missing version information for {}",
                key
            )));
        }

        match key.as_str() {
            "node" => {
                result.insert(VersionedCommand::Node, value);
            }
            "python" => {
                result.insert(VersionedCommand::Python, value);
            }
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unknown version command: {}",
                    key
                )));
            }
        }
    }

    Ok(result)
}
