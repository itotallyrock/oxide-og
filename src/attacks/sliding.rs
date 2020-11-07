
// Crate imports
use crate::bitboard::Bitboard;
use crate::square::Square;
// Std imports
#[cfg(not(feature = "low_memory"))]
use std::time::Instant;
// External imports
#[cfg(not(feature = "low_memory"))]
use lazy_static::*;
#[cfg(not(feature = "low_memory"))]
use log::trace;

#[cfg(not(feature = "low_memory"))]
mod magic_numbers {
    pub const MAX_ROOK_VARIATIONS: usize = 4096;
    pub const MAX_BISHOP_VARIATIONS: usize = 512;

    pub const ROOK_BLOCKER_COUNTS: [u8; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12,
    ];
    pub const BISHOP_BLOCKER_COUNTS: [u8; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6,
        5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
        6, 5, 5, 5, 5, 5, 5, 6,
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

    #[cfg(test)]
    mod bench {
        use super::*;

        extern crate test;
        use test::Bencher;

        #[bench]
        fn get_attack_index_bench(bencher: &mut Bencher) {
            const OCCUPANCY_MASK_BISHOP_D4: u64 = 0x40221400142200;
            let occupied_mask = test::black_box(0x8000180408402200);
            bencher.iter(|| get_attack_index(occupied_mask, OCCUPANCY_MASK_BISHOP_D4));
        }
    }
}

#[cfg(not(feature = "low_memory"))]
pub fn init_sliding_attacks() {
    let start = Instant::now();
    lazy_static::initialize(&ROOK_BLOCKED_ATTACKS);
    lazy_static::initialize(&BISHOP_BLOCKED_ATTACKS);
    trace!("initialized sliding attacks in {}ms", start.elapsed().as_millis());
}

#[cfg(not(feature = "low_memory"))]
const BISHOP_OCCUPANCY_MASK: [u64; Square::COUNT] = [
    0x40201008040200, 0x402010080400,   0x4020100A00,     0x40221400,       0x2442800,        0x204085000,      0x20408102000,    0x2040810204000,
    0x20100804020000, 0x40201008040000, 0x4020100A0000,   0x4022140000,     0x244280000,      0x20408500000,    0x2040810200000,  0x4081020400000,
    0x10080402000200, 0x20100804000400, 0x4020100A000A00, 0x402214001400,   0x24428002800,    0x2040850005000,  0x4081020002000,  0x8102040004000,
    0x8040200020400,  0x10080400040800, 0x20100A000A1000, 0x40221400142200, 0x2442800284400,  0x4085000500800,  0x8102000201000,  0x10204000402000,
    0x4020002040800,  0x8040004081000,  0x100A000A102000, 0x22140014224000, 0x44280028440200, 0x8500050080400,  0x10200020100800, 0x20400040201000,
    0x2000204081000,  0x4000408102000,  0xA000A10204000,  0x14001422400000, 0x28002844020000, 0x50005008040200, 0x20002010080400, 0x40004020100800,
    0x20408102000,    0x40810204000,    0xA1020400000,    0x142240000000,   0x284402000000,   0x500804020000,   0x201008040200,   0x402010080400,
    0x2040810204000,  0x4081020400000,  0xA102040000000,  0x14224000000000, 0x28440200000000, 0x50080402000000, 0x20100804020000, 0x40201008040200,
];

#[cfg(not(feature = "low_memory"))]
const ROOK_OCCUPANCY_MASK: [u64; Square::COUNT] = [
    0x101010101017E,    0x202020202027C,    0x404040404047A,    0x8080808080876,    0x1010101010106E,   0x2020202020205E,   0x4040404040403E,   0x8080808080807E,
    0x1010101017E00,    0x2020202027C00,    0x4040404047A00,    0x8080808087600,    0x10101010106E00,   0x20202020205E00,   0x40404040403E00,   0x80808080807E00,
    0x10101017E0100,    0x20202027C0200,    0x40404047A0400,    0x8080808760800,    0x101010106E1000,   0x202020205E2000,   0x404040403E4000,   0x808080807E8000,
    0x101017E010100,    0x202027C020200,    0x404047A040400,    0x8080876080800,    0x1010106E101000,   0x2020205E202000,   0x4040403E404000,   0x8080807E808000,
    0x1017E01010100,    0x2027C02020200,    0x4047A04040400,    0x8087608080800,    0x10106E10101000,   0x20205E20202000,   0x40403E40404000,   0x80807E80808000,
    0x17E0101010100,    0x27C0202020200,    0x47A0404040400,    0x8760808080800,    0x106E1010101000,   0x205E2020202000,   0x403E4040404000,   0x807E8080808000,
    0x7E010101010100,   0x7C020202020200,   0x7A040404040400,   0x76080808080800,   0x6E101010101000,   0x5E202020202000,   0x3E404040404000,   0x7E808080808000,
    0x7E01010101010100, 0x7C02020202020200, 0x7A04040404040400, 0x7608080808080800, 0x6E10101010101000, 0x5E20202020202000, 0x3E40404040404000, 0x7E80808080808000,
];


#[cfg(not(feature = "low_memory"))]
lazy_static! {
    // Rook magic bitboard attack table
    static ref ROOK_BLOCKED_ATTACKS: Box<[[u64; magic_numbers::MAX_ROOK_VARIATIONS]; Square::COUNT]> = {
        let mut attack_table = box [[0u64; magic_numbers::MAX_ROOK_VARIATIONS]; Square::COUNT];
        // Iterate through each square
        for i in 0..Square::COUNT {
            // Create mask from the index
            let from_mask = 1u64 << i;
            // Get relevant occupancy mask for current square
            let occupancy_mask = ROOK_OCCUPANCY_MASK[i];
            // Get how many blockers (index bits) can exist for square
            let blocker_count = magic_numbers::ROOK_BLOCKER_COUNTS[i];

            // Iterate through all possible blocker arrangements for current square
            // or 1 << 9 store the moves for those blockers
            for blocker_index in 0..(1u16 << blocker_count) {
                // Get blocker mask from an index
                let blocker_mask = magic_numbers::map_blocker_index_to_occupancy_mask(blocker_index, occupancy_mask);
                // Get attacks mask
                let attack_mask = Bitboard::cardinal_ray_attacks(from_mask, !blocker_mask);
                // Store attack in magic table
                attack_table[i][blocker_index as usize] = attack_mask;
            }
        }

        attack_table
    };

    // Generate table of all attacks for blocker arrangements
    static ref BISHOP_BLOCKED_ATTACKS: Box<[[u64; magic_numbers::MAX_BISHOP_VARIATIONS]; Square::COUNT]> = {
        let mut attack_table = box [[0u64; magic_numbers::MAX_BISHOP_VARIATIONS]; Square::COUNT];
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
                attack_table[i][blocker_index as usize] = attack_mask;
            }
        }

        attack_table
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

    ROOK_BLOCKED_ATTACKS[square_offset][index]
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

    BISHOP_BLOCKED_ATTACKS[square_offset][index]
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