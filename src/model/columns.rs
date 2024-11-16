use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::column::Column;
#[derive(PartialEq, Debug, Clone)]
pub struct Columns {
    values: Vec<Column>,
}
impl Columns {
    pub fn new()->Self{
        Self {
            values:  vec![Column::new();9]
        }
    }
    pub fn add_column(&mut self, column: Column) {
        self.values.push(column);
    }
    pub fn add_to_column(&mut self, col:usize,cell:Cell) {
        // get column
        // add the cell
        self.values.get_mut(col).unwrap().add_cell(cell)
    }
    pub fn get_column(&self, col:usize) -> &Column {
        self.values.get(col).unwrap()
    }
    pub fn collect_columns(&mut self,cells: Cells)  {
        let values = cells.values();
        let chunks = values.chunks(9);
        for c in chunks {
            for (col, cell) in c.iter().enumerate() {
                self.add_to_column(col, cell.clone() );
            }
        }
    }
    pub fn values(&self)->Vec<Column>{
        self.clone().values
    }
}
#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;
    use crate::model::columns::Columns;

    #[test]
    fn test_add_to_column(){
        let mut columns = Columns::new();
        let mut cell = Cell::new(8);
        columns.add_to_column(0,cell);
        println!("{:?}",columns);
        cell = Cell::new(7);
        columns.add_to_column(1,cell);
        println!("{:?}",columns);
        let column_check1 = columns.get_column(0);
        assert_eq!(column_check1.get_at(0),Cell::new(8));
        let column_check2 = columns.get_column(1);
        assert_eq!(column_check2.get_at(0), Cell::new(7));
    }
    #[test]
    fn test_collect_cols() {
        let puzzle_data = "310450900072986143906010508639178020150090806004003700005731009701829350000645010";

        let cells = Cells::from(puzzle_data);
        let mut columns = Columns::new();
        columns.collect_columns(cells);
        assert_eq!(columns.values().len(), 9);
    }
}