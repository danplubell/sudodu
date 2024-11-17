use crate::model::grid::Grid;
const N: u8 = 9;
pub fn solve_sudoku_naive(grid: &Grid, mut row:u8, mut col:u8) -> bool{
    if (row == N - 1 && col == N) {
        true
    };
    // Check if column value  becomes 9 , 
    // we move to next row and
    //  column start from 0
    if (col == N) {
        row += 1;
        col = 0;
    };
    // Check if the current position of 
    // the grid already contains
    // value >0, we iterate for next column
    if (grid.get_value_at_row_col(row,col) > 0) {
        return solve_sudoku_naive(&grid, row, col + 1);
    }
    // Check if the current position of 
    // the grid already contains
    // value >0, we iterate for next column
    if (grid.get_value_at_row_col(row, col) > 0) {
        return solve_sudoku_naive(&grid, row, col + 1);
    }
   // for (int num = 1; num <= N; num++)
    for num in 1..=N 
    {

        // Check if it is safe to place 
        // the num (1-9)  in the
        // given row ,col  ->we 
        // move to next column
        if (grid.is_safe_placement(row, col, num))
        {

            /* Assigning the num in 
               the current (row,col)
               position of the grid
               and assuming our assigned 
               num in the position
               is correct     */
            grid.set_at_row_col(row, col,num) = num;

            //  Checking for next possibility with next
            //  column
            if (solve_sudoku_naive(grid, row, col + 1)) {
                return true;
            }
        }

        // Removing the assigned num , 
        // since our assumption
        // was wrong , and we go for 
        // next assumption with
        // diff num value
        grid.set_at_row_col(row,col,0);
    }
    return false;
}