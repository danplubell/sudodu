use crate::model::cell::Cell;

#[derive(Clone, PartialEq, Debug )]
pub struct Column<'a> {
    values: Vec<&'a Cell>
}

impl<'a> Column<'a> {
    pub fn new() -> Self {
        Self {
            values: Vec::with_capacity(9)
        }
    }
    pub fn add_cell(&mut self, cell: &'a Cell) {
        self.values.push(cell);
    }
    pub fn values(self) -> Vec<&'a Cell> {
        self.values
    }
}