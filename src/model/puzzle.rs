use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::collect_cols::collect_cols;
use crate::model::collect_rows::collect_rows;
use crate::model::cols::Cols;
use crate::model::regions::Regions;
use crate::model::rows::Rows;

#[derive(Clone, PartialEq, Debug)]
pub struct Puzzle {
    data: Cells,
    regions: Regions,
    rows: Rows,
    cols: Cols,
}
impl Puzzle {
    fn new(value: &str) -> Result<Self, ParsePuzzleError> {
        Puzzle::try_from(value)
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
        let data = value
            .chars()
            .map(|c| Cell {
                value: c.to_digit(10).unwrap() as u8,
            })
            .collect();
        //regions
        //rows
        //cols
        let rows:Rows = collect_rows(&data);
        let cols:Cols = collect_cols(&data);
        let regions:Regions = collect_regions(&data);
        Ok(Puzzle { data, regions, rows, cols })
    }
}



#[cfg(test)]
mod tests {
    use crate::puzzle::{ParsePuzzleError, Puzzle};
    use std::convert::TryFrom;

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
}
