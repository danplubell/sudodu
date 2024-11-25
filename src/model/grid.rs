use std::fmt::{Display, Formatter};
use crate::model::cells::Cells;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    values: Cells
}

impl Grid {
    pub fn new() -> Self{
        Self {
            values: Cells::new(),
        }
    }
}
impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            values: Cells::from(value),
        }
    }
}