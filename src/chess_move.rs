// Module imports
use crate::pieces::Piece;
use crate::square::Square;
use std::fmt::{Display, Formatter, Result as FormatResult};
use crate::square::named::{E1, G1, C1, E8, G8, C8};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Flags {
    // Quiet move
    Quiet,
    // Double pawn push
    DoublePawnPush,
    // Castles
    KingSideCastle,
    QueenSideCastle,
    // Captures
    Capture,
    EnPassantCapture,
    // Promotions
    KnightPromotion = 8, // Set to 8 to align promotion captures with capture set bit
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    // Promoting Captures
    KnightPromotingCapture,
    BishopPromotingCapture,
    RookPromotingCapture,
    QueenPromotingCapture = 15,
}

impl Flags {
    pub fn captures(&self) -> bool {
        *self as u8 & Flags::Capture as u8 != 0
    }
    pub fn promotion(&self) -> Piece {
        match *self {
            Flags::KnightPromotion | Flags::KnightPromotingCapture => Piece::Knight,
            Flags::BishopPromotion | Flags::BishopPromotingCapture => Piece::Bishop,
            Flags::RookPromotion | Flags::RookPromotingCapture => Piece::Rook,
            Flags::QueenPromotion | Flags::QueenPromotingCapture=> Piece::Queen,
            _ => Piece::None
        }
    }
    pub(crate) fn add_capture(self) -> Self {
        match self {
            Flags::Quiet | Flags::Capture => Flags::Capture,
            Flags::DoublePawnPush   => panic!("cannot add capture to double pawn push"),
            Flags::KingSideCastle   => panic!("cannot add capture to king side castle"),
            Flags::QueenSideCastle  => panic!("cannot add capture to queen side castle"),
            Flags::EnPassantCapture => Flags::EnPassantCapture,
            Flags::KnightPromotion  | Flags::KnightPromotingCapture => Flags::KnightPromotingCapture,
            Flags::BishopPromotion  | Flags::BishopPromotingCapture => Flags::BishopPromotingCapture,
            Flags::RookPromotion    | Flags::RookPromotingCapture   => Flags::RookPromotingCapture,
            Flags::QueenPromotion   | Flags::QueenPromotingCapture  => Flags::QueenPromotingCapture,
        }
    }
    pub(crate) fn remove_capture(&self) -> Self {
        match self {
            Flags::Quiet | Flags::Capture | Flags::EnPassantCapture => Flags::Quiet,
            Flags::DoublePawnPush => Flags::DoublePawnPush,
            Flags::KingSideCastle => Flags::KingSideCastle,
            Flags::QueenSideCastle => Flags::QueenSideCastle,
            Flags::KnightPromotion | Flags::KnightPromotingCapture => Flags::KnightPromotion,
            Flags::BishopPromotion | Flags::BishopPromotingCapture => Flags::BishopPromotion,
            Flags::RookPromotion | Flags::RookPromotingCapture => Flags::RookPromotion,
            Flags::QueenPromotion | Flags::QueenPromotingCapture => Flags::QueenPromotion,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ChessMove(pub Square, pub Square, pub Flags);


impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let ChessMove(from, to, flags) = self;
        match *flags {
            Flags::Quiet | Flags::DoublePawnPush => write!(f, "{}{}", from, to),
            // NOTE: Potentially handle differently (chess960 or o-o notation are concerns worth investigating)
            Flags::KingSideCastle | Flags::QueenSideCastle => write!(f, "{}{}", from, to),
            Flags::Capture | Flags::EnPassantCapture => write!(f, "{}x{}", from, to),
            Flags::KnightPromotion => write!(f, "{}{}n", from, to),
            Flags::BishopPromotion => write!(f, "{}{}b", from, to),
            Flags::RookPromotion => write!(f, "{}{}r", from, to),
            Flags::QueenPromotion => write!(f, "{}{}q", from, to),
            Flags::KnightPromotingCapture => write!(f, "{}x{}n", from, to),
            Flags::BishopPromotingCapture => write!(f, "{}x{}b", from, to),
            Flags::RookPromotingCapture => write!(f, "{}x{}r", from, to),
            Flags::QueenPromotingCapture => write!(f, "{}x{}q", from, to),
        }
    }
}

impl ChessMove {
    pub const WHITE_KING_CASTLE: ChessMove = ChessMove(E1, G1, Flags::KingSideCastle);
    pub const WHITE_QUEEN_CASTLE: ChessMove = ChessMove(E1, C1, Flags::QueenSideCastle);
    pub const BLACK_KING_CASTLE: ChessMove = ChessMove(E8, G8, Flags::KingSideCastle);
    pub const BLACK_QUEEN_CASTLE: ChessMove = ChessMove(E8, C8, Flags::QueenSideCastle);

    pub fn from(&self) -> Square {
        self.0
    }
    pub fn to(&self) -> Square {
        self.1
    }
    pub fn flags(&self) -> Flags {
        self.2
    }
}
