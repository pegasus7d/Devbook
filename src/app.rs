use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::{find_config, load_config};
use crate::entities::{DevConfig, RunContext, DEFAULT_ACTIONS};
use crate::error::DevBookError;
use crate::run::run;

/// Application facade: list actions, run an action, or init a new dev.yaml.
pub struct App;

impl App {
    /// List all actions from the config (dev or runbook.yaml). Errors if config not found or invalid.
    pub fn list_actions() -> Result<(), DevBookError> {
        let location = find_config()?;
        let config = load_config(&location.config_path)?;
        for name in config.action_names() {
            println!("  {}", name);
        }
        Ok(())
    }

    /// Run the command for the given action. Errors if config not found, parse error, unknown action, or command failure.
    pub fn run_action(name: &str) -> Result<(), DevBookError> {
        let location = find_config()?;
        let config = load_config(&location.config_path)?;
        let command = config
            .get_command(name)
            .ok_or_else(|| DevBookError::UnknownAction(name.to_string()))?;
        let ctx = RunContext {
            project_root: location.project_root.clone(),
            command: command.to_string(),
        };
        run(&ctx).map_err(|e| {
            if let DevBookError::CommandFailed { message, .. } = e {
                DevBookError::CommandFailed {
                    action: name.to_string(),
                    message,
                }
            } else {
                e
            }
        })
    }

    /// Create dev.yaml in the current directory with default action keys (empty commands).
    /// Errors if config already exists in this or a parent dir, or if write fails.
    pub fn init_config() -> Result<(), DevBookError> {
        if find_config().is_ok() {
            return Err(DevBookError::ParseError {
                path: PathBuf::new(),
                message: "A dev.yaml or runbook.yaml already exists in this directory or a parent."
                    .to_string(),
            });
        }
        let current = std::env::current_dir().map_err(|e| DevBookError::InitWriteError {
            path: PathBuf::new(),
            message: e.to_string(),
        })?;
        let mut actions = HashMap::new();
        for name in DEFAULT_ACTIONS {
            actions.insert((*name).to_string(), String::new());
        }
        let config = DevConfig::new(actions);
        let yaml = serde_yaml::to_string(&config.actions).map_err(|e| {
            DevBookError::InitWriteError {
                path: current.join("dev.yaml"),
                message: e.to_string(),
            }
        })?;
        let path = current.join("dev.yaml");
        std::fs::write(&path, yaml).map_err(|e| DevBookError::InitWriteError {
            path: path.clone(),
            message: e.to_string(),
        })?;
        println!("Created {}", path.display());
        Ok(())
    }
}
