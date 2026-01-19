//! Core data structures for sudoku solvers.
//!
//! This crate provides efficient, specialized data structures for working with
//! sudoku puzzles, particularly bitsets and candidate tracking.
//!
//! # Modules
//!
//! - [`bit_set_9`]: Generic 9-bit set implementation
//! - [`bit_set_81`]: Generic 81-bit set implementation for 9x9 board positions
//! - [`digit_candidates`]: Candidate digits for a single cell
//! - [`position`]: Board position types and utilities
//! - [`candidate_board`]: Candidate bitboard for solving

pub mod bit_set_81;
pub mod bit_set_9;
pub mod candidate_board;
pub mod digit_candidates;
pub mod position;

// Re-export commonly used types
pub use self::{
    candidate_board::{CandidateBoard, DigitPositions, HouseMask},
    digit_candidates::DigitCandidates,
    position::Position,
};
