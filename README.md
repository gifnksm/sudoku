# Numelace

Numelace is a number-place (Sudoku) puzzle application written in Rust, with a desktop-first focus and planned Web/WASM support.

## Project Goals

- **Automatic Puzzle Generation**: Generate Sudoku puzzles with configurable difficulty levels
- **Multiple Solving Strategies**: Implement both algorithmic (backtracking) and human-like solving techniques
- **Cross-Platform**: Desktop GUI today, Web/WASM planned, using egui/eframe

## Current Status

Planned features are tracked in docs/BACKLOG.md.

- ✅ Desktop GUI (core play): 9x9 board, keypad with digit counts, highlights, theme toggle, new game confirmation
- ✅ Auto-save and resume on startup
- ✅ Puzzle generation with unique solution guarantee and reproducible seeds
- ✅ Solver with basic techniques (Naked/Hidden Single) plus backtracking
- ✅ Core gameplay rules: given vs filled cells and solved-state validation
- ✅ Candidate notes (player notes)
- ⚙️ Web/WASM support is planned

## Project Structure

```text
crates/  # workspace crates
docs/    # project documentation
```

## Documentation

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - architecture and design decisions
- [docs/WORKFLOW.md](docs/WORKFLOW.md) - development workflow
- [docs/BACKLOG.md](docs/BACKLOG.md) - ideas and future work
- [docs/DESIGN_LOG.md](docs/DESIGN_LOG.md) - decision history
- [docs/TESTING.md](docs/TESTING.md) - testing guidelines
- [docs/DOCUMENTATION_GUIDE.md](docs/DOCUMENTATION_GUIDE.md) - documentation conventions

For contributions, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Build and Run

```bash
# Build all crates
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test
```

For development commands (clippy, benchmarks, documentation generation, etc.), see [CONTRIBUTING.md](CONTRIBUTING.md).

```bash
# Desktop application
cargo run --release
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
