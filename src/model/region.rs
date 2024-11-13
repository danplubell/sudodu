use crate::model::cell::Cell;

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
}