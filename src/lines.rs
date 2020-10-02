
// Local imports
use crate::bitboard::Bitboard;
#[cfg(not(feature = "low_memory"))]
use crate::square::SQUARES;
use crate::square::{masks::ALL, Square};
// Std imports
#[cfg(not(feature = "low_memory"))]
use std::time::Instant;
// External imports
#[cfg(not(feature = "low_memory"))]
use lazy_static::*;
#[cfg(not(feature = "low_memory"))]
use log::trace;

fn line_bb(from_square: Square, to_square: Square) -> u64 {
    let from_mask = from_square.mask();
    // Ignore from == to (no movement move)
    if from_square == to_square {
        return from_mask;
    }
    // println!("Getting between for {} {}", from_square, to_square);
    // Get to square mask
    let to_mask = to_square.mask();
    // Get rook attacks from the from square blocked by to
    let from_rook = Bitboard::cardinal_ray_attacks(from_mask, !0u64);
    // Get rook attacks from the to square blocked by from
    let to_rook = Bitboard::cardinal_ray_attacks(to_mask, !0u64);

    // Get bishop attacks from the from square blocked by to
    let from_bishop = Bitboard::diagonal_ray_attacks(from_mask, !0u64);
    // Get bishop attacks from the to square blocked by from
    let to_bishop = Bitboard::diagonal_ray_attacks(to_mask, !0u64);

    // If the rook attacks overlap
    if from_rook & to_mask > 0 {
        (from_rook & to_rook) | from_mask | to_mask
    // If the bishop attacks overlap
    } else if from_bishop & to_mask > 0 {
        (from_bishop & to_bishop) | from_mask | to_mask
    // Otherwise 0
    } else {
        0
    }
}

#[cfg(not(feature = "low_memory"))]
lazy_static! {
    static ref LINE_TABLE: [[u64; 64]; 64] = {
        let mut lines = [[0u64; 64]; 64];
        // Iterate through each to-from pair
        for &from_square in SQUARES.iter() {
            for &to_square in SQUARES.iter() {
                lines[from_square.offset() as usize][to_square.offset() as usize] = line_bb(from_square, to_square);
            }
        }

        lines
    };
}

#[cfg(not(feature = "low_memory"))]
pub fn init_line_table() {
    let start = Instant::now();
    lazy_static::initialize(&LINE_TABLE);
    trace!("initialized line table in {}ms", start.elapsed().as_millis())
}

#[inline]
pub fn between_fill(a: Square, b: Square) -> u64 {
    use bitintr::Blsr;
    let line_mask = line_fill(a, b);
    // If aligned
    if line_mask != 0 {
        (line_mask & ((ALL << a.offset() as u64) ^ (ALL << b.offset() as u64))).blsr()
    } else {
        0
    }
}
#[inline]
pub fn aligned(a: Square, b: Square, c: Square) -> bool {
    // If the line between a and b intersects c
    line_fill(a, b) & c.mask() > 0
}
#[inline]
#[cfg(not(feature = "low_memory"))]
pub fn line_fill(a: Square, b: Square) -> u64 {
    LINE_TABLE[a.offset() as usize][b.offset() as usize]
}
#[inline]
#[cfg(feature = "low_memory")]
pub fn line_fill(a: Square, b: Square) -> u64 {
    line_bb(a, b)
}

// TODO: Test
