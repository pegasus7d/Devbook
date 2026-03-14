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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn load_config_parses_flat_yaml() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "run: npm run dev").unwrap();
        writeln!(f, "test: npm test").unwrap();
        f.flush().unwrap();
        let config = load_config(f.path()).unwrap();
        assert_eq!(config.get_command("run"), Some("npm run dev"));
        assert_eq!(config.get_command("test"), Some("npm test"));
    }

    #[test]
    fn load_config_invalid_yaml_returns_error() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "not: valid: yaml: [[[").unwrap();
        f.flush().unwrap();
        let result = load_config(f.path());
        assert!(result.is_err());
        if let Err(DevBookError::ParseError { .. }) = result {
        } else {
            panic!("expected ParseError");
        }
    }
}