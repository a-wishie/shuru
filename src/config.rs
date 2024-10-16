use crate::version_manager::VersionInfo;
use serde::Deserialize;
use shuru::{
    error::{ConfigValidationError, Error},
    version_manager::{deserialize_versions, VersionedCommand},
};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub command: String,
    pub dir: Option<String>,
    pub default: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub tasks: HashMap<String, TaskConfig>,
    #[serde(default, deserialize_with = "deserialize_versions")]
    pub versions: HashMap<VersionedCommand, VersionInfo>,
}

impl TaskConfig {
    pub fn validate(&self, task_name: &str) -> Result<(), ConfigValidationError> {
        self.validate_command(task_name)?;
        self.validate_dir(task_name)?;
        Ok(())
    }

    fn validate_command(&self, task_name: &str) -> Result<(), ConfigValidationError> {
        if self.command.is_empty() {
            return Err(ConfigValidationError::EmptyCommandError(
                task_name.to_string(),
            ));
        }
        Ok(())
    }

    fn validate_dir(&self, task_name: &str) -> Result<(), ConfigValidationError> {
        if let Some(dir) = &self.dir {
            if dir.is_empty() {
                return Err(ConfigValidationError::EmptyDirError(task_name.to_string()));
            }
        }
        Ok(())
    }
}

impl Config {
    pub fn validate_tasks(&self) -> Result<(), Error> {
        for (task_name, task_config) in &self.tasks {
            task_config.validate(task_name)?;
        }
        Ok(())
    }
}
