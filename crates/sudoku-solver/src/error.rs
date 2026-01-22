use sudoku_core::ConsistencyError;

/// Errors that can occur during solving.
#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum SolverError {
    /// A contradiction was detected in the puzzle state.
    ///
    /// This indicates that the puzzle is invalid or unsolvable.
    /// Contradictions occur when:
    /// - A cell has no remaining candidates
    /// - Constraint propagation leads to an impossible state
    #[display("Contradiction detected")]
    Contradiction,
}

impl From<ConsistencyError> for SolverError {
    fn from(ConsistencyError: ConsistencyError) -> Self {
        Self::Contradiction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistency_error_converts_to_solver_error() {
        let consistency_error = ConsistencyError;
        let solver_error: SolverError = consistency_error.into();
        assert!(matches!(solver_error, SolverError::Contradiction));
    }

    #[test]
    fn test_consistency_error_conversion_with_question_mark() {
        fn check() -> Result<(), SolverError> {
            Err(ConsistencyError)?
        }

        let result = check();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SolverError::Contradiction));
    }
}
