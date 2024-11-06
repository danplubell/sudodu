use crate::model::cell::Cell;

#[derive(Clone, PartialEq, Debug)]
pub struct Col {
    values: Vec<Cell>
}

impl Col {
    pub(crate) fn values(self) -> Vec<Cell> {
        self.values
    }
}

impl Col {
    pub fn new(cells: Vec<Cell>) -> Self {
        Col {
            values: cells,
        }
    }
}