# Week Parity Checker

Rust-powered WASM app to calculate odd/even week parity based on a custom semester start date and timezone.

## Features
- Full Client-Side Rendering (CSR).
- Timezone-aware "Today" calculation.
- Noto Serif design.
- Configurable via `config.toml`.

## Structure
- `pkg/`: WASM build artifacts.
- `static/`: CSS and JS.
- `src/`: Rust source code.
- `index.html`: Main entry point.

## Build
```bash
cd week-parity
cargo build --target wasm32-unknown-unknown
# Or use wasm-pack
wasm-pack build --target web
```
