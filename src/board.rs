
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
        *self.mut_piece_mask(p) |= to.mask();
        // Get to_offset
        let Square(to_offset) = to;
        // Add to squares list
        self.squares[to_offset as usize] = p;
    }
    fn remove_piece(&mut self, p: ColoredPiece, from: Square) {
        // Remove from piece mask
        *self.mut_piece_mask(p) &= !from.mask();
        // Get from_offset
        let Square(from_offset) = from;
        // Set piece to none
        self.squares[from_offset as usize] = ColoredPiece::None;
    }
    fn move_piece(&mut self, p: ColoredPiece, from: Square, to: Square) {
        // Update piece mask by removing from 'from' and adding to 'to'
        *self.mut_piece_mask(p) ^= from.mask() ^ to.mask();
        // Update squares
        self.squares[from.0 as usize] = ColoredPiece::None;
        self.squares[to.0 as usize] = p;
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
        self.side = self.side.opposite();
        // Get new castle rights
        let new_castle_right = m.new_castle_permissions(self.castle_rights);
        if new_castle_right != self.castle_rights {
            // Update castle rights
            self.castle_rights = new_castle_right;
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
            self.halfmove_clock = Some(self.halfmove_clock.unwrap_or(0) + 1);
        } else {
            // Handle regular moves
            self.move_piece(m.piece.color(m.side), m.from, m.to);
            // Update halfmove clock
            self.halfmove_clock = if m.piece == Piece::Pawn {
                // If we moved a pawn reset
                Some(0)
            } else {
                // Increment half move clock
                Some(self.halfmove_clock.unwrap_or(0) + 1)
            };
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
    fn make_white_king_castle_works() {
        let mut position = Position::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1".to_string()).unwrap();
        let m = Move::white_king_castle();
        MakeUnmakeBoard::make_move(&mut position, &m);
        // Removes old rook
        assert_eq!(position.squares[square::named::H1.0 as usize], ColoredPiece::None);
        // Moved rook
        assert_eq!(position.squares[square::named::F1.0 as usize], ColoredPiece::WRook);
        // Moved king
        assert_eq!(position.squares[square::named::G1.0 as usize], ColoredPiece::WKing);
        // Removes old king
        assert_eq!(position.squares[square::named::E1.0 as usize], ColoredPiece::None);
        // Updates WKing mask
        assert_eq!(position.piece_mask(ColoredPiece::WKing), 0x40);
        // Updates WRook mask
        assert_eq!(position.piece_mask(ColoredPiece::WRook), 0x21);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::BLACK_ALL);
        // Reset half move clock
        assert_eq!(position.halfmove_clock, Some(0));
    }

    #[test]
    fn unmake_white_king_castle_works() {
        let mut position = Position::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1 w Qkq - 3 1".to_string()).unwrap();
        let m = Move::white_king_castle();
        MakeUnmakeBoard::unmake_move(&mut position, &m, CastlePermissions::ALL, None, Some(3));
        // Moved rook
        assert_eq!(position.squares[square::named::H1.0 as usize], ColoredPiece::WRook);
        // Removes old rook
        assert_eq!(position.squares[square::named::F1.0 as usize], ColoredPiece::None);
        // Removes old king
        assert_eq!(position.squares[square::named::G1.0 as usize], ColoredPiece::None);
        // Moved king
        assert_eq!(position.squares[square::named::E1.0 as usize], ColoredPiece::WKing);
        // Updates WKing mask
        assert_eq!(position.piece_mask(ColoredPiece::WKing), 0x10);
        // Updates WRook mask
        assert_eq!(position.piece_mask(ColoredPiece::WRook), 0x81);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::ALL);
        // Update half move clock
        assert_eq!(position.halfmove_clock, Some(3));
    }

    #[test]
    fn make_black_king_castle_works() {
        let mut position = Position::try_from("rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap();
        let m = Move::black_king_castle();
        MakeUnmakeBoard::make_move(&mut position, &m);
        // Removes old rook
        assert_eq!(position.squares[square::named::H8.0 as usize], ColoredPiece::None);
        // Moved rook
        assert_eq!(position.squares[square::named::F8.0 as usize], ColoredPiece::BRook);
        // Moved king
        assert_eq!(position.squares[square::named::G8.0 as usize], ColoredPiece::BKing);
        // Removes old king
        assert_eq!(position.squares[square::named::E8.0 as usize], ColoredPiece::None);
        // Updates BKing mask
        assert_eq!(position.piece_mask(ColoredPiece::BKing), 0x4000000000000000);
        // Updates BRook mask
        assert_eq!(position.piece_mask(ColoredPiece::BRook), 0x2100000000000000);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::WHITE_ALL);
        // Reset half move clock
        assert_eq!(position.halfmove_clock, Some(0));
    }

    fn unmake_black_king_castle_works() {
        let mut position = Position::try_from("rnbq1rk1/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQq - 0 1".to_string()).unwrap();
        let m = Move::black_king_castle();
        MakeUnmakeBoard::unmake_move(&mut position, &m, CastlePermissions::ALL, None, Some(33));
        // Moved rook
        assert_eq!(position.squares[square::named::H1.0 as usize], ColoredPiece::WRook);
        // Removes old rook
        assert_eq!(position.squares[square::named::F1.0 as usize], ColoredPiece::None);
        // Removes old king
        assert_eq!(position.squares[square::named::G1.0 as usize], ColoredPiece::None);
        // Moved king
        assert_eq!(position.squares[square::named::E1.0 as usize], ColoredPiece::WKing);
        // Updates WKing mask
        assert_eq!(position.piece_mask(ColoredPiece::WKing), 0x10);
        // Updates WRook mask
        assert_eq!(position.piece_mask(ColoredPiece::WRook), 0x81);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::ALL);
        // Update half move clock
        assert_eq!(position.halfmove_clock, Some(33));
    }

    #[test]
    fn make_white_queen_castle_works() {
        let mut position = Position::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1".to_string()).unwrap();
        let m = Move::white_queen_castle();
        MakeUnmakeBoard::make_move(&mut position, &m);
        // Removes old rook
        assert_eq!(position.squares[square::named::A1.0 as usize], ColoredPiece::None);
        // Moved rook
        assert_eq!(position.squares[square::named::D1.0 as usize], ColoredPiece::WRook);
        // Moved king
        assert_eq!(position.squares[square::named::C1.0 as usize], ColoredPiece::WKing);
        // Removes old king
        assert_eq!(position.squares[square::named::E1.0 as usize], ColoredPiece::None);
        // Updates WKing mask
        assert_eq!(position.piece_mask(ColoredPiece::WKing), 0x4);
        // Updates WRook mask
        assert_eq!(position.piece_mask(ColoredPiece::WRook), 0x88);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::BLACK_ALL);
        // Reset half move clock
        assert_eq!(position.halfmove_clock, Some(0));
    }

    #[test]
    fn unmake_white_queen_castle_works() {
        let mut position = Position::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/2KR1BNR w Kkq - 0 1".to_string()).unwrap();
        let m = Move::white_queen_castle();
        MakeUnmakeBoard::unmake_move(&mut position, &m, CastlePermissions::ALL, None, Some(33));
        // Moved rook
        assert_eq!(position.squares[square::named::A1.0 as usize], ColoredPiece::WRook);
        // Removes old rook
        assert_eq!(position.squares[square::named::D1.0 as usize], ColoredPiece::None);
        // Removes old king
        assert_eq!(position.squares[square::named::C1.0 as usize], ColoredPiece::None);
        // Moved king
        assert_eq!(position.squares[square::named::E1.0 as usize], ColoredPiece::WKing);
        // Updates WKing mask
        assert_eq!(position.piece_mask(ColoredPiece::WKing), 0x10);
        // Updates WRook mask
        assert_eq!(position.piece_mask(ColoredPiece::WRook), 0x81);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::ALL);
        // Update half move clock
        assert_eq!(position.halfmove_clock, Some(33));
    }

    #[test]
    fn make_black_queen_castle_works() {
        let mut position = Position::try_from("r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap();
        let m = Move::black_queen_castle();
        MakeUnmakeBoard::make_move(&mut position, &m);
        // Removes old rook
        assert_eq!(position.squares[square::named::A8.0 as usize], ColoredPiece::None);
        // Moved rook
        assert_eq!(position.squares[square::named::D8.0 as usize], ColoredPiece::BRook);
        // Moved king
        assert_eq!(position.squares[square::named::C8.0 as usize], ColoredPiece::BKing);
        // Removes old king
        assert_eq!(position.squares[square::named::E8.0 as usize], ColoredPiece::None);
        // Updates WKing mask
        assert_eq!(position.piece_mask(ColoredPiece::BKing), 0x400000000000000);
        // Updates WRook mask
        assert_eq!(position.piece_mask(ColoredPiece::BRook), 0x8800000000000000);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::WHITE_ALL);
        // Reset half move clock
        assert_eq!(position.halfmove_clock, Some(0));
    }

    #[test]
    fn unmake_black_queen_castle_works() {
        let mut position = Position::try_from("2kr1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQk - 0 1".to_string()).unwrap();
        let m = Move::black_queen_castle();
        MakeUnmakeBoard::unmake_move(&mut position, &m, CastlePermissions::ALL, None, Some(33));
        // Moved rook
        assert_eq!(position.squares[square::named::A8.0 as usize], ColoredPiece::BRook);
        // Removes old rook
        assert_eq!(position.squares[square::named::D8.0 as usize], ColoredPiece::None);
        // Removes old king
        assert_eq!(position.squares[square::named::C8.0 as usize], ColoredPiece::None);
        // Moved king
        assert_eq!(position.squares[square::named::E8.0 as usize], ColoredPiece::BKing);
        // Updates WKing mask
        assert_eq!(position.piece_mask(ColoredPiece::BKing), 0x1000000000000000);
        // Updates WRook mask
        assert_eq!(position.piece_mask(ColoredPiece::BRook), 0x8100000000000000);
        // Update castle permissions
        assert_eq!(position.castle_rights, CastlePermissions::ALL);
        // Update half move clock
        assert_eq!(position.halfmove_clock, Some(33));
    }

    #[test]
    fn make_unmake_preserves_equality() {
        let mut position = Position::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap();
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
        MakeUnmakeBoard::make_move(&mut position, &m);
        MakeUnmakeBoard::unmake_move(&mut position, &m, CastlePermissions::ALL, None, Some(0));
        assert_eq!(String::from(position), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    }

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