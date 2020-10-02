
// Local imports
use crate::side::Side;
use crate::bitboard::Bitboard;

#[inline]
pub fn pawn_attacks(side: Side, from_mask: u64) -> u64 {
    if side == Side::WHITE {
        Bitboard::north_east_shift(from_mask) | Bitboard::north_west_shift(from_mask)
    } else {
        Bitboard::south_east_shift(from_mask) | Bitboard::south_west_shift(from_mask)
    }
}

