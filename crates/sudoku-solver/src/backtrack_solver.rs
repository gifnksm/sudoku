//! Backtracking solver for Sudoku puzzles.
//!
//! This module provides a solver that combines technique-based solving with backtracking.
//! When techniques cannot make further progress, the solver makes assumptions and explores
//! the search space to find solutions.

use sudoku_core::{CandidateGrid, Digit, DigitCandidates, Position};

use crate::{SolverError, TechniqueSolver, TechniqueSolverStats, technique::BoxedTechnique};

/// Statistics collected during backtracking solving.
///
/// Tracks technique applications, assumptions made, and backtrack events.
#[derive(Debug, Default, Clone)]
pub struct BacktrackSolverStats {
    technique: TechniqueSolverStats,
    assumptions: Vec<(Position, Digit)>,
    backtrack_count: usize,
}

impl BacktrackSolverStats {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates statistics with initial technique statistics.
    #[must_use]
    pub fn with_technique(technique: TechniqueSolverStats) -> Self {
        Self {
            technique,
            assumptions: vec![],
            backtrack_count: 0,
        }
    }

    /// Returns the technique solver statistics.
    #[must_use]
    pub fn technique(&self) -> &TechniqueSolverStats {
        &self.technique
    }

    /// Returns the list of assumptions made during solving.
    ///
    /// Each assumption is a `(Position, Digit)` pair representing a cell and
    /// the digit that was assumed for it.
    #[must_use]
    pub fn assumptions(&self) -> &[(Position, Digit)] {
        &self.assumptions
    }

    /// Returns `true` if the puzzle was solved without making any assumptions.
    ///
    /// This indicates the puzzle was solvable using only the configured techniques.
    #[must_use]
    pub fn solved_without_assumptions(&self) -> bool {
        self.assumptions.is_empty()
    }

    /// Returns the number of assumptions that led to contradictions.
    ///
    /// This counts how many times the solver had to backtrack because an
    /// assumption resulted in an inconsistent state during technique application.
    #[must_use]
    pub fn backtrack_count(&self) -> usize {
        self.backtrack_count
    }
}

/// A solver that combines technique-based solving with backtracking.
///
/// `BacktrackSolver` first applies techniques to solve as much as possible,
/// then uses backtracking to explore remaining possibilities when stuck.
/// It can find all solutions to a puzzle, not just the first one.
///
/// # Examples
///
/// ```
/// use sudoku_core::CandidateGrid;
/// use sudoku_solver::BacktrackSolver;
///
/// let solver = BacktrackSolver::with_all_techniques();
/// let grid = CandidateGrid::new();
///
/// // Get first solution
/// if let Some((solution, stats)) = solver.solve(grid)?.next() {
///     println!("Solved with {} assumptions", stats.assumptions().len());
///     if stats.solved_without_assumptions() {
///         println!("No backtracking needed!");
///     }
/// }
/// # Ok::<(), sudoku_solver::SolverError>(())
/// ```
///
/// # Finding Multiple Solutions
///
/// ```
/// use sudoku_core::CandidateGrid;
/// use sudoku_solver::BacktrackSolver;
///
/// let solver = BacktrackSolver::with_all_techniques();
/// let grid = CandidateGrid::new();
///
/// // Check for unique solution
/// let solutions: Vec<_> = solver.solve(grid)?.take(2).collect();
/// match solutions.len() {
///     0 => println!("No solution"),
///     1 => println!("Unique solution"),
///     _ => println!("Multiple solutions"),
/// }
/// # Ok::<(), sudoku_solver::SolverError>(())
/// ```

#[derive(Debug, Clone)]
pub struct BacktrackSolver {
    technique: TechniqueSolver,
}

impl BacktrackSolver {
    /// Creates a new backtracking solver with the specified technique solver.
    #[must_use]
    pub fn new(technique: TechniqueSolver) -> Self {
        Self { technique }
    }

    /// Creates a solver with all available techniques enabled.
    #[must_use]
    pub fn with_all_techniques() -> Self {
        Self::new(TechniqueSolver::with_all_techniques())
    }

    /// Creates a solver with only the specified techniques.
    #[must_use]
    pub fn with_techniques(techniques: Vec<BoxedTechnique>) -> Self {
        Self::new(TechniqueSolver::new(techniques))
    }

