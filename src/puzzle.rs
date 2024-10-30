#[derive(Clone, PartialEq, Debug)]
pub struct Puzzle(Vec<u8>);

impl Puzzle {
    fn new(values: &str) -> Self {
        Puzzle::try_from(values).unwrap()
    }
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
        if value.len() > 81 {
            return Err(ParsePuzzleError::TooLong);
        }
        if value.len() < 81 {
            return Err(ParsePuzzleError::TooShort);
        }
        if value.chars().any(|c| !c.is_numeric()) {
            return Err(ParsePuzzleError::HasAlpha);
        }
        let n = value
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        Ok(Puzzle(n))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::Puzzle;
    use std::convert::TryFrom;

    #[test]
    fn too_long(){
        let status = Puzzle::try_from("1234").unwrap();
        assert_eq!(status.0, vec!(1,2,3,4));

    }
    #[test]
    fn not_all_digits() {
        let puzzle = Puzzle::try_from("abcd").unwrap();
        println!("{:?}",puzzle.0);
        assert_eq!(puzzle.0, vec!());
    }
}