use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::validate_cells::validate_cells;

#[derive(Clone, PartialEq, Debug)]
pub struct Col {
    values: Vec<Cell>
}

impl Col {
    pub fn new() -> Self {
        Col {
            values:  Vec::new(),
        }
    }

    pub(crate) fn values(&mut self) -> &mut Vec<Cell> {
        &mut self.values
    }
    pub(crate) fn get_row(&mut self, row: usize) -> Option<&mut Cell> { 
        self.values().get_mut(row)
    }
    pub fn is_valid(&self) -> bool {
        let cells = Cells::new(&self.values);
        let r = validate_cells(&self.values);
        match r {
            Ok(_) => true,
            _  => false
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::model::col::Col;

    #[test]
    fn test_new() {
        let mut col = Col::new();
        assert_eq!(col.values().len(), 9)
    }
}