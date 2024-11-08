use crate::model::cell::Cell;

#[derive(Clone, PartialEq, Debug)]
pub struct Region {
    values: Vec<Cell>
}
impl Region {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self {
            values: cells
        }
    }
}