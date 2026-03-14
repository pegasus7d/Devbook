use std::path::PathBuf;

/// Where the config was found: project root and path to dev.yaml (or runbook.yaml).
#[derive(Debug, Clone)]
pub struct ConfigLocation {
    /// Directory containing the config file; use as cwd when running commands.
    pub project_root: PathBuf,
    /// Full path to the config file (e.g. .../dev.yaml).
    pub config_path: PathBuf,
}