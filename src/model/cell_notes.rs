use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub struct CellNotes {
    values: Vec<u8>,
}
impl CellNotes {
    pub fn new() -> Self {
        CellNotes {
            values: Vec::new()
        }
    }
    pub fn iter(&self) -> Iter<u8> {
        self.values.iter()
    }
    pub fn add_note(&mut self, value: u8) {
        self.values.push(value);
    }
    pub fn remove_note(&mut self, value: u8) {
        self.values.iter().position(|v|*v == value).map(|p| self.values.remove(p));
    }
    pub fn has_note(&self, value: u8) -> bool {
        self.values.iter().any(|v| *v == value)
    }
}
impl<'a> IntoIterator for &'a CellNotes {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell_notes::CellNotes;
    #[test]
    fn test_new() {
        let cell_notes = CellNotes::new();
        assert_eq!(cell_notes.values.len(), 0);
    }
    #[test]
    fn test_add() {
        let mut cell_notes = CellNotes::new();
        cell_notes.add_note(3);
        assert_eq!(cell_notes.values.len(),1);
        assert!(cell_notes.has_note(3));
        cell_notes.remove_note(3);
        assert!(!cell_notes.has_note(3));
    }
    #[test]
    fn test_iter() {
        let mut cell_notes = CellNotes::new();
        for v in 1..=9 {
            cell_notes.add_note(v);
        }
        let r = cell_notes.iter().as_slice();
        assert_eq!(r, [1,2,3,4,5,6,7,8,9]);
    }
    #[test]
    fn test_into_iter() {
        let mut cell_notes = CellNotes::new();
        for v in 1..=9 {
            cell_notes.add_note(v);
        }
        let r = cell_notes.into_iter().as_slice();
        assert_eq!(r,[1,2,3,4,5,6,7,8,9]);
    }
}