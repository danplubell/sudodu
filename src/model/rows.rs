use crate::model::row::Row;

#[derive(Clone, PartialEq, Debug)]
pub struct Rows {
    values: Vec<Row>,
}

impl Rows {
    pub(crate) fn add_row(&mut self, row: Row) {
        self.values.push(row);
    }
    pub fn values(self) -> Vec<Row> {
        self.values
    }
}

impl Rows {
    pub fn new()->Self {
        Self {
            values: Vec::new()
        }
    }
}