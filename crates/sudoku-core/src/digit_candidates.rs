//! Candidate digits (1-9) for a single cell.
//!
//! This module provides [`DigitCandidates`], a specialized implementation of
//! [`BitSet9`] for representing sets of digits 1-9.
//!
//! # Examples
//!
//! ```
//! use sudoku_core::DigitCandidates;
//!
//! let mut set = DigitCandidates::new();
//! set.insert(1);
//! set.insert(5);
//! set.insert(9);
//!
//! assert_eq!(set.len(), 3);
//! assert!(set.contains(5));
//! ```

use crate::bit_set_9::{BitIndex9, BitSet9, BitSet9Semantics};

/// Semantics for digits 1-9.
///
/// This type implements [`BitSet9Semantics`]
/// to map user-facing values (1-9) to internal bit indices (0-8).
///
/// **Note**: This is an implementation detail of [`DigitCandidates`].
/// You should use [`DigitCandidates`] directly instead of constructing
/// `BitSet9<DigitSemantics>` manually.
///
/// # Panics
///
/// The `to_index` method panics if a value outside the range 1-9 is provided.
#[derive(Debug)]
pub struct DigitSemantics;

impl BitSet9Semantics for DigitSemantics {
    type Value = u8;
    fn to_index(value: Self::Value) -> BitIndex9 {
        assert!(
            (1..=9).contains(&value),
            "Number must be between 1 and 9, got {value}"
        );
        BitIndex9::new(value - 1)
    }
    fn from_index(index: BitIndex9) -> Self::Value {
        index.index() + 1
    }
}

/// A set of candidate digits (1-9) for a single cell.
///
/// This is a specialized version of [`BitSet9`] that represents
/// digits in the range 1-9, commonly used to track which digits
/// can be placed in a sudoku cell.
///
/// The implementation uses a 16-bit integer where bits 0-8 represent digits 1-9 respectively,
/// providing efficient storage and fast set operations.
///
/// # Examples
///
/// ```
/// use sudoku_core::DigitCandidates;
///
/// // Create a set with all candidates available
/// let mut candidates = DigitCandidates::FULL;
///
/// // Remove some numbers
/// candidates.remove(5);
/// candidates.remove(7);
///
/// assert_eq!(candidates.len(), 7);
/// assert!(!candidates.contains(5));
/// assert!(candidates.contains(1));
/// ```
///
/// # Set Operations
///
/// ```
/// use sudoku_core::DigitCandidates;
///
/// let a = DigitCandidates::from_iter([1, 2, 3]);
/// let b = DigitCandidates::from_iter([2, 3, 4]);
///
/// // Union
/// let union = a | b;
/// assert_eq!(union, DigitCandidates::from_iter([1, 2, 3, 4]));
///
/// // Intersection
/// let intersection = a & b;
/// assert_eq!(intersection, DigitCandidates::from_iter([2, 3]));
///
/// // Difference
/// let diff = a.difference(b);
/// assert_eq!(diff, DigitCandidates::from_iter([1]));
/// ```
pub type DigitCandidates = BitSet9<DigitSemantics>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_range() {
        let mut set = DigitCandidates::new();
        set.insert(1);
        set.insert(9);
        assert!(set.contains(1));
        assert!(set.contains(9));
        assert_eq!(set.len(), 2);
    }

    #[test]
    #[should_panic(expected = "Number must be")]
    fn test_rejects_zero() {
        let mut set = DigitCandidates::new();
        set.insert(0);
    }

    #[test]
    #[should_panic(expected = "Number must be")]
    fn test_rejects_ten() {
        let mut set = DigitCandidates::new();
        set.insert(10);
    }

    #[test]
    fn test_from_iter() {
        let set = DigitCandidates::from_iter([1, 5, 9]);
        assert_eq!(set.len(), 3);
        assert!(set.contains(1));
        assert!(set.contains(5));
        assert!(set.contains(9));
    }

    #[test]
    fn test_iteration_order() {
        let set = DigitCandidates::from_iter([9, 1, 5, 3]);
        let collected: Vec<_> = set.iter().collect();
        assert_eq!(collected, vec![1, 3, 5, 9]);
    }

    #[test]
    fn test_operations() {
        let a = DigitCandidates::from_iter([1, 2, 3]);
        let b = DigitCandidates::from_iter([2, 3, 4]);

        assert_eq!(a.union(b).len(), 4);
        assert_eq!(a.intersection(b).len(), 2);
        assert_eq!(a.difference(b).len(), 1);
    }

    #[test]
    fn test_constants() {
        assert_eq!(DigitCandidates::EMPTY.len(), 0);
        assert_eq!(DigitCandidates::FULL.len(), 9);

        for n in 1..=9 {
            assert!(DigitCandidates::FULL.contains(n));
        }
    }
}
