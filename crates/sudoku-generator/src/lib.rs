#![allow(missing_docs)] // FIXME: remove this after adding doc comments

use rand::{
    Rng, SeedableRng,
    distr::{Distribution, StandardUniform},
    seq::SliceRandom,
};
use rand_pcg::Pcg64;
use sudoku_core::{CandidateGrid, Digit, DigitGrid, DigitSet, Position};
use sudoku_solver::TechniqueSolver;

#[derive(Debug, Clone)]
pub struct PuzzleGenerator<'a> {
    solver: &'a TechniqueSolver,
}

impl<'a> PuzzleGenerator<'a> {
    /// Create a new generator with a solver
    #[must_use]
    pub fn new(solver: &'a TechniqueSolver) -> Self {
        Self { solver }
    }
    /// Generate a puzzle with a random seed
    pub fn generate(&self) -> GeneratedPuzzle {
        Self::generate_with_seed(&self, rand::random())
    }

    /// Generate a puzzle with a specific seed for reproducibility
    pub fn generate_with_seed(&self, seed: PuzzleSeed) -> GeneratedPuzzle {
        let mut rng = Pcg64::from_seed(seed.0);
        let grid = self.generate_solution(&mut rng);
        eprintln!("{grid:#}");

        todo!()
    }

    fn generate_solution<R>(&self, rng: &mut R) -> DigitGrid
    where
        R: Rng,
    {
        let mut grid = CandidateGrid::new();

        // Step 1: Fill the entire first row with shuffled digits 1-9
        let mut top_row = Digit::ALL;
        top_row.shuffle(rng);
        for (x, digit) in (0..9).zip(top_row) {
            let pos = Position::new(x, 0);
            grid.place(pos, digit);
        }

        // Step 2: Fill the remaining 6 cells in the top-left box (3x3)
        // top_row[0..3] are already used in the first row of the top-left box
        // top_row[3..9] contains the 6 digits unused in the top-left box
        let mut remaining: [Digit; 6] = top_row[3..9].try_into().unwrap();
        remaining.shuffle(rng);
        for (i, digit) in (3..9).zip(remaining) {
            let pos = Position::from_box(0, i); // Box 0, positions 3-8
            grid.place(pos, digit);
        }

        // Step 3: Fill the rest of the grid using backtracking with solver assistance
        let mut stack = vec![];
        let assumption = find_best_assumption(&grid);
        stack.push((grid, assumption));

        while let Some((mut grid, (pos, mut digits))) = stack.pop() {
            if digits.is_empty() {
                continue;
            }
            // Pick a random candidate digit and try it
            let digit = digits.pop_nth(rng.random_range(0..digits.len())).unwrap();
            stack.push((grid.clone(), (pos, digits)));
            grid.place(pos, digit);
            if grid.is_solved() {
                return grid.to_digit_grid();
            }
            // Use the solver to fill in cells that can be determined logically
            let Ok((solved, _)) = self.solver.solve(&mut grid) else {
                continue; // Contradiction found, backtrack
            };
            if solved {
                return grid.to_digit_grid();
            }
            // Pick the next cell to fill
            let assumption = find_best_assumption(&grid);
            stack.push((grid, assumption));
        }
        unreachable!("Failed to generate complete grid - this should never happen");
    }
}

/// Find the cell with the minimum number of remaining candidates (MRV heuristic)
fn find_best_assumption(grid: &CandidateGrid) -> (Position, DigitSet) {
    // classify_cells::<10> groups cells by candidate count (0-9)
    // [0]: 0 candidates (contradiction), [1]: 1 candidate (decided), [2..]: 2-9 candidates
    let [empty, decided, cells @ ..] = grid.classify_cells::<10>();
    assert!(empty.is_empty() && decided.len() < 81);

    // Pick the first undecided cell with minimum candidates
    let pos = cells.iter().find_map(|cells| cells.first()).unwrap();
    (pos, grid.candidates_at(pos))
}

/// TODO: add doc comment
#[derive(Debug, Clone, Copy)]
pub struct PuzzleSeed([u8; 32]);

