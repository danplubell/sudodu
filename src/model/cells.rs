use crate::model::cell::Cell;
use std::slice::Chunks;
use std::vec::IntoIter;

#[derive(PartialEq, Debug, Clone)]
pub struct Cells {
    values: Vec<Cell>,
}
impl Cells {
    pub fn new() -> Self {
        Cells { values: Vec::new() }
    }
    pub(crate) fn with_cells(cells: Vec<Cell>) -> Self {
        Cells { values: cells }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.values.iter()
    }
    pub fn get_at(&self, index: usize) -> Option<&Cell> {
        self.values.get(index)
    }
    pub fn get_chunks(&self, size: usize) -> Chunks<Cell> {
        self.values.chunks(size)
    }
    pub fn add_cell(&mut self, cell: Cell) {
        self.values.push(cell)
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
}
impl From<&str> for Cells {
    fn from(value: &str) -> Self {
        let values = value
            .chars()
            .map(|c| Cell::new(c.to_digit(10).unwrap() as u8))
            .collect();
        Self { values }
    }
}
impl FromIterator<Cell> for Cells {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut values = Vec::new();
        for c in iter {
            values.push(c)
        }
        Cells { values }
    }
}

impl IntoIterator for Cells {
    type Item = Cell;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter() // We use the iterator from the Vec
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;
    #[test]
    fn from_str_test() {
        let _solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let numbers = "1234";
        let cells = Cells::from(numbers);
        let mut expected = Cells::new();
        expected.add_cell(Cell::new(1));
        expected.add_cell(Cell::new(2));
        expected.add_cell(Cell::new(3));
        expected.add_cell(Cell::new(4));
        assert_eq!(cells, expected);
    }
    #[test]
    fn with_cells_test() {
        let v = vec![Cell::new(1), Cell::new(2)];
        let cells = Cells::with_cells(v);
        assert_eq!(cells.get_at(0), Some(&Cell::new(1)))
    }
}
