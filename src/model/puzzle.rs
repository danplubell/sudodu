use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::collect_cols::collect_cols;
use crate::model::collect_regions::collect_regions;
use crate::model::collect_rows::collect_rows;
use crate::model::cols::Cols;
use crate::model::regions::Regions;
use crate::model::rows::Rows;

#[derive(Clone, PartialEq, Debug)]
pub struct Puzzle {
    cells: Cells,
    regions: Regions,
    rows: Rows,
    cols: Cols,
}
impl Puzzle {
    fn new(value: &str) -> Result<Self, ParsePuzzleError> {
        Puzzle::try_from(value)
    }
    fn validate_puzzle(&self) -> bool {
        let rows_valid = self.rows.iter().all(|row| row.is_valid());
        let cols_valid = self.cols.iter().all(|col| col.is_valid());
        let regions_valid = self.regions.iter().all(|region|region.is_valid());
        matches!((rows_valid, cols_valid, regions_valid), (true,true, true))
    }
    pub(crate) fn cells(self) -> Cells {
        self.cells
    }
    pub fn with_cells(cells: Cells) -> Self {
        let rows:Rows = collect_rows(&cells);
        let cols:Cols = collect_cols(&cells);
        let regions:Regions = collect_regions(&cells);
        Puzzle { cells, cols, rows, regions }
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
impl TryFrom<&str> for Puzzle {
    type Error = ParsePuzzleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
        let rows:Rows = collect_rows(&cells);
        let cols:Cols = collect_cols(&cells);
        let regions:Regions = collect_regions(&cells);
        Ok(Puzzle { cells, cols, rows, regions })
    }
}



#[cfg(test)]
mod tests {
    use crate::model::puzzle::{ParsePuzzleError, Puzzle};

    #[test]
    fn too_short() {
        let err = Puzzle::try_from("1234").unwrap_err();
        println!("{}", err);
        assert_eq!(err, ParsePuzzleError::TooShort);
    }

    #[test]
    fn not_all_digits() {
        let err = Puzzle::try_from("aaaaaaaaaaaaaaa").unwrap_err();
        assert_eq!(err, ParsePuzzleError::HasAlpha);
    }
    #[test]
    fn validate_valid_puzzle() {
        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let puzzle = Puzzle::new(solution);
        let p = puzzle.unwrap();
        assert!(p.validate_puzzle());
    }
    #[test]
    fn validate_invalid_puzzle() {
        let puzzle_data = "310450900072986143906010508639178020150090806004003700005731009701829350000645010";
        let puzzle = Puzzle::new(puzzle_data);
        let p = puzzle.unwrap();
        assert!(!p.validate_puzzle());
    }
}
