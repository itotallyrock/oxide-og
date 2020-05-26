
use std::convert::TryFrom;

use crate::side::Side;
use crate::castles::CastlePermissions;
use crate::square::Square;
use crate::pieces::ColoredPiece;
use crate::errors;

#[derive(Copy, Clone)]
pub struct Position {
    pub side: Side,
    pub fullmove_count: Option<u16>,
    pub halfmove_clock: Option<u8>,
    pub castle_rights: CastlePermissions,
    pub enpassant_square: Option<Square>,
    pub squares: [ColoredPiece; 64],
    piece_masks: [u64; 12],
}

impl Position {
    /// Occupied bit mask
    pub fn occupied_mask(&self) -> u64 {
        self.piece_masks[0] | self.piece_masks[1] | self.piece_masks[2] | self.piece_masks[3] | self.piece_masks[4] | self.piece_masks[5] | self.piece_masks[6] | self.piece_masks[7] | self.piece_masks[8] | self.piece_masks[9] | self.piece_masks[10] | self.piece_masks[11]
    }
    /// Empty bit mask
    pub fn empty_mask(&self) -> u64 {
        !self.occupied_mask()
    }

    pub fn white_mask(&self) -> u64 {
        self.piece_masks[0] | self.piece_masks[1] | self.piece_masks[2] | self.piece_masks[3] | self.piece_masks[4] | self.piece_masks[5]
    }

    pub fn black_mask(&self) -> u64 {
        self.piece_masks[6] | self.piece_masks[7] | self.piece_masks[8] | self.piece_masks[9] | self.piece_masks[10] | self.piece_masks[11]
    }

