use crate::model::cell::Cell;

#[derive(PartialEq, Debug)]
pub struct Col {
    values: [Cell; 9],
}

impl Col {
    pub fn new() -> Self {
        Col {
            values: [Cell::new(0); 9],
        }
    }

    pub(crate) fn values(&self) -> &[Cell] {
        &self.values
    }
    // gets the value for a row in a column
    pub(crate) fn get_row(&self, row: usize) -> Option<&Cell> {
        self.values().get(row)
    }
    /*
    pub fn is_valid(&self) -> bool {
        let r = validate_cells(self.values);
        match r {
            Ok(_) => true,
            _  => false
        }
    }

     */
    pub fn add_cell(&mut self, index: usize, cell: Cell) {
        self.values[index] = cell;
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::col::Col;

    #[test]
    fn test_new() {
        let mut col = Col::new();
        col.add_cell(0, Cell::new(1));
        col.add_cell(1, Cell::new(2));
        col.add_cell(2, Cell::new(3));
        col.add_cell(3, Cell::new(4));

        assert_eq!(col.values()[0], Cell::new(1));
        assert_eq!(col.values()[1], Cell::new(2));
        assert_eq!(col.values()[2], Cell::new(3));
        assert_eq!(col.values()[3], Cell::new(4));
    }
}
