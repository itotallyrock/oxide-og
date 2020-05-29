
use super::pieces;
use super::square;
use super::castles;
use super::side;
use crate::pieces::PieceRepr;

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
pub struct Move {
    /// Side doing the move
    pub side: side::Side,
    /// What piece is moving
    pub piece: pieces::Piece,
    /// What square is being moved from
    pub from: square::Square,
    /// What square is being moved to
    pub to: square::Square,
    /// What piece is being captured
    pub captured_piece: Option<pieces::Piece>,
    /// What piece is this being promoted to
    pub promoted_piece: Option<pieces::Piece>,
    /// What en passant square is left after this move
    pub enpassant_square: Option<square::Square>,
    /// Which castles were used in the move (not the permissions before or after move)
    pub castles_used: castles::CastlePermissions,
    /// If the attack was on an en passant pawn
    pub enpassant_capture: bool,
}

impl Move {
    pub fn new_castle_permissions(&self, current_rights: castles::CastlePermissions) -> castles::CastlePermissions {
        let mut new_rights: castles::CastlePermissions = current_rights;
        if self.piece == pieces::Piece::Rook {
            match self.from.offset {
                square::named::H1 if self.side == side::Side::White => { new_rights &= castles::CastlePermissions::BLACK_ALL_WHITE_QUEEN; },
                square::named::A1 if self.side == side::Side::White => { new_rights &= castles::CastlePermissions::BLACK_ALL_WHITE_KING; },
                square::named::H8 if self.side == side::Side::Black => { new_rights &= castles::CastlePermissions::WHITE_ALL_BLACK_QUEEN; },
                square::named::A8 if self.side == side::Side::Black => { new_rights &= castles::CastlePermissions::WHITE_ALL_BLACK_KING; },
                _ => { },
            }
        } else if !self.castles_used.is_empty() || self.piece == pieces::Piece::King {
            if self.side == side::Side::White {
                new_rights &= castles::CastlePermissions::BLACK_ALL;
            } else {
                new_rights &= castles::CastlePermissions::WHITE_ALL;
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