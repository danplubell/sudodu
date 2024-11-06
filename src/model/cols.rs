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
