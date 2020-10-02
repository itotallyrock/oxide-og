
// Crate imports
use crate::bitboard::Bitboard;
#[cfg(not(feature = "low_memory"))]
use crate::square::masks::*;
use crate::square::Square;
// Std imports
#[cfg(not(feature = "low_memory"))]
use std::time::Instant;
// External imports
#[cfg(not(feature = "low_memory"))]
use lazy_static::*;
#[cfg(not(feature = "low_memory"))]
use log::trace;

// Mask of edges (no center tiles)
#[cfg(not(feature = "low_memory"))]
const EDGE_SQUARE_MASK: u64 = RANK_1 | RANK_8 | A_FILE | H_FILE;

#[cfg(not(feature = "low_memory"))]
mod magic_numbers {
    pub const MAX_ROOK_VARIATIONS: usize = 4096;
    pub const MAX_BISHOP_VARIATIONS: usize = 512;

    pub const ROOK_BLOCKER_COUNTS: [u8; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10,
        11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10,
        10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
    ];
    pub const BISHOP_BLOCKER_COUNTS: [u8; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7,
        5, 5, 5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5,
        5, 5, 5, 6,
    ];

    // Convert/hash a blocker_index from 0-4096/512 to a mask of occupied squares for 0-12/9 potential blockers
    // occupancy_mask is a mask of potential blockers
    #[inline]
    pub fn map_blocker_index_to_occupancy_mask(blocker_index: u16, occupancy_mask: u64) -> u64 {
        use bitintr::Pdep;
        (blocker_index as u64).pdep(occupancy_mask)
    }

    // Get the attack table index for a relevant blocker mask
    #[inline]
    pub fn get_attack_index(occupied_mask: u64, occupancy_mask: u64) -> usize {
        use bitintr::Pext;
        occupied_mask.pext(occupancy_mask) as usize
    }
}

#[cfg(not(feature = "low_memory"))]
pub fn init_sliding_attacks() {
    let start = Instant::now();
    lazy_static::initialize(&ROOK_OCCUPANCY_MASK);
    lazy_static::initialize(&BISHOP_OCCUPANCY_MASK);
    lazy_static::initialize(&ROOK_BLOCKED_ATTACKS);
    lazy_static::initialize(&BISHOP_BLOCKED_ATTACKS);
    trace!("initialized sliding attacks in {}ms", start.elapsed().as_millis());
}

