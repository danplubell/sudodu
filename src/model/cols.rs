use crate::model::cell::Cell;
use crate::model::col::Col;

#[derive( PartialEq, Debug)]
pub struct Cols<'a> {
    values: [Option<&'a Col>;9],
}

impl<'a> Cols<'a> {
    
    pub(crate) fn add_to_column(&mut self, index: usize, cell: Cell) {
        let col = &mut self.values.get_mut(index);
        let cell = Cell::new(cell.value());
        match col {
            Some(c) => {
                c.add(cell);
            }
            None => {
                let mut new_col = Col::new();
                new_col.values().push(cell);
                self.values().push(new_col)
            }
        }
    }
    pub fn value_at_row_col(&mut self, row:usize, col:usize) -> Option<Cell> {
        let col= self.get_col(col);
        col.and_then(|c| c.get_row(row).map(|r| Cell::new(r.value)))
    }

    fn get_col(&mut self, idx: usize) -> Option<&mut Option<&'a Col>> {
        self.values.get_mut(idx)

    }
    pub fn validate(&self) -> bool {
        true
    }
    pub fn new() -> Self {
        Self { values: [None;9] }
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
        cols.add_to_column(0, &Cell::new(5));
        assert_eq!(cols.values()[0].values()[0].value, 5);
    }
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
}
