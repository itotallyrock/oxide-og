
// Std imports
use std::fmt::{Display, Formatter, Result as FormatResult};

// This was an enum, but I've found that using a u8 and bit operations is faster and safer this way
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CastlePermissions(pub(crate) u8);

impl CastlePermissions {
    // All possible castles
    pub const NONE: CastlePermissions = CastlePermissions(0);
    pub const WHITE_KING: CastlePermissions = CastlePermissions(1);
    pub const WHITE_QUEEN: CastlePermissions = CastlePermissions(2);
    pub const WHITE_ALL: CastlePermissions = CastlePermissions(3);
    pub const BLACK_KING: CastlePermissions = CastlePermissions(4);
    pub const BOTH_KINGS: CastlePermissions = CastlePermissions(5);
    pub const WHITE_QUEEN_BLACK_KING: CastlePermissions = CastlePermissions(6);
    pub const WHITE_ALL_BLACK_KING: CastlePermissions = CastlePermissions(7);
    pub const BLACK_QUEEN: CastlePermissions = CastlePermissions(8);
    pub const WHITE_KING_BLACK_QUEEN: CastlePermissions = CastlePermissions(9);
    pub const BOTH_QUEENS: CastlePermissions = CastlePermissions(10);
    pub const WHITE_ALL_BLACK_QUEEN: CastlePermissions = CastlePermissions(11);
    pub const BLACK_ALL: CastlePermissions = CastlePermissions(12);
    pub const BLACK_ALL_WHITE_KING: CastlePermissions = CastlePermissions(13);
    pub const BLACK_ALL_WHITE_QUEEN: CastlePermissions = CastlePermissions(14);
    pub const ALL: CastlePermissions = CastlePermissions(15);
    pub const COUNT: usize = 16;

    pub fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }
    pub fn intersects(&self, other: Self) -> bool {
        self.0 & other.0 > 0
    }
}

impl Default for CastlePermissions {
    fn default() -> Self {
        CastlePermissions::NONE
    }
}

impl Display for CastlePermissions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match *self {
            CastlePermissions::WHITE_KING => write!(f, "K"),
            CastlePermissions::WHITE_QUEEN => write!(f, "Q"),
            CastlePermissions::WHITE_ALL => write!(f, "KQ"),
            CastlePermissions::BLACK_KING => write!(f, "k"),
            CastlePermissions::BOTH_KINGS => write!(f, "Kk"),
            CastlePermissions::WHITE_QUEEN_BLACK_KING => write!(f, "Qk"),
            CastlePermissions::WHITE_ALL_BLACK_KING => write!(f, "KQk"),
            CastlePermissions::BLACK_QUEEN => write!(f, "q"),
            CastlePermissions::WHITE_KING_BLACK_QUEEN => write!(f, "Kq"),
            CastlePermissions::BOTH_QUEENS => write!(f, "Qq"),
            CastlePermissions::WHITE_ALL_BLACK_QUEEN => write!(f, "KQq"),
            CastlePermissions::BLACK_ALL => write!(f, "kq"),
            CastlePermissions::BLACK_ALL_WHITE_KING => write!(f, "Kkq"),
            CastlePermissions::BLACK_ALL_WHITE_QUEEN => write!(f, "Qkq"),
            CastlePermissions::ALL => write!(f, "KQkq"),
            CastlePermissions::NONE | _ => write!(f, "-"),
        }
    }
}

// TODO: Unit test contains, remove, insert, and intersects works exhaustively
