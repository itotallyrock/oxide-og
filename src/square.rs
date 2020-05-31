
use std::convert::TryFrom;

use super::errors;
use std::collections::VecDeque;
use std::fmt::Formatter;
use core::fmt;
use std::ops::{Add, Sub};

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
    use super::Square;

    // Rank 1
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    // Rank 2
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);
    // Rank 3
    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);
    // Rank 4
    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);
    // Rank 5
    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);
    // Rank 6
    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);
    // Rank 7
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);
    // Rank 8
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);
}

/// Convert a unsigned 64 bit mask to an iterator over each set (1 at that offset) square
pub fn mask_to_square_iter(mask: u64) -> impl Iterator<Item=Square> {
    let mut squares: VecDeque<Square> = VecDeque::with_capacity(mask.count_ones() as usize);
    let mut mutable_mask = mask.clone();

    while mutable_mask > 0 {
        // Get index of first 1
        let square_offset = mutable_mask.trailing_zeros() as u8;
        // Create target square
        let square = Square(square_offset);
        // Unset bit
        mutable_mask ^= square.mask();
        // Add square to squares
        squares.push_back(square);
    }

    squares.into_iter()
}

/*
pub mod old {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, Debug)]
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

    impl std::fmt::Display for Square {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}{}", FILE_CHARS[self.x() as usize], RANK_CHARS[self.y() as usize])
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

    impl From<u8> for Square {
        fn from(offset: u8) -> Self {
            if offset > 63 {
                panic!("Offset must be between 0 and 63, received '{}'", offset)
            } else {
                Square { offset }
            }
        }
    }
}*/

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Square(pub u8);

impl Square {
    #[deprecated(note = "Square is now just a trait on the offset itself, offset is no longer needed")]
    #[inline]
    pub fn offset(self) -> u8 {
        let Square(offset) = self;
        offset
    }
    #[inline]
    pub fn x(self) -> u8 {
        let Square(offset) = self;
        offset % 8u8
    }
    #[inline]
    pub fn y(self) -> u8 {
        let Square(offset) = self;
        offset / 8u8
    }
    #[inline]
    pub fn mask(self) -> u64 {
        let Square(offset) = self;
        1u64 << offset as u64
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Square(offset) = self.clone();
        if offset > 63 {
            panic!("Attempting to convert invalid move into string");
        }
        write!(f, "{}{}", FILE_CHARS[self.x() as usize], RANK_CHARS[self.y() as usize])
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
        let file_index = FILE_CHARS.iter().position(|&c| c == file).ok_or(errors::SquareParseError::SquareFileError(file))? as u8;
        let rank_index = RANK_CHARS.iter().position(|&c| c == rank).ok_or(errors::SquareParseError::SquareRankError(rank))? as u8;

        Ok(Square(rank_index * 8 + file_index))
    }
}
impl Add<u8> for Square {
    type Output = Square;

    #[inline]
    fn add(self, rhs: u8) -> Self::Output {
        let Square(offset) = self;
        Square(offset + rhs)
    }
}
impl Sub<u8> for Square {
    type Output = Square;

    #[inline]
    fn sub(self, rhs: u8) -> Self::Output {
        let Square(offset) = self;
        Square(offset - rhs)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_works() {
        // Bottom left to top right diagonal
        assert_eq!(named::A1.x(), 0u8);
        assert_eq!(named::B2.x(), 1u8);
        assert_eq!(named::C3.x(), 2u8);
        assert_eq!(named::D4.x(), 3u8);
        assert_eq!(named::E5.x(), 4u8);
        assert_eq!(named::F6.x(), 5u8);
        assert_eq!(named::G7.x(), 6u8);
        assert_eq!(named::H8.x(), 7u8);
        // Random squares
        assert_eq!(Square(61).x(), 5u8);
        assert_eq!(Square(38).x(), 6u8);
        assert_eq!(Square(40).x(), 0u8);
        assert_eq!(Square(34).x(), 2u8);
        assert_eq!(Square(20).x(), 4u8);
        assert_eq!(Square(6).x(), 6u8);
    }

    #[test]
    fn y_works() {
        // Bottom left to top right diagonal
        assert_eq!(named::A1.y(), 0u8);
        assert_eq!(named::B2.y(), 1u8);
        assert_eq!(named::C3.y(), 2u8);
        assert_eq!(named::D4.y(), 3u8);
        assert_eq!(named::E5.y(), 4u8);
        assert_eq!(named::F6.y(), 5u8);
        assert_eq!(named::G7.y(), 6u8);
        assert_eq!(named::H8.y(), 7u8);
        // Random squares
        assert_eq!(Square(61).y(), 7u8);
        assert_eq!(Square(38).y(), 4u8);
        assert_eq!(Square(40).y(), 5u8);
        assert_eq!(Square(34).y(), 4u8);
        assert_eq!(Square(20).y(), 2u8);
        assert_eq!(Square(6).y(), 0u8);
    }

    #[test]
    fn mask_works() {
        assert_eq!(named::A1.mask(), 0x1u64);
        assert_eq!(named::B2.mask(), 0x200u64);
        assert_eq!(named::C3.mask(), 0x40000u64);
        assert_eq!(named::D4.mask(), 0x8000000u64);
        assert_eq!(named::E5.mask(), 0x1000000000u64);
        assert_eq!(named::F6.mask(), 0x200000000000u64);
        assert_eq!(named::G7.mask(), 0x40000000000000u64);
        assert_eq!(named::H8.mask(), 0x8000000000000000u64);
        assert_eq!(named::H1.mask(), 0x80u64);
        assert_eq!(named::A8.mask(), 0x100000000000000u64);
    }

    #[test]
    fn to_string_works() {
        assert_eq!(named::A2.to_string(), "a2");
        assert_eq!(named::A5.to_string(), "a5");
        assert_eq!(named::A2.to_string(), "a2");
        assert_eq!(named::B3.to_string(), "b3");
        assert_eq!(named::E4.to_string(), "e4");
        assert_eq!(named::F8.to_string(), "f8");
        assert_eq!(named::H1.to_string(), "h1");
        assert_eq!(named::A8.to_string(), "a8");
    }

    #[test]
    #[should_panic]
    fn offboard_to_string_panics() {
        Square(64).to_string();
    }

    // TODO: Test TryFrom<String>
}