use crate::model::cell::Cell;
use crate::model::row::Row;

#[derive(Clone, PartialEq, Debug)]
pub struct Rows {
    values: Vec<Row>,
}

impl Rows {
    pub(crate) fn add_row(&mut self, row: Row) {
        self.values.push(row);
    }
    pub fn iter(&self ) ->impl Iterator<Item = &Row>{
        self.values.iter()
    }
}

impl Rows {
    pub fn new()->Self {
        Self {
            values: Vec::new()
        }
    }
}