    /// Creates a pure backtracking solver with no techniques.
    ///
    /// This solver relies entirely on backtracking to find solutions.
    #[must_use]
    pub fn pure_backtrack() -> Self {
        Self::new(TechniqueSolver::new(vec![]))
    }

    /// Solves the puzzle and returns an iterator over all solutions.
    ///
    /// The iterator yields solutions in the order they are found during the
    /// search. Each solution comes with statistics about how it was found.
    ///
    /// # Errors
    ///
    /// Returns [`SolverError::Contradiction`] if the initial grid is inconsistent
    /// (has cells with no candidates or contradictory placements).
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku_core::CandidateGrid;
    /// use sudoku_solver::BacktrackSolver;
    ///
    /// let solver = BacktrackSolver::with_all_techniques();
    /// let grid = CandidateGrid::new();
    ///
    /// // Get first solution only
    /// match solver.solve(grid)?.next() {
    ///     Some((solution, stats)) => {
    ///         println!("Found solution!");
    ///         println!("Assumptions: {}", stats.assumptions().len());
    ///         println!("Backtracks: {}", stats.backtrack_count());
    ///     }
    ///     None => println!("No solution exists"),
    /// }
    /// # Ok::<(), sudoku_solver::SolverError>(())
    /// ```
    pub fn solve(&self, mut grid: CandidateGrid) -> Result<Solutions<'_>, SolverError> {
        let mut stats = BacktrackSolverStats::new();
        let solved = self.solve_by_technique(&mut grid, &mut stats)?;
        let solutions = if solved {
            Solutions::solved(self, grid, stats)
        } else {
            let assumption = find_best_assumption(&grid);
            Solutions::with_assumptions(self, grid, stats, assumption)
        };
        Ok(solutions)
    }

    fn solve_by_technique(
        &self,
        grid: &mut CandidateGrid,
        stats: &mut BacktrackSolverStats,
    ) -> Result<bool, SolverError> {
        let solved = self
            .technique
            .solve_with_stats(grid, &mut stats.technique)?;
        Ok(solved)
    }
}

/// Finds the best cell to make an assumption for.
///
/// Selects the cell with the minimum number of remaining candidates (MRV heuristic).
///
/// # Preconditions
///
/// The grid must be consistent (no cells with zero candidates) and not fully solved.
fn find_best_assumption(grid: &CandidateGrid) -> (Position, DigitCandidates) {
    let [empty, decided, cells @ ..] = grid.classify_cells::<10>();
    assert!(empty.is_empty() && decided.len() < 81);
    let pos = cells.iter().find_map(|cells| cells.first()).unwrap();
    (pos, grid.candidates_at(pos))
}

/// An iterator over solutions to a Sudoku puzzle.
///
/// Created by [`BacktrackSolver::solve`]. Yields solutions along with statistics
/// about how each solution was found.

#[derive(Debug, Clone)]
pub struct Solutions<'a> {
    solver: &'a BacktrackSolver,
    stack: Vec<SearchState>,
}

#[derive(Debug, Clone)]
struct SearchState {
    grid: CandidateGrid,
    stats: BacktrackSolverStats,
    assumption: Option<(Position, DigitCandidates)>,
}

impl SearchState {
    fn solved(grid: CandidateGrid, stats: BacktrackSolverStats) -> Self {
        Self {
            grid,
            stats,
            assumption: None,
        }
    }

    fn with_assumption(
        grid: CandidateGrid,
        stats: BacktrackSolverStats,
        assumption: (Position, DigitCandidates),
    ) -> Self {
        Self {
            grid,
            stats,
            assumption: Some(assumption),
        }
    }
}

impl<'a> Solutions<'a> {
    fn solved(
        solver: &'a BacktrackSolver,
        grid: CandidateGrid,
        stats: BacktrackSolverStats,
    ) -> Self {
        Self {
            solver,
            stack: vec![SearchState::solved(grid, stats)],
        }
    }

    fn with_assumptions(
        solver: &'a BacktrackSolver,
        grid: CandidateGrid,
        stats: BacktrackSolverStats,
        assumption: (Position, DigitCandidates),
    ) -> Self {
        Self {
            solver,
            stack: vec![SearchState::with_assumption(grid, stats, assumption)],
        }
    }
}

