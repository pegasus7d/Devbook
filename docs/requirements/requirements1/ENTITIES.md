# Entities for Requirements 1

Domain entities and value types needed to implement the DevBook CLI prototype. Each supports the spec in [SPEC.md](./SPEC.md) and the structure in [DESIGN_PATTERNS.md](./DESIGN_PATTERNS.md).

---

## Rule: No serialized format in entities

**We do not store JSON, YAML, or any other serialized string inside any entity.**

- **On disk:** the config file is YAML (`dev.yaml`). That is an I/O detail.
- **In memory:** entities are plain domain types (structs, maps, enums). No entity holds the config file as a string or a JSON/YAML blob.
- **Boundary:** when **reading**, the loader parses the file and returns a **DevConfig** (in-memory map). When **writing** (e.g. `dev init`), we build a **DevConfig** and the writer serializes it to YAML at write time. Serialization happens only at the I/O boundary; entities stay format-agnostic.

So: **entities = domain data only**. No JSON, no YAML, no raw file content inside any entity.

---

## 1. DevConfig

**What:** The in-memory configuration: all actions and their shell commands. **Not** the file content; not YAML or JSON.

**Supports:** R1 (config file)

**Shape:**
- Map of **action name** → **command string** (e.g. `HashMap<String, String>` or a struct with one such field).
- Pure in-memory data only; no I/O, no serialized format stored in this entity.
- Optional: list of **default action names** (install, run, test, build, local, stage, prod) as a constant or method for “known” keys; rest are custom.

**Operations:** Get command for action; list action names; (for init) produce default/empty config. Serialization to/from YAML is done **outside** this entity (in the loader/writer).

---

## 2. ConfigLocation / DiscoveryResult

**What:** Result of config discovery: where the config file is and where the project root is.

**Supports:** R2 (config discovery)

**Shape:**
- **Project root** — directory containing `dev.yaml` (working directory for running commands).
- **Config path** — full path to `dev.yaml` (or `runbook.yaml`).
- Discovery can “fail” (no config found), so this is typically `Result<(PathBuf, PathBuf), ConfigNotFound>` or an enum `Found { project_root, config_path } | NotFound`.

**Operations:** None beyond holding the two paths; consumed by loader and runner.

---

## 3. ActionName

**What:** The key in the config: the name of an action (e.g. `run`, `test`, `install`, `lint`).

**Supports:** R1, R3 (CLI commands)

**Shape:** String (or newtype wrapper for type safety). No fixed enum; user can add any key (R1.4).

**Operations:** Compare with config keys; pass to `run_action(name)`; validate non-empty if needed.

---

## 4. CommandString

**What:** The value in the config: the shell command to run for an action.

**Supports:** R1, R4 (execution)

**Shape:** String. Opaque to the app; passed to the shell as-is.

**Operations:** Pass to `CommandRunner` with a project root.

---

## 5. RunContext (execution input)

**What:** Everything needed to run one command: working directory and the command string.

**Supports:** R4 (execution)

**Shape:**
- **Project root** (or “working directory”) — where to run the command (R4.1).
- **Command string** — what to execute (R4.2: output streams to terminal).

Can be a struct or a tuple `(PathBuf, String)`; the runner takes this (or equivalent args).

**Operations:** None; built from config + discovery result and passed to the runner.

---

## 6. DefaultActions / InitTemplate

**What:** The seven default action keys and, optionally, placeholder command strings. Used to build an in-memory **DevConfig** in `dev init`; the writer then serializes that DevConfig to YAML when writing the file. No YAML/JSON stored here.

**Supports:** R3.4 (dev init), R1.3 (default keys)

**Shape:**
- Ordered list of default action names: `install`, `run`, `test`, `build`, `local`, `stage`, `prod`.
- Optional: default command per key (e.g. empty string or a placeholder like `# add command`). If the wizard prompts the user, this may just be the list of keys; values come from prompts.

**Operations:** Iterate to build initial **DevConfig** (in-memory); used by init flow only. Writing to disk (YAML) is done by the loader/writer at the I/O boundary, not by this entity.

---

## 7. DevBookError

**What:** The single error type for all fallible operations in the CLI.

**Supports:** Clear reporting and exit codes (design pattern: Result-based errors)

**Shape (variants):**
- **ConfigNotFound** — discovery walked to root and found no `dev.yaml` (and no `runbook.yaml`).
- **ParseError** — config file exists but YAML is invalid or shape is wrong (e.g. value not a string). Can carry path and message.
- **UnknownAction** — user ran `dev <action>` but that key is not in the config. Carries the action name.
- **CommandFailed** — the subprocess failed (e.g. non-zero exit code). Can carry action name and exit code or message.
- Optional: **IoError** or **InitWriteError** for `dev init` (e.g. cannot write file).

**Operations:** Map to user-facing message and process exit code at the CLI boundary.

---

## 8. Summary table

| Entity            | Purpose                                      | Supports   |
|-------------------|----------------------------------------------|------------|
| **DevConfig**     | Parsed action → command map                  | R1         |
| **ConfigLocation**| Project root + config path (or not found)    | R2         |
| **ActionName**    | Name of an action (key)                      | R1, R3     |
| **CommandString** | Shell command for an action                  | R1, R4     |
| **RunContext**    | (project_root, command) for execution        | R4         |
| **DefaultActions**| Default keys (and optional placeholders)     | R1.3, R3.4 |
| **DevBookError**  | All error cases and reporting                | All        |

---

## 9. Traits (interfaces, not entities)

These are dependencies of the facade, not domain data:

- **ConfigFinder** — returns `Result<ConfigLocation, DevBookError>` (or equivalent).
- **ConfigLoader** — takes config path, reads file, parses (e.g. YAML) into **DevConfig**; returns in-memory DevConfig only. No raw file format stored in the returned entity.
- **CommandRunner** — takes `RunContext` (or cwd + command), returns `Result<(), DevBookError>`.

The **App** (facade) holds these and uses the entities above to implement `list_actions()`, `init_config()`, and `run_action(name)`.
