use crate::model::cell::Cell;
use crate::model::validate_cells::validate_cells;

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Region {
    values: [Cell;9]
}
impl Region {
    pub fn new() -> Self {
        Self {
            values: [Cell::new(0); 9],
        }
    }
    pub fn with_cells(cells: [Cell;9]) -> Self {
        Self {
            values: cells
        }
    }
    pub fn is_valid(&self) -> bool {
        let r = validate_cells(self.values);
        r.is_ok()
    }
}