use std::path::PathBuf;
/// Input to the runner: run a command in the project root.
#[derive(Debug, Clone)]
pub struct RunContext {
    /// Working directory for the command (project root).
    pub project_root: PathBuf,
    /// The shell command to run (value from dev.yaml).
    pub command: String,
}