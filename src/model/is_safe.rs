use crate::model::cells::Cells;
use crate::model::grid::Grid;
use crate::model::validate_cells::ValidateCellsResults;

pub fn is_safe_placement(cells: Cells, row: usize, col: usize, value: u8) -> bool {
    let grid = Grid::new();
    let test_cells = Cells::new();
    cells
        .values()
        .iter()
        .for_each(|c| test_cells.add_cell_by_value(c.get_value()));
    test_cells.set_inner_at_row_col(row, col, value);
    let grid = grid.with_cells(&test_cells);
    grid.is_safe()
}

pub fn is_safe(cells: &Cells) -> Result<bool, ValidateCellsResults> {
    let mut cell_reg = [0u8; 10];
    let mut multiple: Vec<u8> = Vec::new();
    let not_found: Vec<u8> = Vec::new();
    // count the number of times a value is in the cells collection
    cells.values().iter().for_each(|c| {
        let r = cell_reg.get_mut(c.get_value() as usize);
        if let Some(v) = r {
            *v += 1
        }
    });
    // skip 0 and see if a number is missing or has multiples
    cell_reg
        .iter()
        .skip(1)
        .enumerate()
        .for_each(|(i, c)| if let 2.. = *c { multiple.push(i as u8) });

    // return results
    match multiple.is_empty() {
        true => Ok(true),
        _ => Err(ValidateCellsResults {
           not_found,
            multiple,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cells::Cells;
    use crate::model::is_safe::{is_safe, is_safe_placement};

    #[test]
    fn test_save_add() {
        let _solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let puzzle_data =
            "310450900072986143906010508639178020150090806004003700005731009701829350000645010";
        let cells = Cells::from(puzzle_data);
        assert!(is_safe_placement(cells, 0, 2, 8))
    }
    #[test]
    fn test_unsafe_placement() {
        let puzzle_data =
            "310450900072986143906010508639178020150090806004003700005731009701829350000645010";
        let cells = Cells::from(puzzle_data);
        assert!(!is_safe_placement(cells, 0, 2, 9))

    }
    #[test]
    fn test_is_not_safe() {
        let not_ok_values = [1,2,4,4,5,5,7,8,9];
        let not_ok_cells = Cells::new();
        not_ok_values.iter().for_each(|x| {not_ok_cells.add_cell_by_value(*x)});
        let r = is_safe(&not_ok_cells);
        let err = r.unwrap_err();
        assert_eq!(err.multiple, [3,4 ]);
    }
    #[test]
    fn test_is_safe_zeros() {
        let not_ok_values = [1,2,4,0,5,0,7,8,9];
        let not_ok_cells = Cells::new();
        not_ok_values.iter().for_each(|x| {not_ok_cells.add_cell_by_value(*x)});
        let r = is_safe(&not_ok_cells);
        
        assert!(r.is_ok());
    }
    #[test]
    fn test_is_safe() {
        let not_ok_values = [1,2,4,3,5,6,7,8,9];
        let not_ok_cells = Cells::new();
        not_ok_values.iter().for_each(|x| {not_ok_cells.add_cell_by_value(*x)});
        let r = is_safe(&not_ok_cells);

        assert!(r.is_ok());
    }

}
