use crate::model::cell::Cell;
use crate::model::col::Col;

#[derive(Clone, PartialEq, Debug)]
pub struct Cols {
    values: Vec<Col>,
}

impl Cols {
    pub(crate) fn add_to_column(&mut self, index: usize, cell: &Cell) {
        let col = &mut self.values.get_mut(index);
        let cell = Cell::new(cell.value);

        match col {
            Some(c) => {
                c.values().push(cell);
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
        match col {
            Some( c) => {
                let r = c.get_row(row);
                match r {
                    Some(r)=> Some(Cell::new(r.value)),
                    None=> None
                }
            },
            None=> None
            
        }
    }

    fn get_col(&mut self, idx: usize) -> Option<&mut Col> { 
        self.values.get_mut(idx)

    }
}

impl Cols {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn values(&mut self) -> &mut Vec<Col> {
        &mut self.values
    }
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
}
