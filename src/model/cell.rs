#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub(crate) value: u8,
}
impl Cell {
    pub fn new(value: u8) -> Self {
        Self { value }
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
