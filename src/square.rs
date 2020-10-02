
// Local imports
use named::*;
// Std imports
use std::ops::{Add, Sub};
use std::fmt::{Display, Result as FormatResult, Formatter};

// Square char representation
const FILE_CHARS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

// Array of all valid squares (for easily iterating through)
pub const SQUARES: [Square; 64] = [
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
];

// Bitboard masks
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

    // Whole board
    pub const ALL: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const NONE: u64 = 0x0;

    // Inverted files for edge piece filtering
    pub const NOT_H_FILE: u64 = !H_FILE;
    pub const NOT_A_FILE: u64 = !A_FILE;
    pub const NOT_G_OR_H_FILE: u64 = !(G_FILE | H_FILE);
    pub const NOT_A_OR_B_FILE: u64 = !(A_FILE | B_FILE);

    #[inline]
    pub fn file_for_x(x: u8) -> u64 {
        match x {
            0 => A_FILE,
            1 => B_FILE,
            2 => C_FILE,
            3 => D_FILE,
            4 => E_FILE,
            5 => F_FILE,
            6 => G_FILE,
            7 => H_FILE,
            _ => 0,
        }
    }

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

// Every possible square with its name, useful for using names instead of offsets
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

// Square type
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Square(u8);

impl Square {
    // How many possible squares
    pub const COUNT: usize = 64;
    // Create a square safely, makes sure the offset is valid
    #[inline]
    pub fn new(offset: u8) -> Self {
        if offset > 63 {
            panic!("invalid offset {} is not < 64", offset);
        }

        Self(offset)
    }
    // Get the zero-based (A File=0) x offset
    #[inline]
    pub fn x(self) -> u8 {
        self.0 % 8u8
    }
    // Get the zero-based (Rank 1=0) y offset
    #[inline]
    pub fn y(self) -> u8 {
        self.0 / 8u8
    }
    // Get a bit mask for only this square
    #[inline]
    pub fn mask(self) -> u64 {
        1u64 << self.0 as u64
    }
    // Get the offset of the square
    #[inline]
    pub const fn offset(self) -> u8 {
        self.0
    }
}

impl Display for Square {
    // UCI Square format using file + rank
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}{}", FILE_CHARS[self.x() as usize], RANK_CHARS[self.y() as usize])
    }
}

impl Add<i8> for Square {
    type Output = Self;

    // Add a square and a value for new square
    #[inline]
    fn add(self, rhs: i8) -> Self::Output {
        let sum = Square((self.0 as i8 + rhs) as u8);
        debug_assert!(sum.0 < 64, "Overflow while attempting to add to a square");

        sum
    }
}

impl Sub<i8> for Square {
    type Output = Self;

    // Subtract a square and a value for a new square
    #[inline]
    fn sub(self, rhs: i8) -> Self::Output {
        let difference = Square((self.0 as i8 - rhs) as u8);
        debug_assert!(difference.0 < 64, "Underflow while attempting to subtract from a square: {:?}", difference);

        difference
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
    fn file_for_x_works() {
        assert_eq!(masks::file_for_x(0), masks::A_FILE);
        assert_eq!(masks::file_for_x(1), masks::B_FILE);
        assert_eq!(masks::file_for_x(2), masks::C_FILE);
        assert_eq!(masks::file_for_x(3), masks::D_FILE);
        assert_eq!(masks::file_for_x(4), masks::E_FILE);
        assert_eq!(masks::file_for_x(5), masks::F_FILE);
        assert_eq!(masks::file_for_x(6), masks::G_FILE);
        assert_eq!(masks::file_for_x(7), masks::H_FILE);
    }
}
