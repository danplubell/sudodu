use crate::model::cell::Cell;
use crate::model::col::Col;

#[derive( PartialEq, Debug)]
pub struct Cols {
    values: [Col;9],
}

impl Cols {
    
    pub fn new() -> Self {
        Self { values: [Col::new();9] }
    }
    pub(crate) fn add_to_row_column(&mut self, row: usize, cell: Cell, col: usize) {
        self.values[col].add_cell(row,cell);
    }
    pub fn value_at_row_col(&self, row:usize, col:usize) -> Cell {
        self.values[col].get_row(row)
    }

    /*
    pub fn values(&mut self) -> &mut Vec<Col> {
        &mut self.values
    }
    
     */
    /*
    pub fn iter(&self ) ->impl Iterator<Item = &Col>{
        self.values.iter()
    }
    
     */

}


#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cols::Cols;

    
    #[test]
    fn test_add_column() {
        let mut cols = Cols::new();
        cols.add_to_row_column(0, Cell::new(5), 0);
        assert_eq!(cols.value_at_row_col(0,0), Cell::new(5));
        cols.add_to_row_column(1, Cell::new(6), 0);
        assert_eq!(cols.value_at_row_col(0,1), Cell::new(6));
    }/*
    #[test]
    fn test_value_at_row_col() {
        let mut cols = Cols::new();
        cols.add_to_column(0, &Cell::new(5));
        let r = cols.value_at_row_col(0,0);
        assert_eq!(r.unwrap().value,5);
        cols.add_to_column(1, &Cell::new(6));
        let r = cols.value_at_row_col(0,1);
        assert_eq!(r.unwrap().value,6);
    }
    
     */
}
