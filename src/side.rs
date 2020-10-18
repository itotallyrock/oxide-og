
// Std imports
use std::fmt::{Display, Formatter, Result as FormatResult};

// Side struct (true for black, false for white)
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Side(pub(crate) bool);

impl Side {
    // Side consts (used to represent white and black)
    pub const WHITE: Side = Side(false);
    pub const BLACK: Side = Side(true);
    // Since only 2 sides exist use
    pub const COUNT: usize = 2;
    #[inline]
    pub const fn opposite(&self) -> Self {
        Self(!self.0)
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        if *self == Side::WHITE {
            write!(f, "white")
        } else {
            write!(f, "black")
        }
    }
}

impl const From<Side> for usize {
    #[inline]
    fn from(side: Side) -> Self {
        side.0 as Self
    }
}

impl From<Side> for char {
    #[inline]
    fn from(side: Side) -> Self {
        if side == Side::WHITE {
            'w'
        } else {
            'b'
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn opposite_works() {
        assert_eq!(Side::WHITE.opposite(), Side::BLACK);
        assert_eq!(Side::BLACK.opposite(), Side::WHITE);
    }
}