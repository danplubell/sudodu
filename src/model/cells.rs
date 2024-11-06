use crate::model::cell::Cell;
use std::vec::IntoIter;

#[derive(PartialEq, Debug, Clone)]
pub struct Cells {
    values: Vec<Cell>,
}

impl Cells {
    pub fn new(cells: Vec<Cell>) -> Self {
        Cells { values: cells }
    }
    pub fn values(self) -> Vec<Cell> {
        self.values
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
        assert_eq!(
            cells,
            Cells::new(vec!(Cell::new(1), Cell::new(2), Cell::new(3), Cell::new(4)))
        );
        assert_eq!(
            cells.values(),
            vec!(Cell::new(1), Cell::new(2), Cell::new(3), Cell::new(4))
        );
    }
}
