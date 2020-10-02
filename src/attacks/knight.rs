
// Local imports
use crate::square::masks::*;

#[inline]
pub fn knight_attacks(knight_mask: u64) -> u64 {
    (((knight_mask << 15) | (knight_mask >> 17)) & !H_FILE)
        | (((knight_mask >> 15) | (knight_mask << 17)) & !A_FILE)
        | (((knight_mask << 6) | (knight_mask >> 10)) & !(G_FILE | H_FILE))
        | (((knight_mask >> 6) | (knight_mask << 10)) & !(A_FILE | B_FILE))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knight_attacks_works() {
        assert_eq!(knight_attacks(0x40000000000u64), 0xa1100110a000000u64);
        assert_eq!(knight_attacks(0x2000000000u64), 0x50880088500000u64);
        assert_eq!(knight_attacks(0x80u64), 0x402000u64);
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
        bencher.iter(|| knight_attacks(from_mask));
    }
}
