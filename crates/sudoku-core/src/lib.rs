//! Core data structures for sudoku solvers.
//!
//! This crate provides efficient, specialized data structures for working with
//! sudoku puzzles, particularly sets of numbers 1-9.
//!
//! # Modules
//!
//! - [`bit_set_9`]: Generic 9-bit set implementation
//! - [`bit_set_81`]: Generic 81-bit set implementation for 9x9 board positions
//! - [`number_set`]: Specialized set for numbers 1-9

pub mod bit_set_81;
pub mod bit_set_9;
pub mod number_set;
