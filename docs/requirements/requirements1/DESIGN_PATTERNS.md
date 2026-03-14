# Design patterns for Requirements 1

Design patterns we use to implement the DevBook CLI prototype (see [SPEC.md](./SPEC.md)). Full discussion: [DESIGN_PATTERNS](../../DESIGN_PATTERNS.md) in docs.

---

## Patterns we will use

| Pattern | Role for requirement1 |
|--------|------------------------|
| **Command (dispatch)** | Map `dev`, `dev init`, `dev <action>` to separate handlers. Implements R3.x (CLI commands). |
| **Single Responsibility** | One concern per module: config discovery (R2), config parse (R1), run (R4), CLI. |
| **Dependency Inversion** | Depend on traits (`ConfigFinder`, `ConfigLoader`, `CommandRunner`); inject concrete impls in `main`. Keeps code testable and swappable. |
| **Facade** | One entry (e.g. `App`) that exposes `list_actions()`, `init_config()`, `run_action(name)`. CLI only talks to the facade. |
| **Result / explicit errors** | All fallible steps return `Result`. One error type (e.g. `ConfigNotFound`, `ParseError`, `UnknownAction`, `CommandFailed`). CLI maps to messages and exit codes. |
| **Data-centric config** | Parsed config = data only (e.g. `HashMap<String, String>`). Implements R1.x (config file) without logic in the config struct. |

---

## Mapping to spec

- **R1 (Config file)** → Data-centric config + config parse module.
- **R2 (Config discovery)** → Config discover module; returns project root + config path.
- **R3 (CLI commands)** → Command pattern (dispatch) + facade methods.
- **R4 (Execution)** → Command runner (trait + impl); run in project root, stream output.

Errors from discovery, parse, unknown action, and command failure flow through the single error type and are handled at the CLI boundary.
