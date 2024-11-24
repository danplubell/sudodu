use crate::model::cell::Cell;
use crate::model::validate_cells::validate_cells;

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Col {
    values: [Cell; 9],
}

impl Col {
    pub fn new() -> Self {
        Col {
            values: [Cell::new(0); 9],
        }
    }
    pub fn with_cells(cells: [Cell;9]) -> Self {
        Self {
            values: cells
        }
    }
    pub(crate) fn values(&self) -> [Cell;9] {
        self.values
    }
    // gets the value for a row in a column
    pub(crate) fn get_at_row(&self, row: usize) -> Cell {
       self.values[row]
    }
    
    pub fn is_valid(&self) -> bool {
        let r = validate_cells(self.values);
        r.is_ok()
    }

     
    pub fn add_cell_at(&mut self, index: usize, cell: Cell) {
        self.values[index] = cell;
    }
}

#[cfg(test)]
mod tests {
    use std::array;
    use crate::model::cell::Cell;
    use crate::model::col::Col;

    #[test]
    fn test_add_cell_at() {
        let mut col = Col::new();
        col.add_cell_at(0, Cell::new(1));
        col.add_cell_at(1, Cell::new(2));
        col.add_cell_at(2, Cell::new(3));
        col.add_cell_at(3, Cell::new(4));

        assert_eq!(col.values()[0], Cell::new(1));
        assert_eq!(col.values()[1], Cell::new(2));
        assert_eq!(col.values()[2], Cell::new(3));
        assert_eq!(col.values()[3], Cell::new(4));
    }
    #[test]
    fn test_with_new(){
        let cells = [Cell::new(1); 9];
        let col = Col::with_cells(cells);
        assert_eq!(col.values()[0], Cell::new(1));
    }
    
    #[test]
    fn test_is_invalid() {
        let col = Col::new();
        assert!(col.is_valid());
    }
    #[test]
    fn test_is_valid(){
        let cells = array::from_fn(|i| Cell::new((i + 1) as u8)); 
        let col = Col::with_cells(cells);
        assert!(col.is_valid());
    }
    #[test]
    fn test_get_at_row(){
        let cells = array::from_fn(|i| Cell::new((i + 1) as u8));
        let col = Col::with_cells(cells);
        assert_eq!(col.get_at_row(0),Cell::new(1));
    }
}
