use crate::model::col::Col;

#[derive(Clone, PartialEq, Debug)]
pub struct Cols {
    values: Vec<Col>,
}
impl Cols {
    pub fn new(cols: Vec<Col>) -> Self {
        Self {
            values: cols,
        }
    }
    pub fn values(self) -> Vec<Col> {
        self.values
    }
}