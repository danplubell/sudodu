#[derive(Clone, PartialEq, Debug)]
pub struct Puzzle(Vec<u8>);

impl Puzzle {
    fn new(values: &str) -> Self {
        Puzzle::try_from(values).unwrap()
    }
}
#[derive(Clone, PartialEq, Debug, thiserror::Error)]
enum ParsePuzzleError {
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
