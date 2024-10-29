#[derive(Clone,PartialEq,Debug)]
pub struct Puzzle([u8;81]);

#[derive(Clone,PartialEq, Debug)]

struct ParsePuzzleError(String);
impl TryFrom<&str> for Puzzle {
    type Error = ParsePuzzleError;
    fn try_from(value: &str) -> Result<Self,Self::Error> {
        if value.len() > 81 {
            return Err(ParsePuzzleError("Value string is too long".to_string()))
        }
        Ok(Puzzle([0;81]))
    }
}
