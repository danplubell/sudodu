use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::validate_cells::validate_cells;

#[derive(Clone, PartialEq, Debug)]
pub struct Row {
    values: Cells
}
impl Row {
    pub fn new(vec: Vec<Cell>) -> Self {
        Self {
            values: Cells::new(vec),
        }
    }
    pub fn is_valid(&self) -> bool {
        let r = validate_cells(&self.values);
        match r {
            Ok(_) => true,
            _  => false
        }
    }
}