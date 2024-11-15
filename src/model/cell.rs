use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Cell {
    value: Rc<RefCell<u8>>
}
impl Cell {
    pub fn new(value: u8) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }
    pub fn value(&self) -> u8 {
        self.value.take()
    }
    pub fn replace(&self, value: u8) {
        self.value.replace(value);
    }
}