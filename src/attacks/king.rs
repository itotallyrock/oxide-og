
// Local imports
use crate::bitboard::Bitboard;

#[inline]
pub fn king_attacks(mut king_mask: u64) -> u64 {
    let mut attacks = Bitboard::east_shift(king_mask) | Bitboard::west_shift(king_mask);
    king_mask |= attacks;
    attacks |= Bitboard::north_shift(king_mask) | Bitboard::south_shift(king_mask);

    attacks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn king_attacks_works() {
        assert_eq!(king_attacks(1), 0x302u64);
        assert_eq!(king_attacks(0x2000000000u64), 0x705070000000u64);
        assert_eq!(king_attacks(0x80000000000000u64), 0xc040c00000000000u64);
    }
}

#[cfg(test)]
mod bench {
    // Local imports
    use super::*;

    // External test for benchmarking
    extern crate test;
    use test::Bencher;
    use crate::square::named::*;

    #[bench]
    fn knight_attack_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3).mask();
        bencher.iter(|| king_attacks(from_mask));
    }
}