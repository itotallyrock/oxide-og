
// Local imports
use crate::side::Side;
use crate::bitboard::Bitboard;
use crate::square::Square;

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

#[inline]
pub fn pawn_square_reverse_push(side: Side, square: Square) -> Square {
    pawn_square_push(side.opposite(), square)
}

#[inline]
pub fn pawn_square_push(side: Side, square: Square) -> Square {
    if side == Side::WHITE {
        square + 8
    } else {
        square - 8
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::named::*;

    #[test]
    fn pawn_pushes_works() {
        assert_eq!(pawn_pushes(Side::WHITE, 0x4000000), 0x400000000);
        assert_eq!(pawn_pushes(Side::BLACK, 0x40000000), 0x400000);
        assert_eq!(pawn_pushes(Side::WHITE, 0x401000000000), 0x40100000000000);
        assert_eq!(pawn_pushes(Side::BLACK, 0x40100000000000), 0x401000000000);
        assert_eq!(pawn_pushes(Side::WHITE, 0xff00), 0xff0000);
        assert_eq!(pawn_pushes(Side::WHITE, 0xff0000), 0xff000000);
        assert_eq!(pawn_pushes(Side::BLACK, 0xff000000000000), 0xff0000000000);
        assert_eq!(pawn_pushes(Side::BLACK, 0xff0000000000), 0xff00000000);
    }

    #[test]
    fn west_attacks_works() {
        assert_eq!(west_attacks(Side::WHITE, 0x10), 0x800);
        assert_eq!(west_attacks(Side::BLACK, 0x800), 0x4);
        assert_eq!(west_attacks(Side::WHITE, 0x2044000000), 0x102200000000);
        assert_eq!(west_attacks(Side::BLACK, 0x2044000000), 0x10220000);
        assert_eq!(west_attacks(Side::WHITE, 0xff00), 0x7f0000);
        assert_eq!(west_attacks(Side::BLACK, 0xff000000000000), 0x7f0000000000);
    }

    #[test]
    fn east_attacks_works() {
        assert_eq!(east_attacks(Side::WHITE, 0x10), 0x2000);
        assert_eq!(east_attacks(Side::BLACK, 0x2000), 0x40);
        assert_eq!(east_attacks(Side::WHITE, 0x2044000000), 0x408800000000);
        assert_eq!(east_attacks(Side::BLACK, 0x2044000000), 0x40880000);
        assert_eq!(east_attacks(Side::WHITE, 0xff00), 0xfe0000);
        assert_eq!(east_attacks(Side::BLACK, 0xff000000000000), 0xfe0000000000);
    }

    #[test]
    fn pawn_attacks_works() {
        assert_eq!(pawn_attacks(Side::WHITE, 0x8000000), 0x1400000000);
        assert_eq!(pawn_attacks(Side::BLACK, 0x8000000), 0x140000);
        assert_eq!(pawn_attacks(Side::WHITE, 0x201004400000), 0x50280aa0000000);
        assert_eq!(pawn_attacks(Side::BLACK, 0x201004400000), 0x50280aa000);
        assert_eq!(pawn_attacks(Side::WHITE, 0xff00), 0xff0000);
        assert_eq!(pawn_attacks(Side::BLACK, 0xff000000000000), 0xff0000000000);
    }

    #[test]
    fn pawn_square_reverse_push_works() {
        assert_eq!(pawn_square_reverse_push(Side::WHITE, A2), A1);
        assert_eq!(pawn_square_reverse_push(Side::BLACK, A2), A3);
        assert_eq!(pawn_square_reverse_push(Side::WHITE, C4), C3);
        assert_eq!(pawn_square_reverse_push(Side::BLACK, C4), C5);
        assert_eq!(pawn_square_reverse_push(Side::WHITE, H8), H7);
        assert_eq!(pawn_square_reverse_push(Side::BLACK, H1), H2);
    }

    #[test]
    fn pawn_square_push_works() {
        assert_eq!(pawn_square_push(Side::WHITE, A2), A3);
        assert_eq!(pawn_square_push(Side::BLACK, A2), A1);
        assert_eq!(pawn_square_push(Side::WHITE, H4), H5);
        assert_eq!(pawn_square_push(Side::BLACK, H5), H4);
        assert_eq!(pawn_square_push(Side::WHITE, C2), C3);
        assert_eq!(pawn_square_push(Side::BLACK, C4), C3);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[cfg(should_panic)]
    fn pawn_square_reverse_push_white_offboard_panics() {
        pawn_square_reverse_push(Side::WHITE, D1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[cfg(should_panic)]
    fn pawn_square_reverse_push_black_offboard_panics() {
        pawn_square_reverse_push(Side::BLACK, D8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[cfg(should_panic)]
    fn pawn_square_push_white_offboard_panics() {
        pawn_square_push(Side::WHITE, C8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[cfg(should_panic)]
    fn pawn_square_push_black_offboard_panics() {
        pawn_square_push(Side::BLACK, C1);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;

    use test::Bencher;
    use super::*;

    #[bench]
    fn white_pawn_attacks_bench(bencher: &mut Bencher) {
        let side = test::black_box(Side::WHITE);
        let from_mask = test::black_box(0x2040140000u64);
        bencher.iter(|| pawn_attacks(side, from_mask));
    }

    #[bench]
    fn white_pawn_pushes_bench(bencher: &mut Bencher) {
        let side = test::black_box(Side::WHITE);
        let from_mask = test::black_box(0x2040140000u64);
        bencher.iter(|| pawn_pushes(side, from_mask));
    }

    #[bench]
    fn black_pawn_attacks_bench(bencher: &mut Bencher) {
        let side = test::black_box(Side::BLACK);
        let from_mask = test::black_box(0x2040140000u64);
        bencher.iter(|| pawn_attacks(side, from_mask));
    }

    #[bench]
    fn black_pawn_pushes_bench(bencher: &mut Bencher) {
        let side = test::black_box(Side::BLACK);
        let from_mask = test::black_box(0x2040140000u64);
        bencher.iter(|| pawn_pushes(side, from_mask));
    }
}
