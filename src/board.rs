
use std::convert::TryFrom;

use super::position::Position;
use super::chess_move::Move;
use super::side::Side;
use super::pieces::{ColoredPiece, Piece};
use super::square;
use super::square::Square;
use super::castles::CastlePermissions;
use std::borrow::BorrowMut;

/// Extend position to allow piece manipulation
pub trait MutablePosition {
    /// Add a colored piece 'p' at square 'to'
    fn add_piece(&mut self, p: ColoredPiece, to: Square);
    /// Remove a colored piece 'p' at square 'from'
    fn remove_piece(&mut self, p: ColoredPiece, from: Square);
    /// Move a colored piece 'p' from square 'from' to square 'to'
    fn move_piece(&mut self, p: ColoredPiece, from: Square, to: Square);
}

impl MutablePosition for Position {
    fn add_piece(&mut self, p: ColoredPiece, to: Square) {
        // Add to piece mask
        self.piece_masks[p as usize] |= to.mask();
        // Get to_offset
        let Square(to_offset) = to;
        // Add to squares list
        self.squares[to_offset as usize] = p;
    }
    fn remove_piece(&mut self, p: ColoredPiece, from: Square) {
        // Remove from piece mask
        self.piece_masks[p as usize] &= !from.mask();
        // Get from_offset
        let Square(from_offset) = from;
        // Set piece to none
        self.squares[from_offset as usize] = ColoredPiece::None;
    }
    fn move_piece(&mut self, p: ColoredPiece, from: Square, to: Square) {
        // Update piece mask by removing from 'from' and adding to 'to'
        self.piece_masks[p as usize] ^= from.mask() ^ to.mask();
        // Get from_offset
        let Square(from_offset) = from;
        // Get to_offset
        let Square(to_offset) = to;
        // Update squares
        self.squares[from_offset as usize] = ColoredPiece::None;
        self.squares[to_offset as usize] = p;
    }
}

pub trait CopyMakeBoard {
    fn make_move(self, m: &Move) -> Self;
}

pub trait MakeUnmakeBoard {
    fn make_move(&mut self, m: &Move);
    fn unmake_move(&mut self, m: &Move, prev_castle_permissions: CastlePermissions, prev_en_passant: Option<Square>, prev_half_move: Option<u8>);
}

impl MakeUnmakeBoard for Position {
    fn make_move(&mut self, m: &Move) {
        self.enpassant_square = m.enpassant_square;
        self.castle_rights = m.new_castle_permissions(self.castle_rights);
        self.side = self.side.opposite();
        // Get new castle rights
        let new_castle_right = m.new_castle_permissions(self.castle_rights);
        if new_castle_right != self.castle_rights {
            // Reset half move clock on castle change
            self.halfmove_clock = Some(0);
        }

        if m.captured_piece.is_some() && m.promoted_piece.is_some() {
            // Remove captured piece
            self.remove_piece(m.captured_piece.unwrap().color(m.side.opposite()), m.to);
            // Remove old piece
            self.remove_piece(Piece::Pawn.color(m.side), m.from);
            // Add promoted piece
            self.add_piece(m.promoted_piece.unwrap().color(m.side), m.to);
            // Reset half move clock on capture
            self.halfmove_clock = Some(0);
        } else if m.captured_piece.is_some() {
            // Remove captured piece
            self.remove_piece(m.captured_piece.unwrap().color(m.side.opposite()), m.to);
            // Move the original piece
            self.move_piece(m.piece.color(m.side), m.from, m.to);
            // Reset half move clock on capture
            self.halfmove_clock = Some(0);
        } else if m.castles_used.intersects(CastlePermissions::BOTH_KINGS) {
            // Move the king
            self.move_piece(Piece::King.color(m.side), m.from, m.to);
            // Move appropriate rook
            if m.side == Side::White {
                self.move_piece(ColoredPiece::WRook, square::named::H1, square::named::F1);
            } else {
                self.move_piece(ColoredPiece::BRook, square::named::H8, square::named::F8);
            }
        } else if m.castles_used.intersects(CastlePermissions::BOTH_QUEENS) {
            // Move the king
            self.move_piece(Piece::King.color(m.side), m.from, m.to);
            // Move appropriate rook
            if m.side == Side::White {
                self.move_piece(ColoredPiece::WRook, square::named::A1, square::named::D1);
            } else {
                self.move_piece(ColoredPiece::BRook, square::named::A8, square::named::D8);
            }
        } else if m.enpassant_capture {
            // Get to offset
            let Square(to_offset) = m.to;
            // Get location of jumped over pawn
            let en_passant_offset = ((to_offset as i8) + if m.side == Side::White { -8 } else { 8 }) as u8;
            let en_passant_square = Square(en_passant_offset);
            // Remove en passant pawn
            self.remove_piece(Piece::Pawn.color(m.side.opposite()), en_passant_square);
            // Move the piece
            self.move_piece(m.piece.color(m.side), m.from, m.to);
            // Reset half move clock on capture
            self.halfmove_clock = Some(0);
        } else if m.promoted_piece.is_some() {
            // Remove old pawn
            self.remove_piece(Piece::Pawn.color(m.side), m.from);
            // Add the promoted piece
            self.add_piece(m.promoted_piece.unwrap().color(m.side), m.to);
            // Increment half move clock
            self.halfmove_clock = if self.halfmove_clock.is_some() { Some(self.halfmove_clock.unwrap() + 1) } else { Some(0) };
        } else {
            // Handle regular moves
            self.move_piece(m.piece.color(m.side), m.from, m.to);
            // Increment half move clock
            self.halfmove_clock = if self.halfmove_clock.is_some() { Some(self.halfmove_clock.unwrap() + 1) } else { Some(0) };
        }
    }

