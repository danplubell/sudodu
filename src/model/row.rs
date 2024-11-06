use crate::model::cell::Cell;
use crate::model::cells::Cells;

#[derive(Clone, PartialEq, Debug)]
pub struct Row {
    value: Cells
}
impl Row {
    pub fn new(vec: Vec<Cell>) -> Self {
        Self {
            value: Cells::new(vec),
        }
    }
}