use crate::model::cell::Cell;
use crate::model::cells::Cells;

#[derive(Debug, thiserror::Error)]
#[error("`{invalid_number}` is not a valid number")]
pub struct ValidationError {
    invalid_number: u8,
    reason: ErrorReason,
}
#[derive(Clone, PartialEq, Debug, thiserror::Error)]
enum ErrorReason {
    #[error("Number not found")]
    NotFound,
    #[error("Multiple instances")]
    Multiple(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct ValidateCellsResults {
    not_found: Vec<u8>,
    multiple: Vec<u8>,
}

pub fn validate_cells(cells: &Cells) -> Result<bool, ValidateCellsResults> {
    let mut cell_reg = [0u8; 10];
    let mut not_found: Vec<u8> = Vec::new();
    let mut multiple: Vec<u8> = Vec::new();

    cells.iter().for_each(|c| {
        let r = cell_reg.get_mut(c.value as usize);
        if let Some(v) = r {
            *v += 1
        }
    });
    cell_reg.iter().skip(1).enumerate().for_each(|(i,c)|{
        match *c {
            0 => not_found.push(i as u8),
            2.. => multiple.push(i as u8),
            _=>{}
        }
    });
    match (not_found.is_empty(), not_found.is_empty()) {
        (true, true) => Ok(true),
        _ => Err(ValidateCellsResults {
            not_found,
            multiple,
        }),
    }
}
pub fn v_cells(cells: &[Cell]) -> Result<bool, ValidateCellsResults> {
    let mut not_found: Vec<u8> = Vec::new();
    let mut multiple: Vec<u8> = Vec::new();

    for i in 1..=9 {
        let mut count = 0;

        cells.iter().for_each(|z| {
            if z.value == i {
                count += 1;
            }
        });
        match count {
            0 => not_found.push(1),
            2.. => multiple.push(1),
            _ => {}
        }
    }
    match (not_found.is_empty(), not_found.is_empty()) {
        (true, true) => Ok(true),
        _ => Err(ValidateCellsResults {
            not_found,
            multiple,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;
    use crate::model::validate_cells::validate_cells;

    #[test]
    fn test_multiple_cells() {
        let ok_cells = vec![
            Cell::new(1),
            Cell::new(2),
            Cell::new(3),
            Cell::new(4),
            Cell::new(5),
            Cell::new(6),
            Cell::new(7),
            Cell::new(8),
            Cell::new(9),
        ];
        let cells = Cells::new(ok_cells);
        let r = validate_cells(&cells);
        println!("{:?}", r);
    }
    #[test]
    fn test_validate_invalid_cells() {
        let not_ok_cells = vec![
            Cell::new(1),
            Cell::new(2),
            Cell::new(4),
            Cell::new(4),
            Cell::new(5),
            Cell::new(5),
            Cell::new(7),
            Cell::new(8),
            Cell::new(9),
        ];
        let cells = Cells::new(not_ok_cells);
        let r = validate_cells(&cells);
        println!("{:?}", r);
    }
}