    fn unmake_move(&mut self, m: &Move, prev_castle_permissions: CastlePermissions, prev_en_passant: Option<Square>, prev_half_move: Option<u8>) {
        self.enpassant_square = prev_en_passant;
        self.halfmove_clock = prev_half_move;
        // Update castling rights (works because xor is symmetric)
        self.castle_rights = prev_castle_permissions;
        self.side = self.side.opposite();

        if m.captured_piece.is_some() && m.promoted_piece.is_some() {
            // TODO: TEST ME
            // Remove promoted piece
            self.remove_piece(m.promoted_piece.unwrap().color(m.side), m.to);
            // Add captured piece
            self.add_piece(m.captured_piece.unwrap().color(m.side.opposite()), m.to);
            // Add original pawn
            self.add_piece(Piece::Pawn.color(m.side), m.from);
        } else if m.captured_piece.is_some() {
            // Move the original piece
            self.move_piece(m.piece.color(m.side), m.to, m.from);
            // Remove captured piece
            self.remove_piece(m.captured_piece.unwrap().color(m.side.opposite()), m.to);
        } else if m.castles_used.intersects(CastlePermissions::BOTH_KINGS) {
            // Move the king
            self.move_piece(Piece::King.color(m.side), m.to, m.from);
            // Move appropriate rook
            if m.side == Side::White {
                self.move_piece(ColoredPiece::WRook, square::named::F1, square::named::H1);
            } else {
                self.move_piece(ColoredPiece::BRook, square::named::F8, square::named::H8);
            }
        } else if m.castles_used.intersects(CastlePermissions::BOTH_QUEENS) {
            // Move the king
            self.move_piece(Piece::King.color(m.side), m.to, m.from);
            // Move appropriate rook
            if m.side == Side::White {
                self.move_piece(ColoredPiece::WRook, square::named::D1, square::named::A1);
            } else {
                self.move_piece(ColoredPiece::BRook, square::named::D8, square::named::A8);
            }
        } else if m.enpassant_capture {
            // TODO: TEST ME
            // Get location of jumped over pawn
            // TODO: Checkme
            let Square(to_offset) = m.to;
            let en_passant_offset = ((to_offset as i8) + if m.side == Side::White { -8 } else { 8 }) as u8;
            let en_passant_square = Square(en_passant_offset);
            // Move the piece
            // TODO: Checkme
            self.move_piece(m.piece.color(m.side), m.from, m.to);
            // Add en passant pawn
            self.add_piece(Piece::Pawn.color(m.side.opposite()), en_passant_square);
        } else if m.promoted_piece.is_some() {
            // Remove promoted piece
            self.remove_piece(m.promoted_piece.unwrap().color(m.side), m.to);
            // Add the original pawn
            self.add_piece(Piece::Pawn.color(m.side), m.from);
        } else {
            // Handle regular moves
            self.move_piece(m.piece.color(m.side), m.to, m.from);
        }
    }
}

impl CopyMakeBoard for Position {
    fn make_move(mut self, m: &Move) -> Self {
        <Position as MakeUnmakeBoard>::make_move(&mut self, m);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_make_board_copies() {
        let position = Position::try_from("8/8/8/8/8/8/4P3/8 w - - 0 1".to_string()).unwrap();
        let m = Move {
            side: Side::White,
            piece: Piece::Pawn,
            from: square::named::E2,
            to: square::named::E4,
            captured_piece: None,
            promoted_piece: None,
            enpassant_square: Some(square::named::E3),
            castles_used: Default::default(),
            enpassant_capture: false
        };
        let moved_position = CopyMakeBoard::make_move(position, &m );
        assert_eq!(String::from(moved_position), "8/8/8/8/4P3/8/8/8 b - e3 0 1".to_string(), "Updated FEN was not as expected after make move");
    }
}