# DevBook – Design Patterns

Design patterns and structure to keep the CLI clear, testable, and easy to extend. Aligned with [CORE_IDEA.md](./CORE_IDEA.md) and [requirements](./requirements/requirements1.md).

---

## 1. Problems we need to solve

| Problem | Description |
|--------|-------------|
| **Config discovery** | Find `dev.yaml` by walking up from cwd; know the project root. |
| **Config parsing** | Load flat YAML (action → command); validate shape. |
| **CLI dispatch** | Map `dev`, `dev init`, `dev <action>` to the right behavior. |
| **Execution** | Run a shell command in project root; stream output. |
| **Errors** | Config missing, parse error, unknown action, command failure — report clearly. |

---

## 2. Patterns that fit

### 2.1 Command pattern (CLI subcommands)

**Idea:** Treat each user-facing operation as a distinct command. The CLI layer only parses args and delegates to a command handler.

**Use for:** `dev` (list), `dev init`, `dev <action>` (run).

**In code:**

- One handler per “command”: e.g. `list_actions`, `init_config`, `run_action(action_name)`.
- clap defines the subcommands; your `main` or a dispatcher calls the right handler with a small context (e.g. project root, config if present).

**Benefit:** Adding a new subcommand (e.g. `dev validate`) is a new handler; CLI and config/execution stay decoupled.

---

### 2.2 Single responsibility + thin facade

**Idea:** Split “find config”, “parse config”, and “run command” into separate modules. A thin top-level layer (or facade) orchestrates them.

**Use for:** Config discovery, parsing, and execution.

**Suggested modules:**

| Module | Responsibility |
|--------|----------------|
| **config::discover** | Walk up from cwd; return path to config file and project root (or “not found”). |
| **config::parse** | Read file, parse YAML into a map `action_name → command_string`; return struct or error. |
| **run** | Given project root + command string, spawn process in that dir and stream stdout/stderr. |
| **cli** | Parse argv with clap; call discover → parse (if needed) → list / init / run. |

**Benefit:** You can test “find config”, “parse this YAML”, and “run this command” in isolation; no need to run the full CLI.

---

### 2.3 Strategy (optional, for later)

**Idea:** Swap how we discover config or how we run commands without changing the rest of the app.

**Use for (later):**  
- Config discovery: “walk up from cwd” vs “use explicit path” vs “use env var”.  
- Execution: “real shell” vs “dry-run (print only)” vs “capture output for tests”.

**In code:** Traits (e.g. `ConfigFinder`, `CommandRunner`) with one default impl; inject in tests or via flags. For the prototype, a single implementation is enough; introduce traits only when you need a second strategy.

---

### 2.4 Result-based error handling

**Idea:** Use `Result<T, E>` (and optionally `Option`) for every fallible step. Define a small error type (or enum) for “config not found”, “parse error”, “unknown action”, “command failed”.

**Use for:** Discovery, parse, run; propagate errors to `main` and print a clear message (and exit code).

**Benefit:** No panics for expected failures; easy to test “when config is missing we get this error”; optional `anyhow`/`thiserror` for nicer messages and context.

---

### 2.5 Data-centric config

**Idea:** Config is a simple data structure (e.g. `HashMap<String, String>` or a struct with that shape). No behavior in the config type; only “list keys” and “get command for key”.

**Use for:** Parsed `dev.yaml` — just “action name → command string”.

**Benefit:** Serialization/deserialization (serde) is straightforward; easy to add defaults or validation in one place (e.g. “prefer `run` if `local` is missing” later).

---

## 3. Suggested flow (high level)

```
main()
  → clap parses argv
  → match subcommand:
      "dev" (list)     → discover → parse (if found) → list keys
      "init"           → discover; if not found → init_config (write dev.yaml)
      "<action>"       → discover → parse → get command for action → run in project root
  → map errors to messages and exit code
```

- **Discovery** and **parse** are reused by list and run; **init** only needs discovery (to know where to write or that we’re already inside a project).
- **Run** module only needs project root + command string; it doesn’t care about YAML or CLI.

---

## 4. Summary

| Pattern | Use in DevBook |
|---------|-----------------|
| **Command** | Each CLI subcommand = one handler (list, init, run). |
| **Single responsibility + thin facade** | Separate modules: discover, parse, run; CLI ties them together. |
| **Strategy** | Optional later: pluggable discover/run for tests and dry-run. |
| **Result-based errors** | Discovery/parse/run return `Result`; one error type; clear messages in `main`. |
| **Data-centric config** | Parsed config = map of action → command; no logic in the config struct. |

This keeps the prototype simple and makes it easy to add “optional later” features (e.g. `dev init` with AI, MCP, or inference from `package.json`) by adding new commands or new strategies behind the same interfaces.
