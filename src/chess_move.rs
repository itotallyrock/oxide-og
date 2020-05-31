
use super::square::Square;
use super::castles::CastlePermissions;
use super::pieces::{PieceRepr, Piece};
use super::side::Side;
use super::square;

/// Allows converting move to SAN
pub trait SanMove {
    /// Move to SAN string
    fn to_san(&self) -> String;
}

/// Allows converting move to UCI
pub trait UCIMove {
    /// Move to UCI string
    fn to_uci(&self) -> String;
}

/// Move associated data irrespective of board state
/// Reversible entirely
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    /// Side doing the move
    pub side: Side,
    /// What piece is moving
    pub piece: Piece,
    /// What square is being moved from
    pub from: Square,
    /// What square is being moved to
    pub to: Square,
    /// What piece is being captured
    pub captured_piece: Option<Piece>,
    /// What piece is this being promoted to
    pub promoted_piece: Option<Piece>,
    /// What en passant square is left after this move
    pub enpassant_square: Option<Square>,
    /// Which castles were used in the move (not the permissions before or after move)
    pub castles_used: CastlePermissions,
    /// If the attack was on an en passant pawn
    pub enpassant_capture: bool,
}

impl Move {

    pub fn new(side: Side, piece: Piece, from: Square, to: Square) -> Move {
        Move {
            side,
            piece,
            from,
            to,
            captured_piece: None,
            promoted_piece: None,
            enpassant_square: None,
            castles_used: Default::default(),
            enpassant_capture: false
        }
    }

    pub fn capture(&mut self, captured_piece: Piece) -> &mut Self {
        self.captured_piece = Some(captured_piece);
        self
    }

    pub fn promote(&mut self, promoted_piece: Piece) -> &mut Self {
        self.promoted_piece = Some(promoted_piece);
        self
    }

    pub fn en_passant_capture(&mut self) -> &mut Self {
        self.captured_piece = Some(Piece::Pawn);
        self.enpassant_capture = true;
        self
    }

    #[inline]
    pub fn white_king_castle() -> Self {
        Move {
            side: Side::White,
            piece: Piece::King,
            from: square::named::E1,
            to: square::named::G1,
            captured_piece: None,
            promoted_piece: None,
            enpassant_square: None,
            castles_used: CastlePermissions::WHITE_KING,
            enpassant_capture: false
        }
    }

    #[inline]
    pub fn white_queen_castle() -> Self {
        Move {
            side: Side::White,
            piece: Piece::King,
            from: square::named::E1.into(),
            to: square::named::C1.into(),
            captured_piece: None,
            promoted_piece: None,
            enpassant_square: None,
            castles_used: CastlePermissions::WHITE_QUEEN,
            enpassant_capture: false
        }
    }

    #[inline]
    pub fn black_king_castle() -> Self {
        Move {
            side: Side::Black,
            piece: Piece::King,
            from: square::named::E8.into(),
            to: square::named::G8.into(),
            captured_piece: None,
            promoted_piece: None,
            enpassant_square: None,
            castles_used: CastlePermissions::BLACK_KING,
            enpassant_capture: false
        }
    }

    #[inline]
    pub fn black_queen_castle() -> Self {
        Move {
            side: Side::Black,
            piece: Piece::Queen,
            from: square::named::E8.into(),
            to: square::named::C8.into(),
            captured_piece: None,
            promoted_piece: None,
            enpassant_square: None,
            castles_used: CastlePermissions::BLACK_QUEEN,
            enpassant_capture: false
        }
    }

    pub fn new_castle_permissions(&self, current_rights: CastlePermissions) -> CastlePermissions {
        let mut new_rights: CastlePermissions = current_rights;
        if self.piece == Piece::Rook {
            match self.from {
                square::named::H1 if self.side == Side::White => { new_rights &= CastlePermissions::BLACK_ALL_WHITE_QUEEN; },
                square::named::A1 if self.side == Side::White => { new_rights &= CastlePermissions::BLACK_ALL_WHITE_KING; },
                square::named::H8 if self.side == Side::Black => { new_rights &= CastlePermissions::WHITE_ALL_BLACK_QUEEN; },
                square::named::A8 if self.side == Side::Black => { new_rights &= CastlePermissions::WHITE_ALL_BLACK_KING; },
                _ => { },
            }
        } else if !self.castles_used.is_empty() || self.piece == Piece::King {
            if self.side == Side::White {
                new_rights &= CastlePermissions::BLACK_ALL;
            } else {
                new_rights &= CastlePermissions::WHITE_ALL;
            }
        }

        new_rights
    }
}

impl UCIMove for Move {
    fn to_uci(&self) -> String {
        let promotion = if self.promoted_piece.is_some() { self.promoted_piece.unwrap().to_ascii().to_string() } else { String::from("") };
        format!("{}{}{}", self.from.to_string(), self.to.to_string(), promotion)
    }
}