#[cfg(not(feature = "low_memory"))]
lazy_static! {
    // Rook relevant occupancy mask for given tile
    static ref ROOK_OCCUPANCY_MASK: [u64; Square::COUNT] = {
        let mut occupancy_mask = [0u64; Square::COUNT];
        // For every tile
        for i in 0..Square::COUNT {
            let from_mask = 1u64 << i;
            // Get the relevant rook occupancy tiles (attack files/ranks - edges)
            // Removes the rook (from_mask) so it doesn't block itself
            occupancy_mask[i] = ((Bitboard::north_fill(from_mask) & !RANK_8)
                | (Bitboard::south_fill(from_mask) & !RANK_1)
                | (Bitboard::east_fill(from_mask) & !H_FILE)
                | (Bitboard::west_fill(from_mask) & !A_FILE)) ^ from_mask;
        }

        occupancy_mask
    };

    // Bishop relevant occupancy mask for given tile
    static ref BISHOP_OCCUPANCY_MASK: [u64; Square::COUNT] = {
        let mut occupancy_mask = [0; Square::COUNT];
        // For every tile
        for i in 0..Square::COUNT {
            let from_mask = 1u64 << i;
            // Get the relevant rook occupancy tiles (diagonals - edges)
            // Removes the bishop (from_mask) so it doesn't block itself
            occupancy_mask[i] = ((Bitboard::south_west_fill(from_mask) & !RANK_8)
                | Bitboard::north_west_fill(from_mask)
                | Bitboard::south_east_fill(from_mask)
                | Bitboard::north_east_fill(from_mask)) & !(EDGE_SQUARE_MASK | from_mask);
        }

        occupancy_mask
    };

    // Rook magic bitboard attack table
    static ref ROOK_BLOCKED_ATTACKS: Box<[[u64; Square::COUNT]]> = {
        let mut magic_table = vec![[0u64; Square::COUNT]; magic_numbers::MAX_ROOK_VARIATIONS];
        // For every square
        for i in 0..Square::COUNT {
            // Mask of the square we are coming from
            let from_mask = 1u64 << i;
            // Get relevant occupancy mask
            let occupancy_mask = ROOK_OCCUPANCY_MASK[i];
            // Get max blockers for position
            let blocker_count = magic_numbers::ROOK_BLOCKER_COUNTS[i];

            // Iterate through all possible blocker arrangements for current square
            // Blockers are set bits in a 10-12bit representation (so for each possibility ie. 2^12
            // or 1 << 12 store the moves for those blockers
            for blocker_index in 0..(1u16 << blocker_count) {
                // Get blocker mask from an index
                let blocker_mask = magic_numbers::map_blocker_index_to_occupancy_mask(blocker_index, occupancy_mask);
                // Get attacks mask
                let attack_mask = Bitboard::cardinal_ray_attacks(from_mask, !blocker_mask);
                // Store attack in magic table
                magic_table[blocker_index as usize][i] = attack_mask;
            }
        }

        magic_table.into_boxed_slice()
    };

    // Generate table of all attacks for blocker arrangements
    static ref BISHOP_BLOCKED_ATTACKS: Box<[[u64; Square::COUNT]]> = {
        let mut attack_table = vec![[0u64; Square::COUNT]; magic_numbers::MAX_BISHOP_VARIATIONS];
        // Iterate through each square
        for i in 0..Square::COUNT {
            // Create mask from the index
            let from_mask = 1u64 << i;
            // Get relevant occupancy mask for current square
            let occupancy_mask = BISHOP_OCCUPANCY_MASK[i];
            // Get how many blockers (index bits) can exist for square
            let blocker_count = magic_numbers::BISHOP_BLOCKER_COUNTS[i];

            // Iterate through all possible blocker arrangements for current square
            // Blockers are set bits in a 5-9bit representation (so for each possibility ie. 2^12
            // or 1 << 9 store the moves for those blockers
            for blocker_index in 0..(1u16 << blocker_count) {
                // Get blocker mask from an index
                let blocker_mask = magic_numbers::map_blocker_index_to_occupancy_mask(blocker_index, occupancy_mask);
                // Get attacks mask
                let attack_mask = Bitboard::diagonal_ray_attacks(from_mask, !blocker_mask);
                // Store attack in magic table
                attack_table[blocker_index as usize][i] = attack_mask;
            }
        }

        attack_table.into_boxed_slice()
    };
}

#[inline]
#[cfg(not(feature = "low_memory"))]
pub fn rook_attacks(from_square: Square, occupied_mask: u64) -> u64 {
    rook_attacks_lookup(from_square, occupied_mask)
}

#[inline]
#[cfg(feature = "low_memory")]
pub fn rook_attacks(from_square: Square, occupied_mask: u64) -> u64 {
    Bitboard::cardinal_ray_attacks(from_square.mask(), !occupied_mask)
}

#[inline]
#[cfg(not(feature = "low_memory"))]
fn rook_attacks_lookup(from_square: Square, occupied_mask: u64) -> u64 {
    let square_offset = from_square.offset() as usize;
    let occupancy_mask = ROOK_OCCUPANCY_MASK[square_offset];
    let index = magic_numbers::get_attack_index(occupied_mask, occupancy_mask);

    ROOK_BLOCKED_ATTACKS[index][square_offset]
}

#[inline]
#[cfg(not(feature = "low_memory"))]
pub fn bishop_attacks(from_square: Square, occupied_mask: u64) -> u64 {
    bishop_attacks_lookup(from_square, occupied_mask)
}

#[inline]
#[cfg(feature = "low_memory")]
pub fn bishop_attacks(from_square: Square, occupied_mask: u64) -> u64 {
    Bitboard::diagonal_ray_attacks(from_square.mask(), !occupied_mask)
}

#[inline]
#[cfg(not(feature = "low_memory"))]
fn bishop_attacks_lookup(from_square: Square, occupied_mask: u64) -> u64 {
    let square_offset = from_square.offset() as usize;
    let occupancy_mask = BISHOP_OCCUPANCY_MASK[square_offset];
    let index = magic_numbers::get_attack_index(occupied_mask, occupancy_mask);

    BISHOP_BLOCKED_ATTACKS[index][square_offset]
}

