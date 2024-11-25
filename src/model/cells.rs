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
    pub fn has_value(&self, value: u8) -> bool {
        self.values.iter().any(|c| c.value() == value)
    }
    pub fn is_sa
}

impl<'a> IntoIterator for &'a Cells {
    type Item = &'a Cell;
    type IntoIter = Iter<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
impl From<&str> for Cells {
    fn from(value: &str) -> Self {
        let mut v = Vec::new();
        value.chars().for_each(|c| {
            if let Some(d) = c.to_digit(10) { v.push(Cell::new(d as u8)) }
        });
        Self {values: v}
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
    #[test]
    fn test_from() {
        let s = "1234";
        let c = Cells::from(s);
        assert!(c.has_value(1));
        assert!(c.has_value(2));
        assert!(c.has_value(3));
        assert!(c.has_value(4));
            
    }
}
