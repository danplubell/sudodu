use crate::model::cell::Cell;

#[derive(PartialEq, Debug)]
pub struct Cells<'a> {
    values: [&'a Cell],
}

/*
use std::vec::IntoIter;
impl IntoIterator for Cells {
    type Item = Cell;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter() // We use the iterator from the Vec
    }
}


 */
impl Sized for Cells<'a> {}

impl FromIterator<Cell> for Cells {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut values = Vec::new();
        for c in iter {
            values.push(c)
        }
        Cells { values }
    }
}
