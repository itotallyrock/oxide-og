// Module imports
use crate::pieces::Piece;
use crate::square::Square;
use std::fmt::{Display, Formatter, Result as FormatResult};

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
            Flags::KnightPromotion => write!(f, "{}{}k", to, from),
            Flags::BishopPromotion => write!(f, "{}{}b", to, from),
            Flags::RookPromotion => write!(f, "{}{}r", to, from),
            Flags::QueenPromotion => write!(f, "{}{}q", to, from),
            Flags::KnightPromotingCapture => write!(f, "{}x{}k", to, from),
            Flags::BishopPromotingCapture => write!(f, "{}x{}b", to, from),
            Flags::RookPromotingCapture => write!(f, "{}x{}r", to, from),
            Flags::QueenPromotingCapture => write!(f, "{}x{}q", to, from),
        }
    }
}
