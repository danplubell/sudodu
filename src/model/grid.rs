use crate::model::cells::Cells;
use crate::model::columns::Columns;
use crate::model::is_safe::is_safe_placement;
use crate::model::regions::Regions;
use crate::model::rows::Rows;

#[derive(Clone, PartialEq, Debug)]
pub struct Grid {
    raw_cells: Cells,
    rows: Rows,
    columns: Columns,
    regions: Regions,
}
impl Grid {
    pub fn new() -> Self {
        Self {
            raw_cells: Cells::new(),
            rows: Rows::new(),
            columns: Columns::new(),
            regions: Regions::new(),
        }
    }
        pub fn with_cells(&self, cells: &Cells) -> &Grid {
            self.rows.collect_rows(cells);
            self.columns.collect_columns(cells);
            self.regions.collect_regions(cells);
            self
        }
    fn try_from(&self, value: &str) -> Result<(), ParsePuzzleError> {
        if value.chars().any(|c| !c.is_numeric()) {
            return Err(ParsePuzzleError::HasAlpha);
        }
        if value.len() > 81 {
            return Err(ParsePuzzleError::TooLong);
        }
        if value.len() < 81 {
            return Err(ParsePuzzleError::TooShort);
        }
        let cells = Cells::from(value);
        self.rows.collect_rows(&cells);
        self.columns.collect_columns(&cells);
        self.regions.collect_regions(&cells);
        Ok(())
    }
    pub fn validate(&self) -> bool {
        let rows_valid = self.rows.is_valid();
        let cols_valid = self.columns.is_valid();
        let regions_valid = self.regions.is_valid();
        matches!((rows_valid, cols_valid, regions_valid), (true, true, true))
    }
    pub fn check_is_safe(&self,row: usize, col:usize, num: u8) -> bool{
        is_safe_placement(self.raw_cells.clone(), row, col,num)
    }
    pub fn is_safe(&self) -> bool {
        let rows_valid = self.rows.is_safe();
        let cols_valid = self.columns.is_safe();
        let regions_valid = self.regions.is_safe();
        matches!((rows_valid, cols_valid, regions_valid), (true, true, true))
    }
}
#[derive(Clone, PartialEq, Debug, thiserror::Error)]
pub enum ParsePuzzleError {
    #[error("Value string is too long")]
    TooLong,
    #[error("Value string is too short")]
    TooShort,
    #[error("Value string contains alphabetic characters")]
    HasAlpha,
}

#[cfg(test)]
mod tests {
    use crate::model::grid::{Grid, ParsePuzzleError};

    #[test]
    fn too_short() {
        let grid = Grid::new();
        let err = grid.try_from("1234").unwrap_err();
        println!("{}", err);
        assert_eq!(err, ParsePuzzleError::TooShort);
    }

    #[test]
    fn not_all_digits() {
        let grid = Grid::new();
        let err = grid.try_from("aaaaaaaaaaaaaaa").unwrap_err();
        assert_eq!(err, ParsePuzzleError::HasAlpha);
    }
    #[test]
    fn validate_valid_grid() {
        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let grid = Grid::new();
        let _ = grid.try_from(solution);
        
        assert!(grid.validate());
    }
    #[test]
    fn validate_invalid_grid() {
        let puzzle_data = "310450900072986143906010508639178020150090806004003700005731009701829350000645010";
        let grid = Grid::new();
        let _ = grid.try_from(puzzle_data);
        assert!(!grid.validate());
    }

}