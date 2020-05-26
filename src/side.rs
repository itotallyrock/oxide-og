
use std::convert::TryFrom;
use super::errors;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Side {
    White,
    Black,
}


impl TryFrom<String> for Side {
    type Error = errors::InvalidSideError;
    /// Case sensitive (all lowercase) side parsing (w, b, white, black) strings are parsed to Side
    fn try_from(side_string: String) -> Result<Self, Self::Error> {
        match side_string.to_lowercase().as_str() {
            "w" | "white" => Ok(Side::White),
            "b" | "black" => Ok(Side::Black),
            _ => Err(errors::InvalidSideError)
        }
    }
}
