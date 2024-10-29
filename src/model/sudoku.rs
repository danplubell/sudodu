#[derive(Clone, PartialEq, Debug)]
pub struct Sudoku {
    puzzle: [u8;81],
    solution: [u8;81]
}
impl Sudoku {
    fn new(puzzle: [u8;81], solution: [u8;81]) -> Self {
        Self {
            puzzle,
            solution
        }
    }
}