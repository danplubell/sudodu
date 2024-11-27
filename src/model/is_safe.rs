use crate::model::cells::Cells;

pub fn is_safe(cells: &Cells) -> bool{
    let mut value_list = [0u8; 10];
    // count the number of times a value is in the cells collection
    cells.iter().for_each(|c| {
        let r = value_list.get_mut(c.value() as usize);
        if let Some(v) = r {
            *v += 1
        }
    });
    value_list.iter().any(|v| *v > 2)
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;
    use crate::model::is_safe::is_safe;

    #[test]
    fn test_is_safe_fail() {
        let mut cells = Cells::new();
        cells.add_cell(Cell::new(2));
        cells.add_cell(Cell::new(2));
        assert!(!is_safe(&cells));
    }
    fn test_is_safe_safe(){
        #[test]
        fn test_is_safe_fail() {
            let mut cells = Cells::new();
            cells.add_cell(Cell::new(1));
            cells.add_cell(Cell::new(2));
            cells.add_cell(Cell::new(0));
            cells.add_cell(Cell::new(0));
            assert!(is_safe(&cells));
        }

    }
}