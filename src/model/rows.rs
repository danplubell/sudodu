use crate::model::row::Row;

#[derive(Clone, PartialEq, Debug)]
pub struct Rows {
    values: [Row;9]
}

impl Rows {
    pub(crate) fn add_row(&mut self, row: Row, idx: usize) {
        self.values[idx] = row;
    }
    pub fn iter(&self ) ->impl Iterator<Item = &Row>{
        self.values.iter()
    }
    pub fn new()->Self {
        Self {
            values: [Row::new();9]
        }
    }
    pub fn get_row(&self, idx: usize) -> Row {
        self.values[idx]
    }
}

#[cfg(test)]
mod tests {
    use std::array;
    use crate::model::cell::Cell;
    use crate::model::row::Row;
    use crate::model::rows::Rows;

    #[test]
    fn test_new() {
        let rows = Rows::new();
        assert_eq!(rows.values[0],Row::new())
    }

    #[test]
    fn test_add_row() {
        let mut rows = Rows::new();
        let cells = array::from_fn(|i|{
            Cell::new(i as u8)
        });
        let row = Row::with_cells(cells);
        rows.add_row(row,0);
        assert_eq!(rows.get_row(0), row);
    }
}