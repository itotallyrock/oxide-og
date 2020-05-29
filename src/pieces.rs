

use super::side::Side;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum ColoredPiece {
    WPawn,
    WBishop,
    WRook,
    WKing,
    WKnight,
    WQueen,
    BPawn,
    BBishop,
    BRook,
    BKing,
    BKnight,
    BQueen,
    None,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
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

impl From<ColoredPiece> for usize {
    fn from(colored_piece: ColoredPiece) -> usize {
        match colored_piece {
            ColoredPiece::WPawn => 0,
            ColoredPiece::WBishop => 1,
            ColoredPiece::WRook => 2,
            ColoredPiece::WKing => 3,
            ColoredPiece::WKnight => 4,
            ColoredPiece::WQueen => 5,
            ColoredPiece::BPawn => 6,
            ColoredPiece::BBishop => 7,
            ColoredPiece::BRook => 8,
            ColoredPiece::BKing => 9,
            ColoredPiece::BKnight => 10,
            ColoredPiece::BQueen => 11,
            ColoredPiece::None => 12,
        }
    }
}

impl Piece {
    /// Color the regular piece so it belongs to a side
    pub fn color(&self, side: Side) -> ColoredPiece {
        match self {
            Piece::Pawn =>
                if side == Side::White { ColoredPiece::WPawn } else { ColoredPiece::BPawn },
            Piece::Bishop =>
                if side == Side::White { ColoredPiece::WBishop } else { ColoredPiece::BBishop },
            Piece::Rook =>
                if side == Side::White { ColoredPiece::WRook } else { ColoredPiece::BRook },
            Piece::King =>
                if side == Side::White { ColoredPiece::WKing } else { ColoredPiece::BKing },
            Piece::Knight =>
                if side == Side::White { ColoredPiece::WKnight } else { ColoredPiece::BKnight },
            Piece::Queen =>
                if side == Side::White { ColoredPiece::WQueen } else { ColoredPiece::BQueen },
            Piece::None => ColoredPiece::None
        }
    }
}

impl ColoredPiece {
    /// Remove ownership of a piece getting just its type
    pub fn uncolor(&self) -> Piece {
        match self {
            ColoredPiece::WPawn | ColoredPiece::BPawn => Piece::Pawn,
            ColoredPiece::WBishop | ColoredPiece::BBishop => Piece::Bishop,
            ColoredPiece::WRook | ColoredPiece::BRook => Piece::Rook,
            ColoredPiece::WKing | ColoredPiece::BKing => Piece::King,
            ColoredPiece::WKnight | ColoredPiece::BKnight => Piece::Knight,
            ColoredPiece::WQueen | ColoredPiece::BQueen => Piece::Queen,
            ColoredPiece::None => Piece::None
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
            _ => ColoredPiece::None,
        }
    }
}

impl From<char> for Piece {
    fn from(c: char) -> Piece {
        ColoredPiece::from(c).uncolor()
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
            Piece::None => ' '
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
            ColoredPiece::None => ' '
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
        assert_eq!(ColoredPiece::None.uncolor(), Piece::None, "Failed to uncolor none");
    }

    #[test]
    fn black_uncolor_works() {
        assert_eq!(ColoredPiece::BPawn.uncolor(), Piece::Pawn, "Black pawn failed to uncolor");
        assert_eq!(ColoredPiece::BBishop.uncolor(), Piece::Bishop, "Black bishop failed to uncolor");
        assert_eq!(ColoredPiece::BRook.uncolor(), Piece::Rook, "Black rook failed to uncolor");
        assert_eq!(ColoredPiece::BKing.uncolor(), Piece::King, "Black king failed to uncolor");
        assert_eq!(ColoredPiece::BKnight.uncolor(), Piece::Knight, "Black knight failed to uncolor");
        assert_eq!(ColoredPiece::BQueen.uncolor(), Piece::Queen, "Black queen failed to uncolor");
        assert_eq!(ColoredPiece::None.uncolor(), Piece::None, "Failed to uncolor none");
    }

    #[test]
    fn white_color_works() {
        assert_eq!(Piece::Pawn.color(Side::White), ColoredPiece::WPawn, "Pawn failed to color white");
        assert_eq!(Piece::Bishop.color(Side::White), ColoredPiece::WBishop, "Bishop failed to color white");
        assert_eq!(Piece::Rook.color(Side::White), ColoredPiece::WRook, "Rook failed to color white");
        assert_eq!(Piece::King.color(Side::White), ColoredPiece::WKing, "King failed to color white");
        assert_eq!(Piece::Knight.color(Side::White), ColoredPiece::WKnight, "Knight failed to color white");
        assert_eq!(Piece::Queen.color(Side::White), ColoredPiece::WQueen, "Queen failed to color white");
        assert_eq!(Piece::None.color(Side::White), ColoredPiece::None, "Failed to uncolor none");
    }

    #[test]
    fn black_color_works() {
        assert_eq!(Piece::Pawn.color(Side::Black), ColoredPiece::BPawn, "Pawn failed to color black");
        assert_eq!(Piece::Bishop.color(Side::Black), ColoredPiece::BBishop, "Bishop failed to color black");
        assert_eq!(Piece::Rook.color(Side::Black), ColoredPiece::BRook, "Rook failed to color black");
        assert_eq!(Piece::King.color(Side::Black), ColoredPiece::BKing, "King failed to color black");
        assert_eq!(Piece::Knight.color(Side::Black), ColoredPiece::BKnight, "Knight failed to color black");
        assert_eq!(Piece::Queen.color(Side::Black), ColoredPiece::BQueen, "Queen failed to color black");
        assert_eq!(Piece::None.color(Side::Black), ColoredPiece::None, "Failed to uncolor none");
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
