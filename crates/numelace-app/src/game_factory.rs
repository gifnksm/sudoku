use numelace_game::Game;
use numelace_generator::PuzzleGenerator;
use numelace_solver::TechniqueSolver;

pub fn generate_random_game() -> Game {
    let technique_solver = TechniqueSolver::with_all_techniques();
    let puzzle = PuzzleGenerator::new(&technique_solver).generate();
    Game::new(puzzle)
}
