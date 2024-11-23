use crate::model::grid::Grid;
use crate::solvers::solve_sudoku_backtrack::solve_sudoku_backtrack;

const N: usize = 9;
pub fn solve_sudoku_notes(grid: &Grid, mut row: usize, mut col: usize) -> bool {
    // go through all the cells
    for row in 0..9 {
        for col in 0..9 {
            let cell = grid.get_value_at_row_col(row, col);
            if cell == 0 {
                for num in 1..=N {
                    if grid.check_is_safe(row, col, num as u8) {
                        grid.add_note_at_row_col(row, col, num as u8);
                    }
                }
            }
            let notes = grid.raw_cells().get_notes_at_row_col(row, col);
        }
    }
    match grid.find_unassigned_location() {
        Some((row, col)) => {
            let note_option = grid.raw_cells().get_notes_at_row_col(row, col);
           
            let notes = match note_option {
                Some(n) => n.borrow().clone(),
                None => vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            };
            for num in notes.iter() {
                if grid.check_is_safe(row, col, *num ) {
                    grid.set_value_at_row_col(row, col, *num);
                    //clear value from any notes at row, col, and region
                    grid.clear_note(row,col,num);
                    if solve_sudoku_backtrack(grid, row, col) {
                        return true;
                    }
                    grid.set_value_at_row_col(row, col, 0);
                }
            }
            /*
            for num in 1..=N {
                if grid.check_is_safe(row, col, num as u8) {
                    grid.set_value_at_row_col(row, col, num as u8);
                    if solve_sudoku_backtrack(grid, row, col) {
                        return true;
                    }
                    grid.set_value_at_row_col(row, col, 0);
                }
            }

             */
        }
        None => return true,
    }
    false
}
#[cfg(test)]
mod tests {
    use crate::model::grid::Grid;
    use crate::solvers::solve_sudoku_backtrack::solve_sudoku_backtrack;
    use crate::solvers::solve_sudoku_notes::solve_sudoku_notes;

    #[test]
    fn test_solve_sudoku_notes() {
        let puzzle_data =
            "306508400520000000087000031003010080900863005050090600130000250000000074005206300";
        let solution =
            "316578492529134768487629531263415987974863125851792643138947256692351874745286319";
        assert_eq!(puzzle_data.len(), 81);
        let mut grid = Grid::new();
        grid.try_from(puzzle_data).expect("TODO: panic message");
        let r = solve_sudoku_notes(&grid, 0, 0);
       
        assert!(r);
        let result: String = grid.raw_cells().values().iter().map(|c| char::from_digit(c.get_value() as u32, 10).unwrap()).collect();
        assert_eq!(result, solution);
    }
}
