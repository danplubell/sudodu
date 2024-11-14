use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::column::Column;

#[derive(Clone, PartialEq, Debug)]
pub struct Columns<'a> {
    values: Vec<Column<'a>>,
}

impl<'a> Columns<'a> {
    pub fn new() -> Self {
        Self {
            values: vec![Column::new();9]//Vec::with_capacity(9),
        }
    }
    pub fn collect_columns(&mut self,cells: &'a mut Cells)  {
        // let mut cols = Columns::new();
        let chunks = cells.get_chunks(9);
        for (row,c) in chunks.enumerate() {
            for (col, cell) in c.iter().enumerate() {
                self.add_to_column(col, cell );
            }
        }
    }

    pub(crate) fn add_to_column(&mut self, col: usize, cell: &'a Cell) {
        let column = self.values.get_mut(col).unwrap();
        column.add_cell(cell)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Column> {
        self.values.iter()
    }
    pub fn values(self) -> Vec<Column<'a>> {
        self.values
    }
}

#[cfg(test)]
mod tests {
    use crate::model::columns::Columns;
    use crate::model::cells::Cells;

    #[test]
    fn test_collect_cols() {
        let s9 = "123456789";
        let ss9 = [s9; 9];
        let sol9 = ss9.join("");

        let mut cells;
        {
            let mut columns = Columns::new();
            cells = Cells::from(sol9.as_str());
            columns.collect_columns(&mut cells);
            println!("initial columns {:?}", columns);
        }

        let mut new_cells;
        let mut columns;
        {
            columns = Columns::new();
            new_cells = Cells::from(sol9.as_str());
            columns.collect_columns(&mut new_cells);
        }
        new_cells.set_at(0, 8u8);
        println!("changed columns {:?}", columns);

    }
}