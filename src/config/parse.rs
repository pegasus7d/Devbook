use std::collections::HashMap;
use std::path::Path;

use crate::entities::DevConfig;
use crate::error::DevBookError;

/// Read the config file at `path` and parse it as flat YAML (key = action, value = command).
/// Returns in-memory DevConfig; no YAML/JSON stored in the entity.
pub fn load_config(path: &Path) -> Result<DevConfig, DevBookError> {
    let contents = std::fs::read_to_string(path).map_err(|e| DevBookError::ParseError {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;

    let actions: HashMap<String, String> = serde_yaml::from_str(&contents).map_err(|e| {
        DevBookError::ParseError {
            path: path.to_path_buf(),
            message: e.to_string(),
        }
    })?;

    Ok(DevConfig::new(actions))
}