use crate::model_inner::cells::Cells;

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
    pub not_found: Vec<u8>,
    pub multiple: Vec<u8>,
}

pub fn validate_cells(cells: &Cells) -> Result<bool, ValidateCellsResults> {
    let mut cell_reg = [0u8; 10];
    let mut not_found: Vec<u8> = Vec::new();
    let mut multiple: Vec<u8> = Vec::new();
    
    // count the number of times a value is in the cells collection
    cells.values().iter().for_each(|c| {
        let r = cell_reg.get_mut(c.get_value() as usize);
        if let Some(v) = r {
            *v += 1
        }
    });
    // skip 0 and see if a number is missing or has multiples
    cell_reg.iter().skip(1).enumerate().for_each(|(i,c)|{
        match *c {
            0 => not_found.push(i as u8),
            2.. => multiple.push(i as u8),
            _=>{}
        }
    });
    
    // return results
    match (not_found.is_empty(), multiple.is_empty()) {
        (true, true) => Ok(true),
        _ => Err(ValidateCellsResults {
            not_found,
            multiple,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::model_inner::cells::Cells;
    use crate::model_inner::validate_cells::validate_cells;

    #[test]
    fn test_valid_cells() {
        let ok_values = [1u8,2,3,4,5,6,7,8,9];
        let ok_cells = Cells::new();
        ok_values.iter().for_each(|x| {ok_cells.add_cell_by_value(*x)});
        let r = validate_cells(&ok_cells);
        assert!(r.is_ok());
    }
    #[test]
    fn test_validate_invalid_cells() {
        let not_ok_values = [1,2,4,4,5,5,7,8,9];
        let not_ok_cells = Cells::new();
        not_ok_values.iter().for_each(|x| {not_ok_cells.add_cell_by_value(*x)});
        let r = validate_cells(&not_ok_cells);
        let err = r.unwrap_err();
        assert_eq!(err.not_found, [2, 5]);
        assert_eq!(err.multiple, [3,4 ]);
        
    }
}
