# sudoku-generator Design Document

## Overview

This document outlines the design for the `sudoku-generator` crate, which generates valid Sudoku puzzles with unique solutions.

## Goals

- Generate valid Sudoku puzzles programmatically
- Ensure generated puzzles have exactly one solution
- Support reproducible generation via seed
- Keep implementation simple and extensible

## Algorithm: Removal Method

We adopt the **removal method** (also known as subtraction method):

1. **Generate a complete solution grid** - Create a fully filled valid Sudoku grid
2. **Remove cells** - Remove digits one by one while maintaining unique solution
3. **Stop when no more cells can be removed** - Maximum difficulty (minimum hints)

### Why Removal Method?

- Straightforward to implement
- Leverages existing `sudoku-solver` infrastructure
- Naturally produces valid puzzles with unique solutions
- Easy to understand and test

### Key Design Dependency

**This implementation critically depends on a property of `TechniqueSolver`:**

`TechniqueSolver` uses only logical deduction to determine cells. If a puzzle has multiple solutions, there will be ambiguous cells that cannot be determined by logic alone, causing the solver to get stuck and fail. Conversely, if `TechniqueSolver` successfully solves a puzzle, it proves the puzzle has a unique solution.

This property allows us to:

1. Verify unique solutions without explicit multiple-solution checking
2. Naturally limit puzzle difficulty to human-solvable levels
3. Avoid generating puzzles that require guessing or backtracking

## Generation Process

### Step 1: Generate Complete Grid

Generate a fully filled valid Sudoku grid (all 81 cells filled).

Implementation approach to be determined during development (e.g., random placement with backtracking, or seed with solver).

### Step 2: Remove Cells

1. Create a list of all 81 cell positions
2. Shuffle the list using seeded RNG
3. Iterate through shuffled positions:
   - Try removing the digit at that position
   - Verify the puzzle is still solvable using human-like techniques
   - If solvable: keep it removed
   - If not solvable: restore the digit and skip
4. Stop when all positions have been tried

### Step 3: Solvability Verification

Use `TechniqueSolver` to check solvability:

- Attempt to solve the puzzle using only human-like techniques
- If solvable: puzzle has reasonable difficulty AND unique solution (accept removal)
- If not solvable: puzzle is either too difficult OR has multiple solutions (reject removal, restore digit)

This approach achieves two goals simultaneously:

1. **Unique solution guarantee**: Leverages `TechniqueSolver`'s property (see above)
2. **Difficulty control**: Limits puzzles to human-solvable difficulty

## API Design

### Public API

```rust
pub struct PuzzleGenerator<'a> {
    solver: &'a TechniqueSolver,
}

impl<'a> PuzzleGenerator<'a> {
    /// Create a new generator with a solver
    pub fn new(solver: &'a TechniqueSolver) -> Self;
    
    /// Generate a puzzle with a random seed
    pub fn generate(&self) -> GeneratedPuzzle;
    
    /// Generate a puzzle with a specific seed for reproducibility
    pub fn generate_with_seed(&self, seed: [u8; 32]) -> GeneratedPuzzle;
}

pub struct GeneratedPuzzle {
    pub problem: DigitGrid,
    pub solution: DigitGrid,
    pub seed: [u8; 32],
}
```

### Design Decisions

- **Dependency Injection**: `TechniqueSolver` is passed to constructor, making dependencies explicit
- **Stateless Generation**: RNG is created per `generate()` call using seed
- **Type Safety**: Use struct instead of tuple `(DigitGrid, DigitGrid)` to avoid confusion
- **Return Problem, Solution, and Seed**: Seed enables puzzle reproduction
- **Seed Type**: `[u8; 32]` provides full 256-bit entropy for `Pcg64`
- **Error Handling**: Start without `Result`; add if needed during implementation

## Dependencies

- **`sudoku-core`**: `DigitGrid`, `CandidateGrid`
- **`sudoku-solver`**: `TechniqueSolver` for solvability verification
- **`rand`**: Random number generation (latest version)
- **`rand_pcg`**: `Pcg64` for reproducible, fast, non-cryptographic RNG

## Module Structure

To be determined during implementation based on actual code organization needs.

## Future Extensions

These are explicitly **not** part of the initial implementation:

- Difficulty-based generation (easy, medium, hard)
- Symmetry constraints (rotational, mirror)
- Configurable hint count (min/max)
- Generation from partial grids
- Statistical analysis of generated puzzles

## Testing Strategy

Use `proptest` for property-based testing:

- **Property**: Generated puzzles are solvable with TechniqueSolver
- **Property**: Generated puzzles are always solvable
- **Property**: Same seed produces same puzzle
- **Property**: Generated problem is a subset of solution

## Non-Goals

- Difficulty evaluation (deferred until more solving techniques are implemented)
- Optimization for generation speed (focus on correctness first)
- Advanced generation strategies (symmetry, patterns, etc.)

---

**Status**: Design Complete, Ready for Implementation
**Next Step**: Create crate and implement basic generation logic
