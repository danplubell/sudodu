use crate::model_inner::cell::Cell;
use crate::model_inner::cells::Cells;
use crate::model_inner::is_safe::is_safe;
use crate::model_inner::validate_cells::validate_cells;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Columns {
    values: Rc<RefCell<Vec<Cells>>>,
}
impl Columns {
    pub fn new() -> Self {
        Self {
            values: Rc::new(RefCell::new((0..9).map(|_| Cells::new()).collect())),
        }
    }
    pub fn add_to_column(&self, col: usize, cell: Cell) {
        if let Some(column) = self.values.borrow_mut().get_mut(col) {
            column.add_cell(cell);
        }
    }
    pub fn get_column(&self, col: usize) -> Cells {
        self.values.borrow().get(col).unwrap().clone()
    }
    pub fn collect_columns(&self, cells: &Cells) {
        let values = cells.values();
        let chunks = values.chunks(9);
        for c in chunks {
            for (col, cell) in c.iter().enumerate() {
                self.add_to_column(col, cell.clone());
            }
        }
    }
    pub fn values(&self) -> Vec<Cells> {
        self.values.borrow().clone()
    }
    pub fn is_valid(&self) -> bool {
        self.values
            .borrow()
            .iter()
            .all(|c| validate_cells(c).is_ok())
    }
    pub fn is_safe(&self) -> bool {
        self.values.borrow().iter().all(|c| is_safe(c).is_ok())
    }
}
#[cfg(test)]
mod tests {
    use crate::model_inner::cell::Cell;
    use crate::model_inner::cells::Cells;
    use crate::model_inner::columns::Columns;

    #[test]
    fn test_add_to_column() {
        let columns = Columns::new();
        let cell = Cell::new(8);
        columns.add_to_column(0, cell);
        let cell1 = Cell::new(7);
        columns.add_to_column(1, cell1);

        let column_check1 = columns.get_column(0);
        assert_eq!(column_check1.get_at(0), Cell::new(8));
        let column_check2 = columns.get_column(1);
        assert_eq!(column_check2.get_at(0), Cell::new(7));
    }
    #[test]
    fn test_collect_cols() {
        let puzzle_data =
            "310450900072986143906010508639178020150090806004003700005731009701829350000645010";

        let cells = Cells::from(puzzle_data);
        let columns = Columns::new();
        columns.collect_columns(&cells);
        assert_eq!(columns.values().len(), 9);
        println!("{:?}", columns);
    }
}
