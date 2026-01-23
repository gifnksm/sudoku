# sudoku-game: Game Logic Design

## 1. Overview

**Purpose**: Manage game sessions and player interactions for Sudoku gameplay.

**Responsibilities**:

- Manage game state (initial puzzle, current board, solution)
- Accept and validate player operations (set digit, clear cell)
- Distinguish between initial (fixed) cells and player-input cells
- Provide game completion detection
- Provide state access for UI rendering

**Non-responsibilities** (handled by other crates):

- UI rendering and user input handling (`sudoku-app`)
- Puzzle generation (`sudoku-generator`)
- Solving algorithms (`sudoku-solver`)

---

## 2. Requirements

### Minimum Viable Implementation

**Must Have**:

- Start a new game from a generated puzzle
- Place digits in editable cells (player input)
- Clear digits from editable cells
- Prevent editing of initial (fixed) cells
- Query current board state
- Distinguish initial cells from player-input cells
- Detect game completion (all cells filled AND matches solution)
- Allow rule-violating inputs (permissive approach)

**Explicitly Out of Scope for Minimum Viable Implementation**:

- Candidate marks (pencil marks)
- Undo/Redo functionality
- Hint system
- Mistake detection/highlighting
- Save/Load functionality
- Timer or scoring

---

## 3. Design Decisions

### 3.1 Rule Constraint Handling

**Decision**: Permissive input validation

**Rationale**:

- Allow rule-violating inputs (e.g., duplicate digit in same row)
- Players can freely experiment
- Mistakes are discovered organically or via future mistake detection feature
- Matches common Sudoku app behavior

**Implementation**: Only prevent editing of initial cells; all other inputs are accepted

### 3.2 Game Completion Definition

**Decision**: Completion = All cells filled AND no rule violations (consistent)

**Rationale**:

- Accepts any valid solution (handles puzzles with multiple solutions)
- Clear condition: board is completely filled without conflicts
- Aligns with player expectations (any valid completion counts)

### 3.3 Thread Safety

**Decision**: No thread-safety guarantees in initial implementation

**Rationale**:

- Single-player game with single-threaded UI (typical use case)
- If multi-threading needed later, can wrap `Game` in `Mutex` or use message passing
- Keeps implementation simple

### 3.4 Data Representation

**Decision**: To be decided during implementation

**Options**:

- Option A: Store initial `DigitGrid` + track player-modified positions
- Option B: Store initial puzzle positions only + current board state

**Note**: Choice affects how we track initial vs player cells and check completion

---

## 4. Data Structures

### 4.1 Game State

```rust
pub struct Game {
    // Initial puzzle (fixed cells)
    // Current board state
    // Solution (for completion check)
    // Player-modified cell tracking
}
```

### 4.2 Error Type

```rust
pub enum GameError {
    InitialCellNotEditable,
    // Other errors as needed
}
```

---

## 5. Public API

### 5.1 Core Operations

```rust
impl Game {
    /// Create a new game from a generated puzzle
    pub fn new(puzzle: GeneratedPuzzle) -> Self;
    
    /// Place a digit at the given position
    /// Returns error if position is an initial cell
    pub fn set_digit(&mut self, pos: Position, digit: Digit) -> Result<(), GameError>;
    
    /// Clear the digit at the given position
    /// Returns error if position is an initial cell
    pub fn clear_cell(&mut self, pos: Position) -> Result<(), GameError>;
}
```

### 5.2 State Query

```rust
impl Game {
    /// Get the current board state
    pub fn current_grid(&self) -> &DigitGrid;
    
    /// Check if a position is an initial (non-editable) cell
    pub fn is_initial_cell(&self, pos: Position) -> bool;
    
    /// Check if the game is completed (all filled + no rule violations)
    pub fn is_completed(&self) -> bool;
}
```

---

## 6. Dependencies

- `sudoku-core`: Core data structures (`Digit`, `Position`, `DigitGrid`)
- `sudoku-solver`: (Optional for minimum viable implementation, needed for future hint system)
- `sudoku-generator`: `GeneratedPuzzle` type

---

## 7. Testing Strategy

- **Unit tests**: Test each operation (set_digit, clear_cell, etc.)
- **Doctests**: Demonstrate API usage in documentation
- **Edge cases**: Initial cell editing, completion detection, empty puzzle
- **Property-based tests**: Consider for future enhancements (undo/redo invariants)

---

## 8. Feature Extension

**Not included in minimum viable implementation, to be designed and implemented later**:

- **Candidate marks**: Toggle pencil marks for cells
- **Undo/Redo**: Operation history with rollback
- **Hint system**: Use `TechniqueSolver` to suggest next move
- **Mistake detection**: Highlight cells that violate rules or contradict solution
- **Save/Load**: Serialize/deserialize game state (JSON or binary)
- **Timer**: Track elapsed play time
- **Statistics**: Move count, hint usage, etc.

---

## 9. Implementation Plan

**Step 1**: Finalize data structure design (how to track initial vs player cells)

**Step 2**: Implement core `Game` struct and basic operations

**Step 3**: Implement state query methods

**Step 4**: Write comprehensive tests

**Step 5**: Update documentation and mark TODO as complete

**After Minimum Viable Implementation**: Move to `sudoku-app` implementation for basic GUI