#[inline]
#[cfg(not(feature = "low_memory"))]
pub fn queen_attacks(from_square: Square, occupied_mask: u64) -> u64 {
    bishop_attacks_lookup(from_square, occupied_mask) | rook_attacks_lookup(from_square, occupied_mask)
}

#[inline]
#[cfg(feature = "low_memory")]
pub fn queen_attacks(from_square: Square, occupied_mask: u64) -> u64 {
    let from_mask = from_square.mask();
    Bitboard::diagonal_ray_attacks(from_mask, !occupied_mask) | Bitboard::cardinal_ray_attacks(from_mask, !occupied_mask)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::named::*;

    #[test]
    fn rook_occupancy_mask_table_works() {
        assert_eq!(ROOK_OCCUPANCY_MASK[0], 0x101010101017e);
        assert_eq!(ROOK_OCCUPANCY_MASK[37], 0x20205e20202000);
        assert_eq!(ROOK_OCCUPANCY_MASK[14], 0x40404040403e00);
        assert_eq!(ROOK_OCCUPANCY_MASK[9], 0x2020202027c00);
    }

    #[test]
    fn bishop_occupancy_mask_table_works() {
        assert_eq!(BISHOP_OCCUPANCY_MASK[0], 0x40201008040200);
        assert_eq!(BISHOP_OCCUPANCY_MASK[63], 0x40201008040200);
        assert_eq!(BISHOP_OCCUPANCY_MASK[63], 0x40201008040200);
        assert_eq!(BISHOP_OCCUPANCY_MASK[7], 0x2040810204000);
        assert_eq!(BISHOP_OCCUPANCY_MASK[56], 0x2040810204000);
        assert_eq!(BISHOP_OCCUPANCY_MASK[37], 0x8500050080400);
    }

    #[test]
    fn map_blocker_index_to_occupancy_mask_works() {
        // No blockers index for rook on h1
        assert_eq!(
            magic_numbers::map_blocker_index_to_occupancy_mask(0x0, 0x8080808080807e),
            0x0
        );

        // Squares for rook in h1
        assert_eq!(
            magic_numbers::map_blocker_index_to_occupancy_mask(0xfff, 0x8080808080807e),
            0x8080808080807e
        );
        assert_eq!(
            magic_numbers::map_blocker_index_to_occupancy_mask(0x9ff, 0x8080808080807e),
            0x8000008080807e
        );
        assert_eq!(
            magic_numbers::map_blocker_index_to_occupancy_mask(0x841, 0x8080808080807e),
            0x80000000008002
        );

        // Diagonals from different square
        assert_eq!(
            magic_numbers::map_blocker_index_to_occupancy_mask(0x1, 0x50005008040200),
            0x200
        );
        assert_eq!(
            magic_numbers::map_blocker_index_to_occupancy_mask(0xff, 0x50005008040200),
            0x50005008040200
        );
        assert_eq!(
            magic_numbers::map_blocker_index_to_occupancy_mask(0xf0, 0x50005008040200),
            0x50004000000000
        );
    }

    #[test]
    fn rook_magic_attacks_lookup_works() {
        assert_eq!(rook_attacks_lookup(F3, 0x200000), 0x2020202020df2020);
    }

    #[test]
    fn bishop_magic_attacks_lookup_works() {
        assert_eq!(bishop_attacks_lookup(F5, 0x2000000000), 0x488500050880402);
    }
}

#[cfg(test)]
mod bench {
    // Local imports
    use super::*;

    // External test for benchmarking
    extern crate test;
    use crate::square::named::*;
    use test::Bencher;

    #[bench]
    fn rook_attack_bench(bencher: &mut Bencher) {
        let from_square = test::black_box(A3);
        let occupied = test::black_box(0x90000);
        bencher.iter(|| rook_attacks(from_square, occupied));
    }

    #[bench]
    fn bishop_attack_bench(bencher: &mut Bencher) {
        let from_square = test::black_box(A3);
        let occupied = test::black_box(0x80000010000);
        bencher.iter(|| bishop_attacks(from_square, occupied));
    }

    #[bench]
    fn queen_attack_bench(bencher: &mut Bencher) {
        let from_square = test::black_box(A3);
        let occupied = test::black_box(0x80000210801);
        bencher.iter(|| queen_attacks(from_square, occupied));
    }
}