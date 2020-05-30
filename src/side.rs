
use std::convert::TryFrom;
use super::errors;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn opposite(&self) -> Self {
        match self {
            Side::White => Side::Black,
            _ => Side::White,
        }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Side::White => write!(f, "white"),
            Side::Black => write!(f, "black"),
        }
    }
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

impl From<Side> for char {
    fn from(side: Side) -> Self {
        match side {
            Side::White => 'w',
            Side::Black => 'b',
        }
    }
}

