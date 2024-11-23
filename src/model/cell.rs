use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Cell {
    value: Rc<RefCell<u8>>,
    notes: Rc<RefCell<Vec<u8>>>
}
impl Cell {
    pub fn new(value: u8) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            notes: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn get_value(&self) -> u8 {
        *self.value.borrow()
    }
    pub fn replace_value(&self, value: u8) {
        *self.value.borrow_mut() = value;
    }
    pub fn add_note_value(&self, value: u8) {
        self.notes.borrow_mut().push(value);
    }
    pub fn get_notes(&self) -> Rc<RefCell<Vec<u8>>> {
        self.notes.clone()
    }
    pub fn clear_note(&self,value: u8) {
        let index = self.notes.borrow_mut().iter().position(|v| *v == value);
        if let Some(index) = index {
            self.notes.borrow_mut().remove(index);
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;

    #[test]
    fn test_get_value() {
        let cell = Cell::new(8);
        assert_eq!(cell.get_value(), 8);
    }
    #[test]
    fn test_replace_value() {
        let cell = Cell::new(9);
        cell.replace_value(0);
        assert_eq!(cell.get_value(), 0);
    }
    #[test]
    fn test_clear_note() {
        let cell = Cell::new(5);
        cell.add_note_value(10);
        cell.clear_note(10);
        assert!(cell.get_notes().borrow().is_empty());
    }
}