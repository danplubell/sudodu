use crate::model::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Cells {
    values: Rc<RefCell<Vec<Cell>>>,
}
impl Cells {
    pub fn new() -> Self {
        Self { values: Rc::new(RefCell::new(Vec::new())) }
    }
    pub fn add_cell_by_value(&self, value: u8) {
        self.values.borrow_mut().push(Cell::new(value));
    }
    pub fn add_cell(&self, cell: Cell) {
        self.values.borrow_mut().push(cell);
    }
    pub fn get_inner_value_at(&self, index: usize) -> u8 {
        match self.values.borrow().get(index) {
            Some(v) => v.get_value(),
            None => {println!("index: {} {}", index, self.values.borrow().len(), ); 0}
        }
//        self.values.borrow().get(index).unwrap().get_value()
    }
    pub fn set_inner_value_at(&self, index: usize, value: u8) {
        if let Some(cell) = self.values.borrow().get(index) {
            cell.replace_value(value);
        }
    }
    pub fn set_inner_at_row_col(&self, row: usize, col: usize, value: u8) {
        let index = row * 9 + col;
        self.set_inner_value_at(index, value);
    }
    pub fn get_inner_at_row_col(&self, row: usize, col: usize) -> u8 {
        let index = row * 9 + col;
        self.get_inner_value_at(index)
    }
    pub fn get_at(&self, index: usize) -> Cell {
        self.values.borrow().get(index).cloned().unwrap()
    }
    pub fn values(&self) -> Vec<Cell> {
        self.values.borrow().clone()
    }
}

impl From<&str> for Cells {
    fn from(value: &str) -> Self {
        let values = value
            .chars()
            .map(|c| Cell::new(c.to_digit(10).unwrap() as u8)).collect();

        Self { values: Rc::new(RefCell::new(values)) }
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
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;

    #[test]
    fn test_add_cell_by_value() {
        let cells = Cells::new();
        cells.add_cell_by_value(5);
        assert_eq!(cells.values().len(), 1);
    }
    #[test]
    fn test_get_at() {
        let cells = Cells::new();
        cells.add_cell_by_value(5);
        cells.add_cell_by_value(6);
        assert_eq!(cells.get_inner_value_at(1), 6);
    }
    #[test]
    fn test_from() {
        let numbers = "1234";
        let cells = Cells::from(numbers);
        let  expected = Cells::new();
        expected.add_cell_by_value(1);
        expected.add_cell_by_value(2);
        expected.add_cell_by_value(3);
        expected.add_cell_by_value(4);
        assert_eq!(cells, expected);
    }
    #[test]
    fn test_set_at() {
        let cells = Cells::new();
        cells.add_cell_by_value(10);
        cells.set_inner_value_at(0, 20);
        let cell_value = cells.get_inner_value_at(0);
        assert_eq!(cell_value, 20);
    }
    #[test]
    fn test_set_at_row_col() {
        let cells = Cells::new();
        for i in 0..18 {
            cells.add_cell_by_value((i % 9) as u8)
        }
        cells.set_inner_at_row_col(0, 0, 9);
        cells.set_inner_at_row_col(1, 0, 9);
        assert_eq!(cells.get_inner_value_at(0), 9);
        assert_eq!(cells.get_inner_value_at(9), 9);
    }
    #[test]
    fn test_get_at_row_col() {
        let cells = Cells::new();

        for i in 0..18 {
            cells.add_cell_by_value((i % 9) as u8)
        }
        assert_eq!(cells.get_inner_at_row_col(0, 0), 0);
        assert_eq!(cells.get_inner_at_row_col(1, 0), 0);
    }
    #[test]
    fn test_add_cell() {
        let cells = Cells::new();
        let ref_cells = Cells::new();
        for i in 0..18 {
            cells.add_cell(Cell::new((i % 9) as u8))
        }
        assert_eq!(cells.get_inner_value_at(0), 0);
        assert_eq!(cells.values().len(), 18);
        for c in cells.values() {
            ref_cells.add_cell(c.clone());
        }
        println!("{:?}", ref_cells);
        // set the source of truth position 0 to 9
        cells.set_inner_value_at(0, 9);
        // ref_cells should show the new value because it is a reference to the source
        println!("{:?}", ref_cells);
        assert_eq!(ref_cells.get_inner_value_at(0), 9);
    }
}
