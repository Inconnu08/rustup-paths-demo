# rustup-paths-demo

A prototype CLI tool exploring XDG path support and migration strategy for rustup.

## Motivation

Rustup currently uses coarse-grained directory configuration via:

- `RUSTUP_HOME` (default: `~/.rustup`)
- `CARGO_HOME` (default: `~/.cargo`)

There is ongoing interest in migrating toward a more structured, XDG-compliant layout:

- `$XDG_CONFIG_HOME`
- `$XDG_DATA_HOME`
- `$XDG_CACHE_HOME`

This project explores how rustup could:
- support fine-grained directory separation
- maintain backward compatibility
- provide safe migration strategies

---

## Features

### Path Resolution
- Supports both legacy (`RUSTUP_HOME`, `CARGO_HOME`) and XDG layouts

### Explain Mode
- Shows exactly *why* each path was chosen
- Highlights compatibility constraints

### Migration Planning
- Shows what files/directories would move
- Detects missing sources and existing destinations
- Provides warnings for unsafe scenarios

### Migration Execution
- Supports:
    - `--dry-run` (default)
    - `--execute` (actual filesystem changes)

---