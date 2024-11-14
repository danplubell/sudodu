use crate::model::cell::Cell;

#[derive(Clone, PartialEq, Debug)]
pub struct Column<'a> {
    values: Vec<&'a Cell>,
}

impl<'a> Column<'a> {
    pub fn new() -> Self {
        Self {
            values: Vec::with_capacity(9),
        }
    }
    pub fn add_cell(&mut self, cell: &'a Cell) {
        self.values.push(cell);
    }
    pub fn values(self) -> Vec<&'a Cell> {
        self.values
    }
    pub fn get_at(&self, index: usize) -> &'a Cell {
        self.values.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cells::Cells;
    use crate::model::column::Column;

    #[test]
    fn test_add_cell() {
        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let mut cells = Cells::from(solution);
        let mut column = Column::new();
        {
            let chunks = cells.get_chunks(9);
            let (_, cell_chunk) = chunks.enumerate().next().unwrap();
            for (i, c) in cell_chunk.iter().enumerate() {
                column.add_cell(c);
            }
            for i in 0..9 {
                assert_eq!(column.get_at(i).value(), cells.get_at(i).unwrap().value())
            }
        }
        cells.set_at(0, 9);
        assert_eq!(column.get_at(0).value(), 9)
    }
}
