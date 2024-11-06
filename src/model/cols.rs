use crate::model::cell::Cell;
use crate::model::col::Col;

#[derive(Clone, PartialEq, Debug)]
pub struct Cols {
    values: Vec<Col>,
}

impl Cols {
    pub(crate) fn add_to_column(&mut self, index: usize, cell: &Cell) {
        let col = self.values.get_mut(index);
        let cell = Cell::new(cell.value);
        if let Some(c) = col {
            c.values().push(cell);
        }
    }
}

impl Cols {
    pub fn new() -> Self {
        Self {
            values: vec![Col::new(); 9],
        }
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
    fn test_new() {
        let mut cols = Cols::new();
        assert_eq!(cols.values().len(), 9);
    }#[test]
    fn test_add_column() {
        let mut cols = Cols::new();
        cols.add_to_column(0,&Cell::new(5) );
        println!("{:?}", cols.values()[0]);
    }
}