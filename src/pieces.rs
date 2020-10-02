
// Local imports
use crate::side::Side;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ColoredPiece {
    WPawn,
    BPawn,
    WBishop,
    BBishop,
    WRook,
    BRook,
    WKing,
    BKing,
    WKnight,
    BKnight,
    WQueen,
    BQueen,
    None,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Piece {
    Pawn,
    Bishop,
    Rook,
    King,
    Knight,
    Queen,
    None,
}

impl Default for Piece {
    fn default() -> Self { Piece::None }
}

impl Default for ColoredPiece {
    fn default() -> Self { ColoredPiece::None }
}

impl Piece {
    pub const COUNT: usize = 6;
    /// Color the regular piece so it belongs to a side
    pub fn color(self, side: Side) -> ColoredPiece {
        match self {
            Piece::Pawn if side == Side::WHITE => ColoredPiece::WPawn,
            Piece::Pawn if side == Side::BLACK => ColoredPiece::BPawn,
            Piece::Bishop if side == Side::WHITE => ColoredPiece::WBishop,
            Piece::Bishop if side == Side::BLACK => ColoredPiece::BBishop,
            Piece::Rook if side == Side::WHITE => ColoredPiece::WRook,
            Piece::Rook if side == Side::BLACK => ColoredPiece::BRook,
            Piece::King if side == Side::WHITE => ColoredPiece::WKing,
            Piece::King if side == Side::BLACK => ColoredPiece::BKing,
            Piece::Knight if side == Side::WHITE => ColoredPiece::WKnight,
            Piece::Knight if side == Side::BLACK => ColoredPiece::BKnight,
            Piece::Queen if side == Side::WHITE => ColoredPiece::WQueen,
            Piece::Queen if side == Side::BLACK => ColoredPiece::BQueen,
            Piece::None => panic!("Cannot color none piece"),
            _ => panic!("Unknown side {} for coloring {} piece", side, self.to_ascii()),
        }
    }
}

impl ColoredPiece {
    pub const COUNT: usize = 12;
    /// Get the side of a colored piece
    pub fn side(self) -> Side {
        match self {
            ColoredPiece::WPawn | ColoredPiece::WBishop | ColoredPiece::WRook | ColoredPiece::WKing | ColoredPiece::WKnight | ColoredPiece::WQueen => Side::WHITE,
            ColoredPiece::BPawn | ColoredPiece::BBishop | ColoredPiece::BRook | ColoredPiece::BKing | ColoredPiece::BKnight | ColoredPiece::BQueen => Side::BLACK,
            ColoredPiece::None => panic!("Attempting to get side of none colored piece")
        }
    }
    /// Remove ownership of a piece getting just its type
    pub fn uncolor(self) -> Piece {
        match self {
            ColoredPiece::WPawn | ColoredPiece::BPawn => Piece::Pawn,
            ColoredPiece::WBishop | ColoredPiece::BBishop => Piece::Bishop,
            ColoredPiece::WRook | ColoredPiece::BRook => Piece::Rook,
            ColoredPiece::WKing | ColoredPiece::BKing => Piece::King,
            ColoredPiece::WKnight | ColoredPiece::BKnight => Piece::Knight,
            ColoredPiece::WQueen | ColoredPiece::BQueen => Piece::Queen,
            ColoredPiece::None => panic!("Attempting to uncolor None piece"),
        }
    }
}

impl From<char> for ColoredPiece {
    fn from(c: char) -> ColoredPiece {
        match c {
            'P' => ColoredPiece::WPawn,
            'B' => ColoredPiece::WBishop,
            'R' => ColoredPiece::WRook,
            'K' => ColoredPiece::WKing,
            'N' => ColoredPiece::WKnight,
            'Q' => ColoredPiece::WQueen,
            'p' => ColoredPiece::BPawn,
            'b' => ColoredPiece::BBishop,
            'r' => ColoredPiece::BRook,
            'k' => ColoredPiece::BKing,
            'n' => ColoredPiece::BKnight,
            'q' => ColoredPiece::BQueen,
            _ => {
                debug_assert!(false, "Invalid colored piece '{}', cannot convert", c);
                ColoredPiece::None
            },
        }
    }
}

impl From<char> for Piece {
    fn from(c: char) -> Piece {
        match c {
            'P' | 'p' => Piece::Pawn,
            'B' | 'b' => Piece::Bishop,
            'R' | 'r' => Piece::Rook,
            'K' | 'k' => Piece::King,
            'N' | 'n' => Piece::Knight,
            'Q' | 'q' => Piece::Queen,
            _ => {
                panic!("Invalid piece character '{}'", c);
            },
        }
    }
}

/// Used to get character representation of a piece
pub trait PieceRepr {
    /// Get standard 256 ascii character
    fn to_ascii(&self) -> char;
    /// Get the full unicode representation (falls back to ascii)
    fn to_unicode(&self) -> char {
        self.to_ascii()
    }
}

impl PieceRepr for Piece {
    /// Gets the ascii character for the piece (always lower case)
    fn to_ascii(&self) -> char {
        match self {
            Piece::Pawn => 'p',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::King => 'k',
            Piece::Knight => 'n',
            Piece::Queen => 'q',
            Piece::None => '.'
        }
    }
    /// Get the [unicode chess symbols](https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode) (always black)
    fn to_unicode(&self) -> char {
        match self {
            Piece::Pawn => '\u{265F}',
            Piece::Bishop => '\u{265D}',
            Piece::Rook => '\u{265C}',
            Piece::King => '\u{265A}',
            Piece::Knight => '\u{265E}',
            Piece::Queen => '\u{265B}',
            Piece::None => ' '
        }
    }
}

impl PieceRepr for ColoredPiece {
    /// Gets the ascii character for the piece (white uppercase)
    fn to_ascii(&self) -> char {
        match self {
            ColoredPiece::WPawn => 'P',
            ColoredPiece::BPawn => 'p',
            ColoredPiece::WBishop => 'B',
            ColoredPiece::BBishop => 'b',
            ColoredPiece::WRook => 'R',
            ColoredPiece::BRook => 'r',
            ColoredPiece::WKing => 'K',
            ColoredPiece::BKing => 'k',
            ColoredPiece::WKnight => 'N',
            ColoredPiece::BKnight => 'n',
            ColoredPiece::WQueen => 'Q',
            ColoredPiece::BQueen => 'q',
            ColoredPiece::None => '.'
        }
    }
    /// Get the [unicode chess symbols](https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode)
    fn to_unicode(&self) -> char {
        match self {
            ColoredPiece::WPawn => '\u{2659}',
            ColoredPiece::BPawn => '\u{265F}',
            ColoredPiece::WBishop => '\u{2657}',
            ColoredPiece::BBishop => '\u{265D}',
            ColoredPiece::WRook => '\u{2656}',
            ColoredPiece::BRook => '\u{265C}',
            ColoredPiece::WKing => '\u{2654}',
            ColoredPiece::BKing => '\u{265A}',
            ColoredPiece::WKnight => '\u{2658}',
            ColoredPiece::BKnight => '\u{265E}',
            ColoredPiece::WQueen => '\u{2655}',
            ColoredPiece::BQueen => '\u{265B}',
            ColoredPiece::None => ' '
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_uncolor_works() {
        assert_eq!(ColoredPiece::WPawn.uncolor(), Piece::Pawn, "White pawn failed to uncolor");
        assert_eq!(ColoredPiece::WBishop.uncolor(), Piece::Bishop, "White bishop failed to uncolor");
        assert_eq!(ColoredPiece::WRook.uncolor(), Piece::Rook, "White rook failed to uncolor");
        assert_eq!(ColoredPiece::WKing.uncolor(), Piece::King, "White king failed to uncolor");
        assert_eq!(ColoredPiece::WKnight.uncolor(), Piece::Knight, "White knight failed to uncolor");
        assert_eq!(ColoredPiece::WQueen.uncolor(), Piece::Queen, "White queen failed to uncolor");
    }

    #[test]
    fn black_uncolor_works() {
        assert_eq!(ColoredPiece::BPawn.uncolor(), Piece::Pawn, "Black pawn failed to uncolor");
        assert_eq!(ColoredPiece::BBishop.uncolor(), Piece::Bishop, "Black bishop failed to uncolor");
        assert_eq!(ColoredPiece::BRook.uncolor(), Piece::Rook, "Black rook failed to uncolor");
        assert_eq!(ColoredPiece::BKing.uncolor(), Piece::King, "Black king failed to uncolor");
        assert_eq!(ColoredPiece::BKnight.uncolor(), Piece::Knight, "Black knight failed to uncolor");
        assert_eq!(ColoredPiece::BQueen.uncolor(), Piece::Queen, "Black queen failed to uncolor");
    }

    #[should_panic]
    #[test]
    fn uncolor_none_panics() {
        ColoredPiece::None.uncolor();
    }

    #[test]
    fn white_color_works() {
        assert_eq!(Piece::Pawn.color(Side::WHITE), ColoredPiece::WPawn, "Pawn failed to color white");
        assert_eq!(Piece::Bishop.color(Side::WHITE), ColoredPiece::WBishop, "Bishop failed to color white");
        assert_eq!(Piece::Rook.color(Side::WHITE), ColoredPiece::WRook, "Rook failed to color white");
        assert_eq!(Piece::King.color(Side::WHITE), ColoredPiece::WKing, "King failed to color white");
        assert_eq!(Piece::Knight.color(Side::WHITE), ColoredPiece::WKnight, "Knight failed to color white");
        assert_eq!(Piece::Queen.color(Side::WHITE), ColoredPiece::WQueen, "Queen failed to color white");
    }

    #[test]
    fn black_color_works() {
        assert_eq!(Piece::Pawn.color(Side::BLACK), ColoredPiece::BPawn, "Pawn failed to color black");
        assert_eq!(Piece::Bishop.color(Side::BLACK), ColoredPiece::BBishop, "Bishop failed to color black");
        assert_eq!(Piece::Rook.color(Side::BLACK), ColoredPiece::BRook, "Rook failed to color black");
        assert_eq!(Piece::King.color(Side::BLACK), ColoredPiece::BKing, "King failed to color black");
        assert_eq!(Piece::Knight.color(Side::BLACK), ColoredPiece::BKnight, "Knight failed to color black");
        assert_eq!(Piece::Queen.color(Side::BLACK), ColoredPiece::BQueen, "Queen failed to color black");
    }

    #[should_panic]
    #[test]
    fn color_none_white_panics() {
        Piece::None.color(Side::WHITE);
    }

    #[should_panic]
    #[test]
    fn color_none_black_panics() {
        Piece::None.color(Side::BLACK);
    }

    #[test]
    fn white_from_works() {
        assert_eq!(ColoredPiece::from('P'), ColoredPiece::WPawn, "Failed to get white pawn from character");
        assert_eq!(ColoredPiece::from('B'), ColoredPiece::WBishop, "Failed to get white bishop from character");
        assert_eq!(ColoredPiece::from('R'), ColoredPiece::WRook, "Failed to get white rook from character");
        assert_eq!(ColoredPiece::from('K'), ColoredPiece::WKing, "Failed to get white king from character");
        assert_eq!(ColoredPiece::from('N'), ColoredPiece::WKnight, "Failed to get white knight from character");
        assert_eq!(ColoredPiece::from('Q'), ColoredPiece::WQueen, "Failed to get white queen from character");
    }

    #[test]
    fn black_from_works() {
        assert_eq!(ColoredPiece::from('p'), ColoredPiece::BPawn, "Failed to get black pawn from character");
        assert_eq!(ColoredPiece::from('b'), ColoredPiece::BBishop, "Failed to get black bishop from character");
        assert_eq!(ColoredPiece::from('r'), ColoredPiece::BRook, "Failed to get black rook from character");
        assert_eq!(ColoredPiece::from('k'), ColoredPiece::BKing, "Failed to get black king from character");
        assert_eq!(ColoredPiece::from('n'), ColoredPiece::BKnight, "Failed to get black knight from character");
        assert_eq!(ColoredPiece::from('q'), ColoredPiece::BQueen, "Failed to get black queen from character");
    }

    #[test]
    fn white_to_ascii_works() {
        assert_eq!(ColoredPiece::WPawn.to_ascii(), 'P', "Failed to get correct ascii representation for white pawn");
        assert_eq!(ColoredPiece::WBishop.to_ascii(), 'B', "Failed to get correct ascii representation for white bishop");
        assert_eq!(ColoredPiece::WRook.to_ascii(), 'R', "Failed to get correct ascii representation for white rook");
        assert_eq!(ColoredPiece::WKing.to_ascii(), 'K', "Failed to get correct ascii representation for white king");
        assert_eq!(ColoredPiece::WKnight.to_ascii(), 'N', "Failed to get correct ascii representation for white knight");
        assert_eq!(ColoredPiece::WQueen.to_ascii(), 'Q', "Failed to get correct ascii representation for white queen");
    }

    #[test]
    fn black_to_ascii_works() {
        assert_eq!(ColoredPiece::BPawn.to_ascii(), 'p', "Failed to get correct ascii representation for black pawn");
        assert_eq!(ColoredPiece::BBishop.to_ascii(), 'b', "Failed to get correct ascii representation for black bishop");
        assert_eq!(ColoredPiece::BRook.to_ascii(), 'r', "Failed to get correct ascii representation for black rook");
        assert_eq!(ColoredPiece::BKing.to_ascii(), 'k', "Failed to get correct ascii representation for black king");
        assert_eq!(ColoredPiece::BKnight.to_ascii(), 'n', "Failed to get correct ascii representation for black knight");
        assert_eq!(ColoredPiece::BQueen.to_ascii(), 'q', "Failed to get correct ascii representation for black queen");
    }
}

#[cfg(test)]
mod bench {
    // Local imports
    use super::*;

    // External test for benchmarking
    extern crate test;
    use test::Bencher;

    #[bench]
    fn color_bench(bencher: &mut Bencher) {
        const PIECES: [Piece; 6] = [Piece::Pawn, Piece::Bishop, Piece::King, Piece::Queen, Piece::Knight, Piece::Rook];
        bencher.iter(|| for piece in test::black_box(PIECES.iter()) {
            piece.color(test::black_box(Side::WHITE));
            piece.color(test::black_box(Side::BLACK));
        });
    }
}