impl Iterator for Solutions<'_> {
    type Item = (CandidateGrid, BacktrackSolverStats);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut state) = self.stack.pop() {
            let Some((pos, remaining_digits)) = &mut state.assumption else {
                return Some((state.grid, state.stats));
            };
            let Some(digit) = remaining_digits.pop_first() else {
                continue;
            };
            let pos = *pos;
            let mut grid = state.grid.clone();
            let mut stats = state.stats.clone();
            self.stack.push(state);

            stats.assumptions.push((pos, digit));
            grid.place(pos, digit);
            if grid.is_solved() {
                return Some((grid, stats));
            }
            let Ok(solved) = self.solver.solve_by_technique(&mut grid, &mut stats) else {
                stats.backtrack_count += 1;
                continue;
            };
            if solved {
                return Some((grid, stats));
            }
            let assumption = find_best_assumption(&grid);
            self.stack
                .push(SearchState::with_assumption(grid, stats, assumption));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use sudoku_core::{Digit, Position};

    use super::*;

    #[test]
    fn test_pure_backtrack_solver() {
        let solver = BacktrackSolver::pure_backtrack();
        let grid = CandidateGrid::new();

        // Should be able to solve even without techniques
        let result = solver.solve(grid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_solve_with_all_techniques() {
        let solver = BacktrackSolver::with_all_techniques();
        let mut grid = CandidateGrid::new();

        // Create a naked single
        for digit in Digit::ALL {
            if digit != Digit::D5 {
                grid.remove_candidate(Position::new(4, 4), digit);
            }
        }
        grid.place(Position::new(4, 4), Digit::D5);

        let result = solver.solve(grid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_solve_returns_iterator() {
        let solver = BacktrackSolver::with_all_techniques();
        let mut grid = CandidateGrid::new();

        // Simple setup
        grid.place(Position::new(0, 0), Digit::D1);

        let solutions = solver.solve(grid).unwrap();

        // Should be able to iterate
        let count = solutions.take(5).count();
        assert!(count > 0);
    }

    #[test]
    fn test_stats_solved_without_assumptions() {
        let stats = BacktrackSolverStats::new();

        // Empty stats should indicate no assumptions
        assert!(stats.solved_without_assumptions());
    }

    #[test]
    fn test_stats_getters() {
        let stats = BacktrackSolverStats::new();

        assert_eq!(stats.assumptions().len(), 0);
        assert_eq!(stats.backtrack_count(), 0);
        assert!(stats.solved_without_assumptions());
        assert!(stats.technique().total_steps() == 0);
    }

    #[test]
    fn test_stats_with_technique() {
        let tech_stats = TechniqueSolverStats::new();
        let stats = BacktrackSolverStats::with_technique(tech_stats);

        assert_eq!(stats.assumptions().len(), 0);
        assert_eq!(stats.backtrack_count(), 0);
    }

    #[test]
    fn test_contradiction_in_initial_grid() {
        let solver = BacktrackSolver::with_all_techniques();
        let mut grid = CandidateGrid::new();

        // Create a contradiction: remove all candidates from a cell
        for digit in Digit::ALL {
            grid.remove_candidate(Position::new(0, 0), digit);
        }

        let result = solver.solve(grid);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SolverError::Contradiction));
    }

    #[test]
    fn test_multiple_solutions() {
        let solver = BacktrackSolver::pure_backtrack();
        let grid = CandidateGrid::new();

        // Empty grid has multiple solutions - verify we can get at least 2
        let solutions: Vec<_> = solver.solve(grid).unwrap().take(2).collect();
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_backtrack_count() {
        let stats = BacktrackSolverStats::new();

        // New stats should have zero backtrack count
        assert_eq!(stats.backtrack_count(), 0);
    }

    #[test]
    fn test_with_techniques_constructor() {
        use crate::technique::{BoxedTechnique, NakedSingle};

        let techniques: Vec<BoxedTechnique> = vec![Box::new(NakedSingle::new())];
        let solver = BacktrackSolver::with_techniques(techniques);

        let grid = CandidateGrid::new();
        let result = solver.solve(grid);
        assert!(result.is_ok());
    }
}
