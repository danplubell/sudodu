use crate::model_inner::grid::Grid;
const N: usize = 9;
/// A naive sudoku solver that uses recursion to generate a solution
pub fn solve_sudoku_naive(grid: &Grid, mut row:usize, mut col:usize) -> bool{
    if row == N - 1 && col == N {
        return true
    }
    // Check if column value  becomes 9 , 
    // we move to next row and
    //  column start from 0
    if col == N {
        row += 1;
        col = 0;
    };
    // Check if the current position of 
    // the grid already contains
    // value >0, we iterate for next column
    if grid.get_value_at_row_col(row,col) > 0 {
        return solve_sudoku_naive(grid, row, col + 1);
    }
    // Check if the current position of 
    // the grid already contains
    // value >0, we iterate for next column
    if grid.get_value_at_row_col(row, col) > 0 {
        return solve_sudoku_naive(grid, row, col + 1);
    }
   // for (int num = 1; num <= N; num++)
    for num in 1..=N 
    {

        // Check if it is safe to place 
        // the num (1-9)  in the
        // given row ,col  ->we 
        // move to next column
        if grid.check_is_safe(row, col, num as u8)
        {

            /* Assigning the num in 
               the current (row,col)
               position of the grid
               and assuming our assigned 
               num in the position
               is correct     */
            grid.set_value_at_row_col(row, col,num as u8);

            //  Checking for next possibility with next
            //  column
            if solve_sudoku_naive(grid, row, col + 1) {
                return true;
            }
        }

        // Removing the assigned num , 
        // since our assumption
        // was wrong , and we go for 
        // next assumption with
        // diff num value
        grid.set_value_at_row_col(row,col,0);
    }
    false
}
#[cfg(test)]
mod tests {
    use crate::model_inner::grid::Grid;
    use crate::solvers::solve_sudoku_naive::solve_sudoku_naive;

    #[test]
    fn test_solve_sudoku_naive() {
        let puzzle_data = "306508400520000000087000031003010080900863005050090600130000250000000074005206300";
        let solution = "316578492529134768487629531263415987974863125851792643138947256692351874745286319";
        assert_eq!(puzzle_data.len(), 81);
        let mut grid = Grid::new();
        grid.try_from(puzzle_data).expect("TODO: panic message");
        let r = solve_sudoku_naive(&grid, 0, 0);
        assert!(r);
        let result: String = grid.raw_cells().values().iter().map(|c| char::from_digit(c.get_value() as u32, 10).unwrap()).collect();
        assert_eq!(result, solution);
    }
}