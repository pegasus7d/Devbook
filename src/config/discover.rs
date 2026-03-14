use crate::entities::ConfigLocation;
use crate::error::DevBookError;

/// Config file names we look for, in order.
const CONFIG_FILES: &[&str] = &["dev.yaml", "runbook.yaml"];

/// Walk up from the current directory until we find dev.yaml or runbook.yaml.
/// Returns the project root (directory containing the file) and the full config path.
pub fn find_config() -> Result<ConfigLocation, DevBookError> {
    let current = std::env::current_dir().map_err(|e| DevBookError::ParseError {
        path: std::path::PathBuf::new(),
        message: format!("Could not get current directory: {}", e),
    })?;

    let mut dir = current.as_path();
    loop {
        for name in CONFIG_FILES {
            let config_path = dir.join(name);
            if config_path.exists() {
                return Ok(ConfigLocation {
                    project_root: dir.to_path_buf(),
                    config_path,
                });
            }
        }
        dir = match dir.parent() {
            Some(p) => p,
            None => return Err(DevBookError::ConfigNotFound),
        };
    }
}