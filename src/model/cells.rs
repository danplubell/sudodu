use std::vec::IntoIter;
use crate::model::cell::Cell;

#[derive(PartialEq, Debug, Clone)]
pub struct Cells{
    values: Vec<Cell>,
}

impl Cells {
    pub fn new() -> Self {
        Cells {
            values: Vec::new()
        }
    }
    pub fn add_cell(&mut self, cell: Cell) {
        self.values.push(cell)
    }
    pub fn get_call_at(&self, index: usize) -> Option<u8> {
        self.values.get(index).map(|c| c.get_value()) 
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
}#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;

    #[test]
    fn add_cells() {
        let mut cells = Cells::new();
        cells.add_cell(Cell::new(1));
        cells.add_cell(Cell::new(2));
        for c in cells {
            println!("{:?}", c);
        }
    }
}