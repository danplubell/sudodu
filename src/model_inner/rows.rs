use std::cell::RefCell;
use std::rc::Rc;
use crate::model_inner::cell::Cell;
use crate::model_inner::cells::Cells;
use crate::model_inner::is_safe::is_safe;
use crate::model_inner::validate_cells::validate_cells;

#[derive(PartialEq, Debug, Clone)]

pub struct Rows {
    values: Rc<RefCell<Vec<Cells>>>,
}

impl Rows {
    pub fn new()->Self{
        Self {
            values: Rc::new(RefCell::new((0..9).map(|_| Cells::new()).collect())),        }
    }
    pub fn add_to_row(&self, row: usize, cell: Cell) {
        if let Some(row) = self.values.borrow_mut().get_mut(row) {
            row.add_cell(cell);
        }
    }


    pub fn collect_rows(&self, cells: &Cells) {
        let values = cells.values();
        let c = values.chunks(9);
       
        for (i,chunk) in c.enumerate() {
            for cell in chunk {
               self.add_to_row(i, cell.clone());
            }
        }
    }
    pub fn values(&self)->Vec<Cells>{
        self.values.borrow().clone()
    }

    pub fn get_row(&self, row:usize) -> Cells {
        self.values.borrow().get(row).unwrap().clone()
    }
    pub fn get_value_at_row_col(&self,row:usize, col:usize)->u8 {
        self.get_row(row).values().get(col).unwrap().get_value()
    }
    pub fn is_valid(&self)->bool {
        self.values.borrow().iter().all(|c| validate_cells(c).is_ok())
    }
    pub fn is_safe(&self) -> bool {
        self.values.borrow().iter().all(|c| is_safe(c).is_ok())
    }

}
#[cfg(test)]
mod tests {
    use crate::model_inner::cells::Cells;
    use crate::model_inner::rows::Rows;

    #[test]
    fn test_collect_rows() {
        let puzzle_data = "310450900072986143906010508639178020150090806004003700005731009701829350000645010";
        let cells = Cells::from(puzzle_data);
        let rows = Rows::new();
        rows.collect_rows(&cells.clone());
        assert_eq!(rows.values().len(), 9);
        let row = rows.get_row(0);
        assert_eq!(row.values().len(), 9);
        assert_eq!(rows.get_value_at_row_col(0,0), 3);
        
        // replace the cell value
        // the new value should be in the row
        cells.set_inner_value_at(0,9);
        assert_eq!(rows.get_value_at_row_col(0,0), 9);
        
    }
}