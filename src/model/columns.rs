use crate::model::cell::Cell;
use crate::model::column::Column;

#[derive(Clone, PartialEq, Debug)]
pub struct Columns<'a> {
    values: Vec<Column<'a>>,
}

impl<'a> Columns<'a> {
    pub fn new() -> Self {
        Self {
            values: vec![Column::new();9]//Vec::with_capacity(9),
        }
    }
    pub(crate) fn add_to_column(&mut self, col: usize, cell: &'a Cell) {
        let column = self.values.get_mut(col).unwrap();
        column.add_cell(cell)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Column> {
        self.values.iter()
    }
    pub fn values(self) -> Vec<Column<'a>> {
        self.values
    }
}
