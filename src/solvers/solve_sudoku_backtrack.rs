use crate::model_inner::grid::Grid;
const N:usize = 9;
pub fn solve_sudoku_backtrack(grid: &Grid, mut _row:usize, mut _col:usize)->bool {
    match grid.find_unassigned_location() {
        Some((row,col)) => {
            for num in 1..=N {
                if grid.check_is_safe(row, col, num as u8) {
                    grid.set_value_at_row_col(row, col, num as u8);
                    if solve_sudoku_backtrack(grid, row, col) {
                        return true;
                    }
                    grid.set_value_at_row_col(row, col, 0);
                }
            }
        },
        None=> return true
    }
    false
}
#[cfg(test)]
mod tests {
    use crate::model_inner::grid::Grid;
    use crate::solvers::solve_sudoku_backtrack::solve_sudoku_backtrack;

    #[test]
    fn test_solve_sudoku_backtrack() {
        let puzzle_data = "306508400520000000087000031003010080900863005050090600130000250000000074005206300";
        let solution = "316578492529134768487629531263415987974863125851792643138947256692351874745286319";
        assert_eq!(puzzle_data.len(), 81);
        let mut grid = Grid::new();
        grid.try_from(puzzle_data).expect("TODO: panic message");
        let r = solve_sudoku_backtrack(&grid, 0, 0);
        assert!(r);
        let result: String = grid.raw_cells().values().iter().map(|c| char::from_digit(c.get_value() as u32, 10).unwrap()).collect();
        assert_eq!(result, solution);
    }
}