#[derive(Clone, PartialEq, Debug)]
pub struct Puzzle{
    data: Vec<u8>,
    regions: Vec<Region>,
    rows: Vec<Rows>,
    cols: Vec<Cols>,
}
#[derive(Clone, PartialEq, Debug, thiserror::Error)]
pub enum ParsePuzzleError {
    #[error("Value string is too long")]
    TooLong,
    #[error("Value string is too short")]
    TooShort,
    #[error("Value string contains alphabetic characters")]
    HasAlpha,
}
impl TryFrom<&str> for Puzzle {
    type Error = ParsePuzzleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().any(|c| !c.is_numeric()) {
            return Err(ParsePuzzleError::HasAlpha);
        }
        if value.len() > 81 {
            return Err(ParsePuzzleError::TooLong)
        }
        if value.len() < 81 {
            return Err(ParsePuzzleError::TooShort);
        }
        let n = value
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        Ok(Puzzle { data: n})
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::{ParsePuzzleError, Puzzle};
    use std::convert::TryFrom;

    #[test]
    fn too_short(){
        let err = Puzzle::try_from("1234").unwrap_err();
        println!("{}", err);
        assert_eq!(err, ParsePuzzleError::TooShort);
        
    }
    
    #[test]
    fn not_all_digits() {
        let err = Puzzle::try_from("aaaaaaaaaaaaaaa").unwrap_err();
       
        assert_eq!(err, ParsePuzzleError::HasAlpha);
    }
}