use crate::model::cell_notes::CellNotes;

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    value: u8,
    notes: CellNotes,
}

impl Cell {
    pub fn new(value: u8) -> Self {
        Cell { value, notes: CellNotes::new()}
    }
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn add_note(&mut self, value:u8) {
        self.notes.add_note(value);
    }
    pub fn remove_note(&mut self, value: u8) {
        self.notes.remove_note(value);
    }
    pub fn has_note(&self, value: u8)->bool {
        self.notes.has_note(value)
    }
    pub fn note_count(&self)->usize{
        self.notes.iter().len()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    #[test]
    fn test_new() {
        let cell = Cell::new(3);
        assert_eq!(cell.value(), 3);
    }
    #[test]
    fn test_add() {
        let mut cell = Cell::new(3);
        cell.add_note(3);
        assert_eq!(cell.note_count(),1);
        assert!(cell.has_note(3));
        cell.remove_note(3);
        assert!(!cell.has_note(3));
    }
}