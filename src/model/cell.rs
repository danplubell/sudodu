#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub(crate) value: u8,
}
impl Cell {
    pub fn new(value: u8) -> Self {
        Self {
            value
        }
    }
    pub fn get_value(&self) -> u8 {
        self.value
    } 
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;

    #[test]
    fn test_new(){
        let c = Cell::new(1u8);
        assert_eq!(c.get_value(),1 );
    }
}