impl Distribution<PuzzleSeed> for StandardUniform {
    fn sample<R>(&self, rng: &mut R) -> PuzzleSeed
    where
        R: Rng + ?Sized,
    {
        PuzzleSeed(rng.random())
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedPuzzle {
    pub problem: DigitGrid,
    pub solution: DigitGrid,
    pub seed: PuzzleSeed,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_generator() -> PuzzleGenerator<'static> {
        // Use a leaked solver to get a 'static reference
        // This is acceptable in tests
        let solver = Box::leak(Box::new(TechniqueSolver::with_all_techniques()));
        PuzzleGenerator::new(solver)
    }

    #[test]
    fn test_generated_solution_is_complete() {
        let generator = create_test_generator();
        let mut rng = Pcg64::from_seed([1u8; 32]);
        let solution = generator.generate_solution(&mut rng);

        // All cells should be filled
        for row in 0..9 {
            for col in 0..9 {
                let pos = Position::new(col, row);
                assert!(
                    solution.get(pos).is_some(),
                    "Cell at ({col}, {row}) should be filled"
                );
            }
        }
    }

    #[test]
    fn test_generated_solution_satisfies_sudoku_constraints() {
        let generator = create_test_generator();
        let mut rng = Pcg64::from_seed([2u8; 32]);
        let solution = generator.generate_solution(&mut rng);

        // Check all rows have digits 1-9
        for row in 0..9 {
            let mut digits = DigitSet::EMPTY;
            for col in 0..9 {
                let pos = Position::new(col, row);
                if let Some(digit) = solution.get(pos) {
                    digits.insert(digit);
                }
            }
            assert_eq!(
                digits,
                DigitSet::FULL,
                "Row {row} should contain all digits 1-9"
            );
        }

        // Check all columns have digits 1-9
        for col in 0..9 {
            let mut digits = DigitSet::EMPTY;
            for row in 0..9 {
                let pos = Position::new(col, row);
                if let Some(digit) = solution.get(pos) {
                    digits.insert(digit);
                }
            }
            assert_eq!(
                digits,
                DigitSet::FULL,
                "Column {col} should contain all digits 1-9"
            );
        }

        // Check all 3x3 boxes have digits 1-9
        for box_idx in 0..9 {
            let mut digits = DigitSet::EMPTY;
            for i in 0..9 {
                let pos = Position::from_box(box_idx, i);
                if let Some(digit) = solution.get(pos) {
                    digits.insert(digit);
                }
            }
            assert_eq!(
                digits,
                DigitSet::FULL,
                "Box {box_idx} should contain all digits 1-9"
            );
        }
    }

    #[test]
    fn test_same_seed_produces_same_solution() {
        let generator = create_test_generator();
        let seed = [42u8; 32];

        let mut rng1 = Pcg64::from_seed(seed);
        let solution1 = generator.generate_solution(&mut rng1);

        let mut rng2 = Pcg64::from_seed(seed);
        let solution2 = generator.generate_solution(&mut rng2);

        // Same seed should produce identical solutions
        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_different_seeds_produce_different_solutions() {
        let generator = create_test_generator();

        let mut rng1 = Pcg64::from_seed([1u8; 32]);
        let solution1 = generator.generate_solution(&mut rng1);

        let mut rng2 = Pcg64::from_seed([2u8; 32]);
        let solution2 = generator.generate_solution(&mut rng2);

        // Different seeds should (almost certainly) produce different solutions
        assert_ne!(solution1, solution2);
    }

    #[test]
    fn test_generated_solution_can_be_verified_by_candidate_grid() {
        let generator = create_test_generator();
        let mut rng = Pcg64::from_seed([5u8; 32]);
        let solution = generator.generate_solution(&mut rng);

        // Verify the solution by placing it in a CandidateGrid
        let mut candidate_grid = CandidateGrid::new();
        for row in 0..9 {
            for col in 0..9 {
                let pos = Position::new(col, row);
                if let Some(digit) = solution.get(pos) {
                    candidate_grid.place(pos, digit);
                }
            }
        }

        // Should be completely solved without contradictions
        assert!(candidate_grid.is_solved());
    }
}
