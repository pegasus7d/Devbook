# DevBook

CLI that runs project commands from a single `dev.yaml` config. See **[docs/](./docs/)** for the idea, spec, and tech stack.

## Testing

```bash
cargo test
```

- **Unit tests**: `DevConfig` (entities), error `Display` (error), `load_config` (config/parse).
- **Integration tests**: run the binary in a temp dir — list without config (fails), `init` (creates dev.yaml), list with config (shows actions).
