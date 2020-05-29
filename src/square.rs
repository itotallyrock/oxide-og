
use std::convert::TryFrom;

use super::errors;

pub const FILE_CHARS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

pub mod masks {
    // Files
    pub const A_FILE: u64 = 0x0101010101010101;
    pub const B_FILE: u64 = 0x0202020202020202;
    pub const C_FILE: u64 = 0x0404040404040404;
    pub const D_FILE: u64 = 0x0808080808080808;
    pub const H_FILE: u64 = 0x8080808080808080;
    pub const G_FILE: u64 = 0x4040404040404040;
    pub const F_FILE: u64 = 0x2020202020202020;
    pub const E_FILE: u64 = 0x1010101010101010;
    // Ranks
    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_2: u64 = 0x000000000000FF00;
    pub const RANK_3: u64 = 0x0000000000FF0000;
    pub const RANK_4: u64 = 0x00000000FF000000;
    pub const RANK_5: u64 = 0x000000FF00000000;
    pub const RANK_6: u64 = 0x0000FF0000000000;
    pub const RANK_7: u64 = 0x00FF000000000000;
    pub const RANK_8: u64 = 0xFF00000000000000;

    // Neighbor files
    pub mod neighbors {
        pub const A_FILE: u64 = super::B_FILE;
        pub const B_FILE: u64 = super::A_FILE | super::C_FILE;
        pub const C_FILE: u64 = super::B_FILE | super::D_FILE;
        pub const D_FILE: u64 = super::C_FILE | super::E_FILE;
        pub const E_FILE: u64 = super::D_FILE | super::F_FILE;
        pub const F_FILE: u64 = super::E_FILE | super::G_FILE;
        pub const G_FILE: u64 = super::F_FILE | super::H_FILE;
        pub const H_FILE: u64 = super::G_FILE;
    }
}

pub mod named {
    // Rank 1
    pub const A1: u8 = 0;
    pub const B1: u8 = 1;
    pub const C1: u8 = 2;
    pub const D1: u8 = 3;
    pub const E1: u8 = 4;
    pub const F1: u8 = 5;
    pub const G1: u8 = 6;
    pub const H1: u8 = 7;
    // Rank 2
    pub const A2: u8 = 8;
    pub const B2: u8 = 9;
    pub const C2: u8 = 10;
    pub const D2: u8 = 11;
    pub const E2: u8 = 12;
    pub const F2: u8 = 13;
    pub const G2: u8 = 14;
    pub const H2: u8 = 15;
    // Rank 3
    pub const A3: u8 = 16;
    pub const B3: u8 = 17;
    pub const C3: u8 = 18;
    pub const D3: u8 = 19;
    pub const E3: u8 = 20;
    pub const F3: u8 = 21;
    pub const G3: u8 = 22;
    pub const H3: u8 = 23;
    // Rank 4
    pub const A4: u8 = 24;
    pub const B4: u8 = 25;
    pub const C4: u8 = 26;
    pub const D4: u8 = 27;
    pub const E4: u8 = 28;
    pub const F4: u8 = 29;
    pub const G4: u8 = 30;
    pub const H4: u8 = 31;
    // Rank 5
    pub const A5: u8 = 32;
    pub const B5: u8 = 33;
    pub const C5: u8 = 34;
    pub const D5: u8 = 35;
    pub const E5: u8 = 36;
    pub const F5: u8 = 37;
    pub const G5: u8 = 38;
    pub const H5: u8 = 39;
    // Rank 6
    pub const A6: u8 = 40;
    pub const B6: u8 = 41;
    pub const C6: u8 = 42;
    pub const D6: u8 = 43;
    pub const E6: u8 = 44;
    pub const F6: u8 = 45;
    pub const G6: u8 = 46;
    pub const H6: u8 = 47;
    // Rank 7
    pub const A7: u8 = 48;
    pub const B7: u8 = 49;
    pub const C7: u8 = 50;
    pub const D7: u8 = 51;
    pub const E7: u8 = 52;
    pub const F7: u8 = 53;
    pub const G7: u8 = 54;
    pub const H7: u8 = 55;
    // Rank 8
    pub const A8: u8 = 56;
    pub const B8: u8 = 57;
    pub const C8: u8 = 58;
    pub const D8: u8 = 59;
    pub const E8: u8 = 60;
    pub const F8: u8 = 61;
    pub const G8: u8 = 62;
    pub const H8: u8 = 63;
}

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
