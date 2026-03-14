# DevBook – Tech Stack

This document describes the technologies chosen to implement the DevBook CLI (see [CORE_IDEA.md](./CORE_IDEA.md)).

---

## Language & runtime

| Choice | Purpose |
|--------|---------|
| **Rust** | Single binary, no runtime dependency, cross-platform (macOS, Linux, Windows). Good fit for a small CLI and for learning systems-level concepts (ownership, no GC). |

---

## Core libraries

| Layer | Library | Purpose |
|-------|---------|---------|
| **CLI** | [clap](https://github.com/clap-rs/clap) | Argument parsing, subcommands (`dev`, `dev init`, `dev <action>`), help text, and flags. |
| **Config** | [serde_yaml](https://github.com/dtolnay/serde-yaml) (with [serde](https://github.com/serde-rs/serde)) | Parse `dev.yaml`: flat map of action name → shell command. |
| **Process** | `std::process::Command` (stdlib) | Run the configured shell command in the project directory. No extra dependency. |

---

## Implementation details

- **Config discovery:** Walk up from the current working directory until `dev.yaml` (or `runbook.yaml`) is found; that directory is the project root. Same idea as git / Task.
- **Config format:** Flat YAML — each key is an action (e.g. `run`, `test`, `build`), each value is the command string to execute.
- **Execution:** For `dev <action>`, look up the key in the config and run its value via the system shell in the project root; stream output to the terminal.

---

## Distribution (later)

- **Binaries:** Build one static binary per target (e.g. `x86_64` and `aarch64` for macOS/Linux/Windows), publish to GitHub Releases.
- **Package managers:** Optional Homebrew formula (or similar) that fetches the appropriate binary from releases.
- **Tooling:** [release-please](https://github.com/googleapis/release-please), [cargo-release](https://github.com/crate-ci/cargo-release), or similar can automate versioning and release uploads.

---

## Why Rust (vs Bun/Go)

- **Single binary, no runtime** — Users don’t need Node or any interpreter.
- **Small binary, fast startup** — Fits a launcher that immediately delegates to a subprocess.
- **Systems programming** — Ownership, no GC, low-level control; useful for learning and for long-term maintainability.
- **Ecosystem** — Similar tool ([devrc](https://github.com/devrc-dev/devrc)) exists in Rust; clap and serde are standard choices for CLIs and config.

---

## Summary

| Component | Technology |
|-----------|------------|
| Language | Rust |
| CLI framework | clap |
| Config parsing | serde + serde_yaml |
| Running commands | std::process::Command |
| Config discovery | Custom (walk up to find `dev.yaml`) |
| Distribution | Single binary per platform, GitHub Releases, optional Homebrew |
