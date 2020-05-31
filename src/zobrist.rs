
use super::position;
use super::square::Square;
use super::castles::CastlePermissions;
use super::side::Side;
use crate::pieces::ColoredPiece;
use lazy_static::*;

lazy_static! {
    pub static ref ZOBRIST_KEYS: Vec<u64> = {
        use rand::{SeedableRng, rngs::StdRng, RngCore};
        let seed = 0xDEADB33Fu64;
        let mut rng = StdRng::seed_from_u64(seed);

        (0..781).map(|_| rng.next_u64()).collect()
    };
}

pub trait ZobristHashable {
    fn zobrist_hash(&self) -> u64;
}

impl ZobristHashable for position::Position {
    fn zobrist_hash(&self) -> u64 {
        let mut hash = 0u64;
        // Add each piece to hash
        for i in 0..64 {
            let piece = self.squares[i];
            if piece == ColoredPiece::None {
                continue;
            }
            hash ^= ZOBRIST_KEYS[12 * i + (piece as usize) - 1];
        }
        // Hash side to move
        if self.side == Side::Black {
            hash ^= ZOBRIST_KEYS[768];
        }

        if self.castle_rights.contains(CastlePermissions::BLACK_KING) {
            hash ^= ZOBRIST_KEYS[769];
        }
        if self.castle_rights.contains(CastlePermissions::BLACK_QUEEN) {
            hash ^= ZOBRIST_KEYS[770];
        }
        if self.castle_rights.contains(CastlePermissions::WHITE_KING) {
            hash ^= ZOBRIST_KEYS[771];
        }
        if self.castle_rights.contains(CastlePermissions::WHITE_QUEEN) {
            hash ^= ZOBRIST_KEYS[772];
        }

        if self.enpassant_square.is_some() {
            let Square(offset) = self.enpassant_square.unwrap();
            hash ^= ZOBRIST_KEYS[773 + (offset as usize)];
        }

        hash
    }
}