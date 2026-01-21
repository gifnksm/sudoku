# Sudoku

A Sudoku application written in Rust, supporting both desktop and web platforms.

## Project Goals

- **Automatic Puzzle Generation**: Generate Sudoku puzzles with configurable difficulty levels
- **Multiple Solving Strategies**: Implement both algorithmic (backtracking) and human-like solving techniques
- **Cross-Platform**: Desktop GUI and Web/WASM support using egui/eframe

## Current Status

- ✅ **sudoku-core**: Core data structures **completed**
  - `Digit`: Type-safe representation of numbers 1-9
  - `Position`: Board coordinates with box calculation utilities
  - `CandidateGrid`: Candidate tracking grid for solving algorithms (digit-centric)
  - `DigitGrid`: Simple cell-centric grid with string parsing/formatting
  - Generic containers (`BitSet9`, `BitSet81`, `Array9`, `Array81`)
  - Type-safe indexing with semantic index types
- ✅ **sudoku-solver**: Solving algorithms **completed**
  - `TechniqueSolver`: Human-like solving techniques
  - `BacktrackSolver`: Technique-based solving with backtracking fallback
  - Extensible technique system
  - Solution enumeration for puzzle validation
- 📋 **Next**: Puzzle generation, GUI

## Project Structure

```text
crates/
├── sudoku-core/       # Core data structures (CandidateGrid, DigitGrid, Digit, Position)
├── sudoku-solver/     # Solving algorithms (technique-based + backtracking)
├── sudoku-generator/  # Puzzle generation (planned)
├── sudoku-game/       # Game logic (planned)
└── sudoku-app/        # GUI application (planned)
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for architecture and implementation plans.

## Build and Test

```bash
# Build all crates
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets

# Generate documentation (project crates only, faster)
cargo doc --no-deps

# Generate and open documentation
cargo doc --no-deps --open
```

## Run

```bash
# Desktop application (not yet implemented)
cargo run --release
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
