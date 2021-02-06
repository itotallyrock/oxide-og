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
    #[inline]
    pub fn for_promotion(piece: Piece) -> Self {
        match piece {
            Piece::Bishop => Flags::BishopPromotion,
            Piece::Rook => Flags::RookPromotion,
            Piece::Knight => Flags::KnightPromotion,
            Piece::Queen => Flags::QueenPromotion,
            Piece::Pawn | Piece::King | Piece::None => panic!("invalid promotion piece {}", piece),
        }
    }
    #[inline]
    pub fn captures(&self) -> bool {
        *self as u8 & Flags::Capture as u8 != 0
    }
    #[inline]
    pub fn promotion(&self) -> Piece {
        match *self {
            Flags::KnightPromotion | Flags::KnightPromotingCapture => Piece::Knight,
            Flags::BishopPromotion | Flags::BishopPromotingCapture => Piece::Bishop,
            Flags::RookPromotion | Flags::RookPromotingCapture => Piece::Rook,
            Flags::QueenPromotion | Flags::QueenPromotingCapture => Piece::Queen,
            _ => Piece::None
        }
    }
    #[inline]
    pub fn promotes(&self) -> bool {
        *self as u8 & Flags::KnightPromotion as u8 != 0
    }
    #[inline]
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
    #[inline]
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
pub struct ChessMove {
    pub(crate) from: Square,
    pub(crate) to: Square,
    pub(crate) flags: Flags,
}


impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let &ChessMove { from, to, flags } = self;
        match flags {
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
    pub const WHITE_KING_CASTLE: ChessMove = ChessMove { from: E1, to: G1, flags: Flags::KingSideCastle };
    pub const WHITE_QUEEN_CASTLE: ChessMove = ChessMove { from: E1, to: C1, flags: Flags::QueenSideCastle };
    pub const BLACK_KING_CASTLE: ChessMove = ChessMove { from: E8, to: G8, flags: Flags::KingSideCastle };
    pub const BLACK_QUEEN_CASTLE: ChessMove = ChessMove { from: E8, to: C8, flags: Flags::QueenSideCastle };

    pub fn from(&self) -> Square {
        self.from
    }
    pub fn to(&self) -> Square {
        self.to
    }
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn promotes_works() {
        assert!(!Flags::Quiet.promotes());
        assert!(!Flags::DoublePawnPush.promotes());
        assert!(!Flags::KingSideCastle.promotes());
        assert!(!Flags::QueenSideCastle.promotes());
        assert!(!Flags::Capture.promotes());
        assert!(!Flags::EnPassantCapture.promotes());
        assert!(Flags::KnightPromotion.promotes());
        assert!(Flags::BishopPromotion.promotes());
        assert!(Flags::RookPromotion.promotes());
        assert!(Flags::QueenPromotion.promotes());
        assert!(Flags::KnightPromotingCapture.promotes());
        assert!(Flags::BishopPromotingCapture.promotes());
        assert!(Flags::RookPromotingCapture.promotes());
        assert!(Flags::QueenPromotingCapture.promotes());
    }

    #[test]
    fn for_promotion_works() {
        assert_eq!(Flags::for_promotion(Piece::Knight), Flags::KnightPromotion);
        assert_eq!(Flags::for_promotion(Piece::Rook), Flags::RookPromotion);
        assert_eq!(Flags::for_promotion(Piece::Queen), Flags::QueenPromotion);
        assert_eq!(Flags::for_promotion(Piece::Bishop), Flags::BishopPromotion);
    }

    #[test]
    #[should_panic]
    fn for_promotion_panics_pawn_promotion() {
        Flags::for_promotion(Piece::Pawn);
    }

    #[test]
    #[should_panic]
    fn for_promotion_panics_king_promotion() {
        Flags::for_promotion(Piece::King);
    }

    #[test]
    fn captures_works() {
        assert!(Flags::EnPassantCapture.captures());
        assert!(Flags::BishopPromotingCapture.captures());
        assert!(Flags::KnightPromotingCapture.captures());
        assert!(Flags::QueenPromotingCapture.captures());
        assert!(Flags::RookPromotingCapture.captures());
        assert!(Flags::Capture.captures());
        // Non captures are false
        assert!(!Flags::BishopPromotion.captures());
        assert!(!Flags::KnightPromotion.captures());
        assert!(!Flags::QueenPromotion.captures());
        assert!(!Flags::RookPromotion.captures());
        assert!(!Flags::Quiet.captures());
        assert!(!Flags::QueenSideCastle.captures());
        assert!(!Flags::KingSideCastle.captures());
        assert!(!Flags::DoublePawnPush.captures());
    }

    #[test]
    fn promotion_works() {
        // Non promoting moves are none
        assert_eq!(Flags::QueenSideCastle.promotion(), Piece::None);
        assert_eq!(Flags::KingSideCastle.promotion(), Piece::None);
        assert_eq!(Flags::Quiet.promotion(), Piece::None);
        assert_eq!(Flags::Capture.promotion(), Piece::None);
        assert_eq!(Flags::EnPassantCapture.promotion(), Piece::None);
        assert_eq!(Flags::DoublePawnPush.promotion(), Piece::None);
        // Promotions are their pieces
        assert_eq!(Flags::BishopPromotingCapture.promotion(), Piece::Bishop);
        assert_eq!(Flags::KnightPromotingCapture.promotion(), Piece::Knight);
        assert_eq!(Flags::QueenPromotingCapture.promotion(), Piece::Queen);
        assert_eq!(Flags::RookPromotingCapture.promotion(), Piece::Rook);
        assert_eq!(Flags::BishopPromotion.promotion(), Piece::Bishop);
        assert_eq!(Flags::KnightPromotion.promotion(), Piece::Knight);
        assert_eq!(Flags::QueenPromotion.promotion(), Piece::Queen);
        assert_eq!(Flags::RookPromotion.promotion(), Piece::Rook);
    }

    #[test]
    fn add_capture_works() {
        // Non captures become their capture counter-parts
        assert_eq!(Flags::Quiet.add_capture(), Flags::Capture);
        assert_eq!(Flags::RookPromotion.add_capture(), Flags::RookPromotingCapture);
        assert_eq!(Flags::BishopPromotion.add_capture(), Flags::BishopPromotingCapture);
        assert_eq!(Flags::QueenPromotion.add_capture(), Flags::QueenPromotingCapture);
        assert_eq!(Flags::KnightPromotion.add_capture(), Flags::KnightPromotingCapture);
        // Captures keep their captures
        assert_eq!(Flags::Capture.add_capture(), Flags::Capture);
        assert_eq!(Flags::EnPassantCapture.add_capture(), Flags::EnPassantCapture);
        assert_eq!(Flags::RookPromotingCapture.add_capture(), Flags::RookPromotingCapture);
        assert_eq!(Flags::BishopPromotingCapture.add_capture(), Flags::BishopPromotingCapture);
        assert_eq!(Flags::QueenPromotingCapture.add_capture(), Flags::QueenPromotingCapture);
        assert_eq!(Flags::KnightPromotingCapture.add_capture(), Flags::KnightPromotingCapture);
    }

    #[test]
    #[should_panic]
    fn add_capture_double_pawn_push_panics() {
        Flags::DoublePawnPush.add_capture();
    }

    #[test]
    #[should_panic]
    fn add_capture_king_side_castle_panics() {
        Flags::KingSideCastle.add_capture();
    }

    #[test]
    #[should_panic]
    fn add_capture_queen_side_castle_panics() {
        Flags::QueenSideCastle.add_capture();
    }

    #[test]
    fn remove_capture_works() {
        // Non captures become remain the same
        assert_eq!(Flags::Quiet.remove_capture(), Flags::Quiet);
        assert_eq!(Flags::RookPromotion.remove_capture(), Flags::RookPromotion);
        assert_eq!(Flags::BishopPromotion.remove_capture(), Flags::BishopPromotion);
        assert_eq!(Flags::QueenPromotion.remove_capture(), Flags::QueenPromotion);
        assert_eq!(Flags::KnightPromotion.remove_capture(), Flags::KnightPromotion);
        assert_eq!(Flags::KingSideCastle.remove_capture(), Flags::KingSideCastle);
        assert_eq!(Flags::QueenSideCastle.remove_capture(), Flags::QueenSideCastle);
        assert_eq!(Flags::DoublePawnPush.remove_capture(), Flags::DoublePawnPush);
        // Captures lose their captures
        assert_eq!(Flags::Capture.remove_capture(), Flags::Quiet);
        assert_eq!(Flags::EnPassantCapture.remove_capture(), Flags::Quiet);
        assert_eq!(Flags::RookPromotingCapture.remove_capture(), Flags::RookPromotion);
        assert_eq!(Flags::BishopPromotingCapture.remove_capture(), Flags::BishopPromotion);
        assert_eq!(Flags::QueenPromotingCapture.remove_capture(), Flags::QueenPromotion);
        assert_eq!(Flags::KnightPromotingCapture.remove_capture(), Flags::KnightPromotion);
    }

    #[test]
    fn castles_work() {
        // Check the move's flags
        assert_eq!(ChessMove::WHITE_KING_CASTLE.flags(), Flags::KingSideCastle);
        assert_eq!(ChessMove::BLACK_KING_CASTLE.flags(), Flags::KingSideCastle);
        assert_eq!(ChessMove::WHITE_QUEEN_CASTLE.flags(), Flags::QueenSideCastle);
        assert_eq!(ChessMove::BLACK_QUEEN_CASTLE.flags(), Flags::QueenSideCastle);
        // Check the move's from
        assert_eq!(ChessMove::WHITE_KING_CASTLE.from(), E1);
        assert_eq!(ChessMove::BLACK_KING_CASTLE.from(), E8);
        assert_eq!(ChessMove::WHITE_QUEEN_CASTLE.from(), E1);
        assert_eq!(ChessMove::BLACK_QUEEN_CASTLE.from(), E8);
        // Check the move's to
        assert_eq!(ChessMove::WHITE_KING_CASTLE.to(), G1);
        assert_eq!(ChessMove::BLACK_KING_CASTLE.to(), G8);
        assert_eq!(ChessMove::WHITE_QUEEN_CASTLE.to(), C1);
        assert_eq!(ChessMove::BLACK_QUEEN_CASTLE.to(), C8);
    }
}
