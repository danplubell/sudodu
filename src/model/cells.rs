use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Cells {
    values: Vec<Rc<RefCell<Cell<u8>>>>,
}
impl Cells {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn add_cell(&mut self, value: u8) {
        self.values.push(Rc::new(RefCell::new(Cell::new(value))));
    }
    pub fn get_at(&self, index: usize) -> u8 {
        self.values.get(index).unwrap().take().take()
    }
    pub fn set_at(&mut self, index: usize, value: u8) {
        if let Some(cell) = self.values.get(index) {
            cell.borrow_mut().replace(value);
        }
    }
    pub fn set_at_row_col(&mut self, row: usize, col: usize, value: u8) {
        let index = row * 9 + col;
        self.set_at(index, value);
    }
    pub fn get_at_row_col(&self, row:usize, col:usize)-> u8 {
        let index = row * 9 + col;
        self.get_at(index)
    }
}

impl From<&str> for Cells {
    fn from(value: &str) -> Self {
        let values = value
            .chars()
            .map(|c| Rc::new(RefCell::new(Cell::new(c.to_digit(10).unwrap() as u8))))
            .collect();
        Self { values }
    }
}

/*
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

     */

#[cfg(test)]
mod tests {
    use crate::model::cells::Cells;

    #[test]
    fn test_add_cell() {
        let mut cells = Cells::new();
        cells.add_cell(5);
        assert_eq!(cells.values.len(), 1);
    }
    #[test]
    fn test_get_at() {
        let mut cells = Cells::new();
        cells.add_cell(5);
        cells.add_cell(6);
        assert_eq!(cells.get_at(1), 6);
    }
    #[test]
    fn test_from() {
        let numbers = "1234";
        let cells = Cells::from(numbers);
        let mut expected = Cells::new();
        expected.add_cell(1);
        expected.add_cell(2);
        expected.add_cell(3);
        expected.add_cell(4);
        assert_eq!(cells, expected);
    }
    #[test]
    fn test_set_at() {
        let mut cells = Cells::new();
        cells.add_cell(10);
        cells.set_at(0, 20);
        let cell_value = cells.get_at(0);
        assert_eq!(cell_value, 20);
    }
    #[test]
    fn test_set_at_row_col() {
        let mut cells = Cells::new();
        for i in 0..18 {
            cells.add_cell((i % 9) as u8)
        }
        cells.set_at_row_col(0,0,9);
        cells.set_at_row_col(1,0, 9);
        assert_eq!(cells.get_at(0), 9);
        assert_eq!(cells.get_at(9), 9);
    }
    #[test]
    fn test_get_at_row_col() {
        let mut cells = Cells::new();

        for i in 0..18 {
            cells.add_cell((i % 9) as u8)
        }
        assert_eq!(cells.get_at_row_col(0,0), 0);
        assert_eq!(cells.get_at_row_col(1,0), 0);
    }
}
