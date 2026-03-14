# DevBook – Requirements (v1) – Spec

Requirements for the DevBook CLI prototype. See [CORE_IDEA.md](../../CORE_IDEA.md) for full spec.

---

## 1. Config file


| ID   | Requirement                                                                       | Priority |
| ---- | --------------------------------------------------------------------------------- | -------- |
| R1.1 | One config file per repo: `dev.yaml` (or `runbook.yaml`) at project root.         | Must     |
| R1.2 | Format: flat YAML — key = action name, value = shell command.                     | Must     |
| R1.3 | Default action keys: `install`, `run`, `test`, `build`, `local`, `stage`, `prod`. | Must     |
| R1.4 | User can add any extra keys; no fixed list of actions.                            | Must     |


---

## 2. Config discovery


| ID   | Requirement                                                                    | Priority |
| ---- | ------------------------------------------------------------------------------ | -------- |
| R2.1 | Find config by walking up from current working directory (cwd or parent dirs). | Must     |
| R2.2 | Project root = directory containing `dev.yaml`.                                | Must     |


---

## 3. CLI commands


| ID   | Requirement                                                                                         | Priority |
| ---- | --------------------------------------------------------------------------------------------------- | -------- |
| R3.1 | `dev` — find config; list all actions (defaults + custom).                                          | Must     |
| R3.2 | `dev <action>` — run the command for that key from config (e.g. `dev run`, `dev test`, `dev lint`). | Must     |
| R3.3 | `dev install` — run the command under `install:`.                                                   | Must     |
| R3.4 | `dev init` — if no config: prompt for default actions and write `dev.yaml`.                         | Must     |


---

## 4. Execution


| ID   | Requirement                                                     | Priority |
| ---- | --------------------------------------------------------------- | -------- |
| R4.1 | Commands run in the project directory (where config was found). | Must     |
| R4.2 | Command output streams to the terminal.                         | Must     |


---

## 5. Out of scope (later)

- `dev init` with natural language → AI-generated YAML.
- MCP server for assistants to list/run actions and read config.
- Inference from `package.json` / `Cargo.toml` when a key is missing.

---

## Summary

- **Must:** Config file (R1), config discovery (R2), CLI commands (R3), execution (R4).
- **Later:** AI init, MCP, inference from package manifests.

