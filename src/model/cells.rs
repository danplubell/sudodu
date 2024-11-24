use crate::model::cell::Cell;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub struct Cells {
    values: Vec<Cell>,
}

impl Cells {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn iter(&self) -> Iter<Cell> {
        self.values.iter()
    }
    pub fn add_cell(&mut self, cell: Cell) {
        self.values.push(cell);
    }
}

impl<'a> IntoIterator for &'a Cells {
    type Item = &'a Cell;
    type IntoIter = Iter<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;

    #[test]
    fn test_new() {
        let cells = Cells::new();
        assert_eq!(cells.values.len(), 0);
    }
    #[test]
    fn test_add_cell() {
        let mut cells = Cells::new();
        cells.add_cell(Cell::new(3));
        cells.add_cell(Cell::new(4));
        assert_eq!(cells.values.len(), 2);
    }
}
