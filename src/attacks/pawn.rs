
// Local imports
use crate::side::Side;
use crate::bitboard::Bitboard;

#[inline]
pub const fn pawn_pushes(side: Side, from_mask: u64) -> u64 {
    match side {
        Side::WHITE => Bitboard::north_shift(from_mask),
        _ => Bitboard::south_shift(from_mask)
    }
}

#[inline]
pub const fn west_attacks(side: Side, from_mask: u64) -> u64 {
    match side {
        Side::WHITE => Bitboard::north_west_shift(from_mask),
        _ => Bitboard::south_west_shift(from_mask)
    }
}

#[inline]
pub const fn east_attacks(side: Side, from_mask: u64) -> u64 {
    match side {
        Side::WHITE => Bitboard::north_east_shift(from_mask),
        _ => Bitboard::south_east_shift(from_mask)
    }
}

#[inline]
pub const fn pawn_attacks(side: Side, from_mask: u64) -> u64 {
    east_attacks(side, from_mask) | west_attacks(side, from_mask)
}

