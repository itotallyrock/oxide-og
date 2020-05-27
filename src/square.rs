
use std::convert::TryFrom;

use super::errors;

pub const FILE_CHARS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct Square {
    pub offset: u8,
}

impl Square {
    pub fn x(&self) -> u8 {
        self.offset % 8
    }
    pub fn y(&self) -> u8 {
        self.offset / 8
    }
    pub fn rank(&self) -> char {
        RANK_CHARS[self.y() as usize]
    }
    pub fn file(&self) -> char {
        FILE_CHARS[self.x() as usize]
    }
    pub fn mask(&self) -> u64 {
        1u64 << self.offset as u64
    }
}

impl ToString for Square {
    fn to_string(&self) -> String {
        let mut string = String::with_capacity(2);
        string.push(FILE_CHARS[self.x() as usize]);
        string.push(RANK_CHARS[self.y() as usize]);

        string
    }
}

impl TryFrom<String> for Square {
    type Error = errors::SquareParseError;
    fn try_from(file_rank: String) -> Result<Self, Self::Error> {
        let file_string_copy = file_rank.clone();
        let mut file_rank_chars = file_string_copy.chars();
        let err = errors::SquareParseError::SquareLengthError(file_rank.clone());
        let file = file_rank_chars.next().ok_or(err.clone())?;
        let rank = file_rank_chars.next().ok_or(err.clone())?;
        // If we took 2 chars but there is more left error
        if file_rank_chars.next().is_some() {
            return Err(err.clone());
        }
        let file_index = FILE_CHARS.iter().position(|&c| c == file);
        let rank_index = RANK_CHARS.iter().position(|&c| c == rank);

        if file_index.is_none() {
            let err = errors::SquareParseError::SquareFileError(file);
            return Err(err);
        }
        if rank_index.is_none() {
            let err = errors::SquareParseError::SquareRankError(rank);
            return Err(err);
        }

        Ok(Square {
            offset: (rank_index.unwrap() as u8) * 8 + (file_index.unwrap() as u8),
        })
    }
}

impl TryFrom<u8> for Square {
    type Error = String;
    fn try_from(offset: u8) -> Result<Self, Self::Error> {
        if offset > 63 {
            Err(format!("Offset must be between 0 and 63, received '{}'", offset))
        } else {
            Ok(Square { offset })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_offset_works() {
        for offset in 0..64 {
            let square = Square::try_from(offset).unwrap();
            assert_eq!(square.offset, offset, "Failed to set square offset from offset");
        }
    }

    #[test]
    fn from_bad_offset_gives_error() {
        assert!(Square::try_from(65).is_err(), "Failed to return error on invalid square offset");
    }
}
