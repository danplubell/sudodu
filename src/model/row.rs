use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::validate_cells::validate_cells;

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Row {
    values: [Cell; 9],
}
impl Row {
    pub fn new() -> Self {
        Self {
            values: [Cell::new(0); 9],
        }
    }
    pub fn with_cells(cells: [Cell; 9]) -> Self {
        Self { values: cells }
    }
    pub fn is_valid(&self) -> bool {
        let r = validate_cells(self.values);
        r.is_ok()
    }
}
#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::row::Row;
    use std::array;

    #[test]
    fn test_with_cells() {
        let cells = array::from_fn(|i| Cell::new(i as u8));
        let row = Row::with_cells(cells);
        assert_eq!(row.values[0], Cell::new(0));
    }
    #[test]
    fn test_is_valid_with_valid() {
        let cells = array::from_fn(|i| Cell::new(i as u8 + 1));
        let row = Row::with_cells(cells);
        assert!(row.is_valid());
    }
    #[test]
    fn test_is_valid_with_invalid() {
        let cells = array::from_fn(|_i| Cell::new(1));
        let row = Row::with_cells(cells);
        assert!(!row.is_valid());
    }
}
