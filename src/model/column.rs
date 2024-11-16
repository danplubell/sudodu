use crate::model::cell::Cell;
use crate::model::cells::Cells;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Column {
    values: Rc<RefCell<Cells>>,
}
impl Column {
    pub fn new() -> Self {
        Self {
            values: Rc::new(RefCell::new(Cells::new())),
        }
    }
    pub fn add_cell(&self, cell: Cell) {
        self.values.borrow_mut().add_cell(cell);
    }
    pub fn get_at(&self, index: usize) -> Cell {
        self.values.borrow().get_at(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::column::Column;

    #[test]
    fn test_add_cell() {
        let column = Column::new();
        column.add_cell(Cell::new(0));
        column.add_cell(Cell::new(2));
        let c = column.get_at(1);
        assert_eq!(c.get_value(), 2);
    }
}
