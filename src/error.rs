// Bring in Rust's formatting trait (Display) and the path type we use in error variants.
use std::fmt;
use std::path::PathBuf;

// Derive(Debug) lets us print this enum for debugging (e.g. with {:?} or in logs).
#[derive(Debug)]
// All possible errors the DevBook CLI can return. Callers use Result<T, DevBookError>.
pub enum DevBookError {
    // User ran `dev` or `dev run` but no dev.yaml (or runbook.yaml) was found walking up from cwd.
    ConfigNotFound,

    // Config file exists but YAML was invalid or had wrong shape. We store where and why.
    ParseError {
        path: PathBuf,   // Path to the config file that failed to parse.
        message: String,  // Reason (e.g. "expected string value").
    },

    // User ran `dev <name>` but that action is not defined in dev.yaml.
    UnknownAction(String), // The action name they typed (e.g. "run", "lint").

    // We ran the command for an action but the subprocess failed (e.g. non-zero exit code).
    CommandFailed {
        action: String,  // The action name (e.g. "test").
        message: String, // What went wrong (e.g. exit code or error text).
    },

    // `dev init` could not write dev.yaml (e.g. permission denied, disk full).
    InitWriteError {
        path: PathBuf,   // Path where we tried to write.
        message: String, // Underlying error message.
    },
}

// Implement Display so we can print errors to the user with "{}" and use them with ? in Result.
impl fmt::Display for DevBookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DevBookError::ConfigNotFound => {
                write!(f, "No dev.yaml or runbook.yaml found in this directory or any parent")
            }
            DevBookError::ParseError { path, message } => {
                write!(f, "Failed to parse config at {}: {}", path.display(), message)
            }
            DevBookError::UnknownAction(name) => {
                write!(f, "Unknown action: '{}'. Add it to dev.yaml or run an existing action.", name)
            }
            DevBookError::CommandFailed { action, message } => {
                write!(f, "Command for '{}' failed: {}", action, message)
            }
            DevBookError::InitWriteError { path, message } => {
                write!(f, "Failed to write config at {}: {}", path.display(), message)
            }
        }
    }
}

// Mark DevBookError as an error type so it works with ? and with libraries that expect std::error::Error.
impl std::error::Error for DevBookError {}
