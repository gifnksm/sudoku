use sudoku_generator::PuzzleGenerator;
use sudoku_solver::TechniqueSolver;

fn main() {
    let solver = TechniqueSolver::with_all_techniques();
    let generator = PuzzleGenerator::new(&solver);

    let puzzle = generator.generate();
    println!("{puzzle:?}");
}
