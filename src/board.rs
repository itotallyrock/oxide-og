
use std::convert::TryFrom;

use super::position::Position;
use super::chess_move::Move;
use super::side::Side;
use super::pieces::{ColoredPiece, Piece};
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
        // Add to squares list
        self.squares[to.offset as usize] = p;
    }
    fn remove_piece(&mut self, p: ColoredPiece, from: Square) {
        // Remove from piece mask
        self.piece_masks[p as usize] &= !from.mask();
        // Set piece to none
        self.squares[from.offset as usize] = ColoredPiece::None;
    }
    fn move_piece(&mut self, p: ColoredPiece, from: Square, to: Square) {
        // Update piece mask by removing from 'from' and adding to 'to'
        self.piece_masks[p as usize] ^= from.mask() ^ to.mask();
        // Update squares
        self.squares[from.offset as usize] = ColoredPiece::None;
        self.squares[to.offset as usize] = p;
    }
}

pub trait CopyMakeBoard {
    fn make_move(&self, m: &Move) -> Self;
}

pub trait MakeUnmakeBoard {
    fn make_move(&mut self, m: &Move);
    fn unmake_move(&mut self, m: &Move);
}

impl MakeUnmakeBoard for Position {
    fn make_move(&mut self, m: &Move) {
        self.enpassant_square = m.enpassant_square;
        self.castle_rights = m.new_castle_permissions(self.castle_rights);
        self.side = self.side.opposite();

        if m.captured_piece.is_some() && m.promoted_piece.is_some() {
            // Remove captured piece
            self.remove_piece(m.captured_piece.unwrap().color(m.side.opposite()), m.to);
            // Remove old piece
            self.remove_piece(Piece::Pawn.color(m.side), m.from);
            // Add promoted piece
            self.add_piece(m.promoted_piece.unwrap().color(m.side), m.to);
        } else if m.captured_piece.is_some() {
            // Remove captured piece
            self.remove_piece(m.captured_piece.unwrap().color(m.side.opposite()), m.to);
            // Move the original piece
            self.move_piece(m.piece.color(m.side), m.from, m.to);
        } else if m.castles_used.intersects(CastlePermissions::BOTH_KINGS) {
            // Move the king
            self.move_piece(Piece::King.color(m.side), m.from, m.to);
            // Move appropriate rook
            if m.side == Side::White {
                self.move_piece(ColoredPiece::WRook, Square::try_from(7).unwrap(), Square::try_from(5).unwrap());
            } else {
                self.move_piece(ColoredPiece::BRook, Square::try_from(63).unwrap(), Square::try_from(61).unwrap());
            }
        } else if m.castles_used.intersects(CastlePermissions::BOTH_QUEENS) {
            // Move the king
            self.move_piece(Piece::King.color(m.side), m.from, m.to);
            // Move appropriate rook
            if m.side == Side::White {
                self.move_piece(ColoredPiece::WRook, Square::try_from(0).unwrap(), Square::try_from(3).unwrap());
            } else {
                self.move_piece(ColoredPiece::BRook, Square::try_from(56).unwrap(), Square::try_from(59).unwrap());
            }
        } else if m.enpassant_capture {
            // Get location of jumped over pawn
            let en_passant_offset = ((m.to.offset as i8) + if m.side == Side::White { -8 } else { 8 }) as u8;
            let en_passant_square = Square::try_from(en_passant_offset).unwrap();
            // Remove en passant pawn
            self.remove_piece(Piece::Pawn.color(m.side.opposite()), en_passant_square);
            // Move the piece
            self.move_piece(m.piece.color(m.side), m.from, m.to);
        } else if m.promoted_piece.is_some() {
            // Remove old pawn
            self.remove_piece(Piece::Pawn.color(m.side), m.from);
            // Add the promoted piece
            self.add_piece(m.promoted_piece.unwrap().color(m.side), m.to);
        } else {
            // Handle regular moves
            self.move_piece(m.piece.color(m.side), m.from, m.to);
        }
    }

    fn unmake_move(&mut self, m: &Move) {
        self.enpassant_square = m.enpassant_square;
        // Update castling rights (works because xor is symmetric)
        self.castle_rights = m.new_castle_permissions(self.castle_rights);
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
                self.move_piece(ColoredPiece::WRook, Square::try_from(5).unwrap(), Square::try_from(7).unwrap());
            } else {
                self.move_piece(ColoredPiece::BRook, Square::try_from(61).unwrap(), Square::try_from(63).unwrap());
            }
        } else if m.castles_used.intersects(CastlePermissions::BOTH_QUEENS) {
            // Move the king
            self.move_piece(Piece::King.color(m.side), m.to, m.from);
            // Move appropriate rook
            if m.side == Side::White {
                self.move_piece(ColoredPiece::WRook, Square::try_from(3).unwrap(), Square::try_from(0).unwrap());
            } else {
                self.move_piece(ColoredPiece::BRook, Square::try_from(59).unwrap(), Square::try_from(56).unwrap());
            }
        } else if m.enpassant_capture {
            // TODO: TEST ME
            // Get location of jumped over pawn
            // TODO: Checkme
            let en_passant_offset = ((m.to.offset as i8) + if m.side == Side::White { -8 } else { 8 }) as u8;
            let en_passant_square = Square::try_from(en_passant_offset).unwrap();
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
    fn make_move(&self, m: &Move) -> Self {
        let mut copy_board = self.clone();
        <Position as MakeUnmakeBoard>::make_move(copy_board.borrow_mut(), m);
        copy_board
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
            from: Square { offset: 12 },
            to: Square { offset: 28 },
            captured_piece: None,
            promoted_piece: None,
            enpassant_square: Some(Square { offset: 20 }),
            castles_used: Default::default(),
            enpassant_capture: false
        };
        let moved_position = CopyMakeBoard::make_move(&position, &m );
        assert_eq!(String::from(moved_position), "8/8/8/8/4P3/8/8/8 b - e3 0 1".to_string(), "Updated FEN was not as expected after make move");
    }
}