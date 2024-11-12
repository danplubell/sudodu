use crate::model::cell::Cell;
use crate::model::col::Col;

#[derive(Clone,PartialEq, Debug)]
pub struct Cols {
    values: [Col; 9],
}

impl Cols {
    pub fn new() -> Self {
        Self {
            values: [Col::new(); 9],
        }
    }
    pub fn with_cols(cols: [Col; 9]) -> Self {
        Self { values: cols }
    }
    pub(crate) fn add_to_row_column(&mut self, row: usize, cell: Cell, col: usize) {
        self.values[col].add_cell_at(row, cell);
    }
    pub fn value_at_row_col(&self, row: usize, col: usize) -> Cell {
        self.values[col].get_at_row(row)
    }
    pub fn add_col(&mut self, index:usize, col: Col) {
        self.values[index]=col;
    }

    /*
    pub fn values(&mut self) -> &mut Vec<Col> {
        &mut self.values
    }

     */

    pub fn iter(&self) -> impl Iterator<Item = &Col> {
        self.values.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cols::Cols;

    #[test]
    fn test_add_column() {
        let mut cols = Cols::new();
        cols.add_to_row_column(0, Cell::new(5), 0);
        assert_eq!(cols.value_at_row_col(0, 0), Cell::new(5));
        cols.add_to_row_column(1, Cell::new(6), 0);
        assert_eq!(cols.value_at_row_col(0, 1), Cell::new(6));
    }
    #[test]
    fn test_value_at_row_col() {
        let mut cols = Cols::new();
        cols.add_to_row_column(0, Cell::new(5), 0);
        let r = cols.value_at_row_col(0, 0);
        assert_eq!(r, Cell::new(5));
        cols.add_to_row_column(1, Cell::new(6), 1);
        let r = cols.value_at_row_col(1, 1);
        assert_eq!(r, Cell::new(6));
    }
}
