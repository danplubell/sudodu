#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
     value: u8,
}
impl Cell {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
    pub fn value(&self) -> u8 {
        self.value
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
