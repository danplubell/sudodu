use crate::model::puzzle::Puzzle;

fn is_safe(puzzle: Puzzle, row:usize, col:usize, value: u8) -> bool{
    let candidate = puzzle.cells().clone();
    let test_puzzle = Puzzle::with_cells(candidate);
    
    true
}