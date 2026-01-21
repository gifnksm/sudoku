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
