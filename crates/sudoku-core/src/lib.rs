//! Core data structures for sudoku applications.
//!
//! This crate provides fundamental, efficient data structures for representing and
//! manipulating sudoku puzzles. These structures are used across solving, generation,
//! and game management components.
//!
//! # Core Types
//!
//! ## Basic Sudoku Types
//!
//! - [`Digit`] - Type-safe representation of sudoku digits 1-9
//! - [`Position`] - Board position with (x, y) coordinates in the range 0-8
//!
//! ## Candidate Tracking
//!
//! - [`DigitCandidates`] - Set of candidate digits (1-9) for a single cell.
//!   This is a specialized [`BitSet9`] using [`DigitSemantics`] to map digits to bit indices.
//!
//! - [`CandidateBoard`] - Board-wide candidate tracking for sudoku solving.
//!   Tracks possible placements for each digit across all 81 positions.
//!
//! - [`DigitPositions`] - Set of board positions where a specific digit can be placed.
//!   A type alias for `BitSet81<PositionSemantics>`.
//!
//! - [`HouseMask`] - Bitmask for candidate positions within a house (row/col/box).
//!   A type alias for `BitSet9<CellIndexSemantics>`.
//!
//! # Generic Infrastructure
//!
//! ## Index Types and Semantics
//!
//! The [`index`] module provides index types and semantics for type-safe container access:
//!
//! - [`Index9`] and [`Index81`] - Index types for 9-element and 81-element containers
//! - [`Index9Semantics`] and [`Index81Semantics`] - Traits defining index semantics
//! - [`DigitSemantics`], [`CellIndexSemantics`], [`PositionSemantics`] - Concrete semantics implementations
//!
//! ## Generic Containers
//!
//! The [`containers`] module provides generic containers parameterized by semantics:
//!
//! - [`BitSet9`] - Efficient 9-element bitset
//! - [`BitSet81`] - Efficient 81-element bitset
//! - [`Array9`] - 9-element array with semantic indexing
//!
//! [`Index9`]: index::Index9
//! [`Index81`]: index::Index81
//! [`Index9Semantics`]: index::Index9Semantics
//! [`Index81Semantics`]: index::Index81Semantics
//! [`DigitSemantics`]: index::DigitSemantics
//! [`CellIndexSemantics`]: index::CellIndexSemantics
//! [`PositionSemantics`]: index::PositionSemantics
//! [`BitSet9`]: containers::BitSet9
//! [`BitSet81`]: containers::BitSet81
//! [`Array9`]: containers::Array9
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```
//! use sudoku_core::{Digit, DigitCandidates, Position};
//!
//! // Create a position and digit
//! let pos = Position::new(4, 4);
//! let digit = Digit::D5;
//!
//! // Work with candidate sets
//! let mut candidates = DigitCandidates::FULL;
//! candidates.remove(Digit::D1);
//! candidates.remove(Digit::D9);
//! assert_eq!(candidates.len(), 7);
//! ```
//!
//! ## Candidate Board
//!
//! ```
//! use sudoku_core::{CandidateBoard, Digit, DigitCandidates, Position};
//!
//! // Create a candidate board
//! let mut board = CandidateBoard::new();
//!
//! // Place a digit
//! board.place(Position::new(4, 4), Digit::D5);
//!
//! // Check remaining candidates at a position
//! let candidates: DigitCandidates = board.get_candidates_at(Position::new(4, 5));
//! assert!(!candidates.contains(Digit::D5)); // 5 removed from same column
//! ```

mod candidate_board;
pub mod containers;
mod digit;
pub mod index;
mod position;

// Re-export commonly used types
pub use self::{candidate_board::*, digit::*, position::*};
