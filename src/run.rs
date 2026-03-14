use std::process::Command;

use crate::entities::RunContext;
use crate::error::DevBookError;

/// Run the command from `ctx` in `ctx.project_root`, streaming stdout/stderr to the terminal.
/// Returns Ok(()) on success, CommandFailed on non-zero exit.
pub fn run(ctx: &RunContext) -> Result<(), DevBookError> {
    let mut child = if cfg!(unix) {
        Command::new("sh")
            .arg("-c")
            .arg(&ctx.command)
            .current_dir(&ctx.project_root)
            .spawn()
    } else {
        Command::new("cmd")
            .args(["/C", &ctx.command])
            .current_dir(&ctx.project_root)
            .spawn()
    }
    .map_err(|e| DevBookError::CommandFailed {
        action: String::new(),
        message: e.to_string(),
    })?;

    let status = child.wait().map_err(|e| DevBookError::CommandFailed {
        action: String::new(),
        message: e.to_string(),
    })?;

    if status.success() {
        Ok(())
    } else {
        Err(DevBookError::CommandFailed {
            action: String::new(),
            message: status
                .code()
                .map(|c| format!("exit code {}", c))
                .unwrap_or_else(|| "process failed".to_string()),
        })
    }
}
