# Dev Runbook – Core Idea & Prototype (Full Spec)

## 1. What it is

- A **CLI** (`dev`) you install once.
- Each project has **one config file** that you edit and write by hand.
- The CLI runs the commands defined there so everyone uses the same interface (`dev run`, `dev test`, etc.).

---

## 2. Config file

- **File:** `dev.yaml` (or `runbook.yaml`) at project root.
- **Format:** Flat YAML: one line per action, **key** = action name, **value** = shell command. User can edit and add more lines as needed.

**Example:**

```yaml
install: npm install
run:     npm run dev
test:    npm test
build:   npm run build
local:   npm run dev
stage:   ./scripts/deploy-staging.sh
prod:    ./scripts/deploy-prod.sh
lint:    npm run lint
db-migrate: npx prisma migrate dev
```

- **Defaults we support:** `install`, `run`, `test`, `build`, `local`, `stage`, `prod`.
- **User can add more:** any extra keys (e.g. `lint`, `db-migrate`) are just more commands; no fixed list.

---

## 3. CLI behavior

| Command       | Behavior |
|---------------|----------|
| `dev`         | Find `dev.yaml` in cwd (or parent); list all actions (defaults + custom). |
| `dev run`     | Run the command under `run:` in `dev.yaml`. Same for `dev test`, `dev build`, `dev local`, `dev stage`, `dev prod`, or any key user added (e.g. `dev lint`). |
| `dev install` | Run the command under `install:`. |
| `dev init`    | If no `dev.yaml`: prompt for the default actions (install, run, test, build, local, stage, prod) and write `dev.yaml`. Optional: accept one-line natural language and use AI to generate the YAML. |

**Rule:** For any `dev <action>`, if that key exists in `dev.yaml`, run its value as the shell command in the project directory.

---

## 4. User flow

**Existing project with `dev.yaml`:**
- `dev` → see actions.
- `dev install` → install deps.
- `dev run` → start app (or `dev local`).
- `dev test` → run tests.
- `dev stage` / `dev prod` → run staging/prod commands.
- `dev lint`, `dev db-migrate`, etc. → run any user-defined command.

**New project (no config):**
- `dev init` → wizard (or AI) creates `dev.yaml` with the seven defaults (user can leave some blank).
- User can then open `dev.yaml` and add more commands by adding more lines.

**Editing:** User edits `dev.yaml` directly (add/remove/rename actions, change commands). No special tool; just the config format we give them.

---

## 5. Scope of prototype

**In scope:**
- One config file per repo: `dev.yaml`.
- Format: flat YAML, user-editable and writable; defaults: install, run, test, build, local, stage, prod; any extra keys = extra commands.
- CLI: `dev`, `dev init`, `dev <action>` for any key in the file.
- Commands run in project directory; output in terminal.

**Optional later:**
- `dev init` with natural language → AI generates YAML.
- MCP server so assistants can list/run actions and read config.
- Inference from `package.json` / `Cargo.toml` when a key is missing.

---

## 6. One-sentence summary

**Prototype:** A CLI that reads a single, hand-editable YAML file (`dev.yaml`) where each key is an action name and each value is a shell command; default actions are install, run, test, build, local, stage, prod; user can add more commands by adding more keys; `dev <action>` runs the corresponding command.
