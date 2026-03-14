//! Integration tests: run the devbook binary in a temp dir with dev.yaml or without.

use std::process::Command;
use std::str::from_utf8;

/// Path to the devbook binary (set by Cargo for integration tests).
fn devbook_bin() -> &'static str {
    env!("CARGO_BIN_EXE_devbook")
}

#[test]
fn list_without_config_fails_with_message() {
    let dir = tempfile::tempdir().unwrap();
    let out = Command::new(devbook_bin())
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = from_utf8(&out.stderr).unwrap();
    assert!(
        stderr.contains("No dev.yaml") || stderr.contains("runbook.yaml"),
        "stderr should mention missing config: {}",
        stderr
    );
}

#[test]
fn init_creates_dev_yaml() {
    let dir = tempfile::tempdir().unwrap();
    let out = Command::new(devbook_bin())
        .current_dir(dir.path())
        .arg("init")
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", from_utf8(&out.stderr).unwrap());
    let dev_yaml = dir.path().join("dev.yaml");
    assert!(dev_yaml.exists(), "dev.yaml should exist after init");
    let content = std::fs::read_to_string(&dev_yaml).unwrap();
    for key in ["install", "run", "test", "build", "local", "stage", "prod"] {
        assert!(content.contains(key), "dev.yaml should contain key {}", key);
    }
}

#[test]
fn list_after_init_shows_actions() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("dev.yaml"),
        "run: npm run dev\ntest: npm test\n",
    )
    .unwrap();
    let out = Command::new(devbook_bin())
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", from_utf8(&out.stderr).unwrap());
    let stdout = from_utf8(&out.stdout).unwrap();
    assert!(stdout.contains("run"));
    assert!(stdout.contains("test"));
}

#[test]
fn init_when_config_exists_fails() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("dev.yaml"), "run: npm run dev\n").unwrap();
    let out = Command::new(devbook_bin())
        .current_dir(dir.path())
        .arg("init")
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = from_utf8(&out.stderr).unwrap();
    assert!(
        stderr.contains("already exists") || stderr.contains("dev.yaml"),
        "stderr should say config exists: {}",
        stderr
    );
}

#[test]
fn run_unknown_action_fails() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("dev.yaml"),
        "run: npm run dev\ntest: npm test\n",
    )
    .unwrap();
    let out = Command::new(devbook_bin())
        .current_dir(dir.path())
        .arg("nonexistent-action")
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = from_utf8(&out.stderr).unwrap();
    assert!(
        stderr.contains("Unknown action") || stderr.contains("nonexistent-action"),
        "stderr should mention unknown action: {}",
        stderr
    );
}

#[test]
fn run_action_succeeds_with_simple_command() {
    let dir = tempfile::tempdir().unwrap();
    // Use a no-op that works on both Unix and Windows
    let noop = if cfg!(unix) { "true" } else { "exit 0" };
    std::fs::write(
        dir.path().join("dev.yaml"),
        format!("succeed: {}\n", noop),
    )
    .unwrap();
    let out = Command::new(devbook_bin())
        .current_dir(dir.path())
        .arg("succeed")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        from_utf8(&out.stderr).unwrap()
    );
}

#[test]
fn help_prints_usage() {
    let out = Command::new(devbook_bin()).arg("--help").output().unwrap();
    assert!(out.status.success());
    let stdout = from_utf8(&out.stdout).unwrap();
    assert!(stdout.contains("Usage") || stdout.contains("devbook"));
    assert!(stdout.contains("init"));
}
