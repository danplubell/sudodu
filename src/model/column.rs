use crate::model::cell::Cell;
use crate::model::cells::Cells;

#[derive(PartialEq, Debug, Clone)]
pub struct Column {
    values: Cells,
}
impl Column {
    pub fn new()-> Self{
        Self {
            values: Cells::new()
        }
    }
    pub fn add_cell(&mut self, cell:Cell) {
        self.values.add_cell(cell);
    }
    pub fn get_at(&self, index: usize) -> Cell{
        self.values.get_at(index).clone()
    }
    pub fn get_values(self)-> Cells {
        self.values
    }

}

#[cfg(test)] 
mod tests {
    use crate::model::cell::Cell;
    use crate::model::column::Column;

    #[test]
    fn test_add_cell() {
        let mut column = Column::new();
        column.add_cell(Cell::new(0));
        column.add_cell(Cell::new(2));
        let c = column.get_at(1);
        assert_eq!(c.get_value(),2);
    }
}