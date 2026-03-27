# rustup-paths-demo

A prototype CLI tool exploring how rustup can be refactored to support XDG-style directory layouts while preserving backwards compatibility.

---

## Motivation

Rustup currently uses coarse-grained directory configuration:

- `RUSTUP_HOME` (default: `~/.rustup`)
- `CARGO_HOME` (default: `~/.cargo`)

This mixes configuration, data, and cache jumbled into a single location and tightly couples rustup with Cargo.

---

## Goal of this prototype

This project does **not change rustup behavior**, but instead explores:

- how directory responsibilities can be split (config/data/cache/bin)
- how a path resolution layer could support both legacy and XDG layouts
- how migration could be planned safely

---

## Path Resolution Model

### Precedence (compatibility-first)

1. **Environment overrides (highest priority)**
  - `RUSTUP_HOME`
  - `CARGO_HOME`

2. **Legacy defaults (current rustup behavior)**
  - `~/.rustup`
  - `~/.cargo/bin`

3. **XDG layout (opt-in via `--use-xdg`)**
  - `~/.config/rustup`
  - `~/.local/share/rustup`
  - `~/.cache/rustup`

XDG is **not enabled by default** to avoid breaking existing setups.

---

## Directory Mapping

| Category | Legacy | XDG (opt-in) |
|--------|--------|-------------|
| Config | `~/.rustup` | `~/.config/rustup` |
| Data   | `~/.rustup` | `~/.local/share/rustup` |
| Cache  | `~/.rustup/tmp` | `~/.cache/rustup` |
| Bin (shims) | `~/.cargo/bin` | (unchanged, conservative) |

---

## Features/commands

## Usage Examples

### 1. Resolve paths (default behavior)

cargo run -- resolve


Example output:

config_dir: /home/user/.rustup

data_dir:   /home/user/.rustup

cache_dir:  /home/user/.rustup/tmp

bin_dir:    /home/user/.cargo/bin

---

### 2. Explain resolution decisions

cargo run -- explain

Example:

config_dir: /home/user/.rustup

reason: preserving legacy default rustup config layout

---

### 3. Enable XDG mode by flag
cargo run -- --use-xdg explain

example output:
Warnings:
- RUSTUP_HOME is set, so rustup-owned directories stay in legacy layout even when XDG mode is enabled
- CARGO_HOME is set, so bin_dir follows legacy cargo behavior
- XDG mode currently does not move bin_dir; keeping ~/.cargo/bin-style behavior is the conservative default

config_dir: /home/taufiqrahman/.rustup
reason: explicit RUSTUP_HOME override preserves legacy rustup layout

data_dir: /home/taufiqrahman/.rustup
reason: explicit RUSTUP_HOME override preserves legacy rustup layout

cache_dir: /home/taufiqrahman/.rustup/tmp
reason: explicit RUSTUP_HOME override preserves legacy rustup cache/tmp layout

bin_dir: /home/taufiqrahman/.cargo/bin
reason: explicit CARGO_HOME override preserves cargo-compatible bin directory

cargo run -- --use-xdg resolve

---

### 4. Migration plan (dry run)

cargo run -- --use-xdg migrate-plan

---

### 5. Execute migration

cargo run -- --use-xdg migrate --execute

---

### 6. JSON output

cargo run -- --use-xdg --json explain
