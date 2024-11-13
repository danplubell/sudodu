/// A cell in the soduko grid is represented here
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
     value: u8,
}
impl Cell {
    /// Creates a new instance of the cell from a value
    pub fn new(value: u8) -> Self {
        Self { value }
    }
    /// Gets the value of the Cell
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;

    #[test]
    fn new_get_cell() {
        let cell = Cell::new(5);
        assert_eq!(cell.value, 5);
    }
}
