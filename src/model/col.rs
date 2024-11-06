use crate::model::cell::Cell;

#[derive(Clone, PartialEq, Debug)]
pub struct Col {
    values: Vec<Cell>
}

impl Col {
    pub(crate) fn values(&mut self) -> &mut Vec<Cell> {
        &mut self.values
    }
}

impl Col {
    pub fn new() -> Self {
        Col {
            values:  Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::col::Col;

    #[test]
    fn test_new() {
        let mut col = Col::new();
        assert_eq!(col.values().len(), 9)
    }
}