    pub fn piece_mask(&self, piece: ColoredPiece) -> u64 {
        match piece {
            ColoredPiece::WPawn => self.piece_masks[ColoredPiece::WPawn as usize],
            ColoredPiece::WBishop => self.piece_masks[ColoredPiece::WBishop as usize],
            ColoredPiece::WRook => self.piece_masks[ColoredPiece::WRook as usize],
            ColoredPiece::WKing => self.piece_masks[ColoredPiece::WKing as usize],
            ColoredPiece::WKnight => self.piece_masks[ColoredPiece::WKnight as usize],
            ColoredPiece::WQueen => self.piece_masks[ColoredPiece::WQueen as usize],
            ColoredPiece::BPawn => self.piece_masks[ColoredPiece::BPawn as usize],
            ColoredPiece::BBishop => self.piece_masks[ColoredPiece::BBishop as usize],
            ColoredPiece::BRook => self.piece_masks[ColoredPiece::BRook as usize],
            ColoredPiece::BKing => self.piece_masks[ColoredPiece::BKing as usize],
            ColoredPiece::BKnight => self.piece_masks[ColoredPiece::BKnight as usize],
            ColoredPiece::BQueen => self.piece_masks[ColoredPiece::BQueen as usize],
            ColoredPiece::None => self.empty_mask(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            side: Side::White,
            fullmove_count: Some(0),
            halfmove_clock: Some(0),
            castle_rights: CastlePermissions::NONE,
            enpassant_square: None,
            piece_masks: [0; 12],
            squares: [ColoredPiece::None; 64],
        }
    }
}

impl TryFrom<String> for Position {
    type Error = errors::FenParseError;
    fn try_from(fen: String) -> Result<Self, Self::Error> {
        let side: Side;
        let fullmove_count: Option<u16>;
        let halfmove_clock: Option<u8>;
        let castle_rights: CastlePermissions;
        let enpassant_square: Option<Square>;
        let mut piece_masks = [0; 12];
        let mut squares = [ColoredPiece::None; 64];

        let fen_error = errors::InvalidFenString(fen.clone());
        let mut fen_chunks = fen.split(' ');
        let board_chunk = fen_chunks.next().ok_or(errors::FenParseError::InvalidFenString(fen_error.clone()))?;

        let side_chunk = fen_chunks.next().ok_or(errors::FenParseError::InvalidFenString(fen_error.clone()))?;
        side = Side::try_from(String::from(side_chunk)).or_else(|e| Err(errors::FenParseError::InvalidSideError(e, String::from(side_chunk))))?;

        let castle_chunk = fen_chunks.next().ok_or(errors::FenParseError::InvalidFenString(fen_error.clone()))?;
        castle_rights = CastlePermissions::try_from(String::from(castle_chunk)).or_else(|e| Err(errors::FenParseError::InvalidCastlesError(e, String::from(castle_chunk))))?;

        let enpassant_chunk = fen_chunks.next().ok_or(errors::InvalidFenString(String::from("expected en passant square or -")))?;
        enpassant_square = Square::try_from(String::from(enpassant_chunk)).ok();

        let halfmove_chunk = fen_chunks.next().ok_or(errors::InvalidFenString(String::from("expected half move clock or -")))?;
        halfmove_clock = String::from(halfmove_chunk).parse::<u8>().ok();

        let fullmove_chunk = fen_chunks.next().ok_or(errors::InvalidFenString(String::from("expected full move count or -")))?;
        fullmove_count = String::from(fullmove_chunk).parse::<u16>().ok();

        let mut total_offset: u8 = 56;
        for board_char in board_chunk.chars() {
            if board_char.is_digit(10) {
                total_offset += board_char.to_digit(10).unwrap() as u8;
            } else if board_char == '/' {
                total_offset -= 16u8;
            } else {
                let piece = ColoredPiece::from(board_char);
                let square = Square { offset: total_offset };
                piece_masks[piece as usize] |= square.mask();
                squares[square.offset as usize] = piece;
                total_offset += 1;
            }
        }

        Ok(Position {
            side: side,
            fullmove_count: fullmove_count,
            halfmove_clock: halfmove_clock,
            castle_rights: castle_rights,
            enpassant_square: enpassant_square,
            piece_masks: piece_masks,
            squares: squares,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let position: Position = Default::default();
        assert_eq!(position.occupied_mask(), 0, "Default position is not empty");
    }

    #[test]
    fn default_fen_works() {
        let default_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let position_result: Result<Position, errors::FenParseError> = Position::try_from(default_fen);
        // TODO: Log error from result
        assert!(position_result.is_ok(), "Failed to parse default FEN");

        let position = position_result.unwrap();
        assert_eq!(position.side, Side::White, "Default side was not white");
        assert_eq!(position.white_mask(), 0xffff, "Default white mask was incorrect");
        assert_eq!(position.black_mask(), 0xffff000000000000, "Default black mask was incorrect");
        assert_eq!(position.occupied_mask(), 0xffff00000000ffff, "Default occupied mask was incorrect");

        // Check piece masks
        assert_eq!(position.piece_mask(ColoredPiece::WKing), 0x10, "Default white king mask was incorrect");
        assert_eq!(position.piece_mask(ColoredPiece::BKing), 0x1000000000000000, "Default black king mask was incorrect");

        assert_eq!(position.piece_mask(ColoredPiece::WKnight), 0x42, "Default white knight mask was incorrect");
        assert_eq!(position.piece_mask(ColoredPiece::BKnight), 0x4200000000000000, "Default black knight mask was incorrect");

        assert_eq!(position.piece_mask(ColoredPiece::WBishop), 0x24, "Default white bishop mask was incorrect");
        assert_eq!(position.piece_mask(ColoredPiece::BBishop), 0x2400000000000000, "Default black bishop mask was incorrect");

        assert_eq!(position.piece_mask(ColoredPiece::WQueen), 0x8, "Default white queen mask was incorrect");
        assert_eq!(position.piece_mask(ColoredPiece::BQueen), 0x800000000000000, "Default black queen mask was incorrect");

        assert_eq!(position.piece_mask(ColoredPiece::WRook), 0x81, "Default white rook mask was incorrect");
        assert_eq!(position.piece_mask(ColoredPiece::BRook), 0x8100000000000000, "Default black rook mask was incorrect");

        assert_eq!(position.piece_mask(ColoredPiece::WPawn), 0xff00, "Default white pawn mask was incorrect");
        assert_eq!(position.piece_mask(ColoredPiece::BPawn), 0xff000000000000, "Default black pawn mask was incorrect");

        assert_eq!(position.castle_rights, CastlePermissions::ALL, "Default castle permissions were incorrect");

        // Sample a few squares from squares board to see if that is correct
        assert_eq!(position.squares[0], ColoredPiece::WRook, "White rook expected on a1");
        assert_eq!(position.squares[7], ColoredPiece::WRook, "White rook expected on h1");
        assert_eq!(position.squares[63], ColoredPiece::BRook, "Black rook expected on h8");
        assert_eq!(position.squares[56], ColoredPiece::BRook, "Black rook expected on a8");
        assert_eq!(position.squares[35], ColoredPiece::None, "None expected on d5");
        assert_eq!(position.squares[16], ColoredPiece::None, "None expected on a3");
        assert_eq!(position.squares[15], ColoredPiece::WPawn, "White pawn expected on a2");
        assert_eq!(position.squares[4], ColoredPiece::WKing, "White king expected on e1");
        assert_eq!(position.squares[60], ColoredPiece::BKing, "Black king expected on e8");
    }
}
