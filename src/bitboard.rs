
use crate::square::masks::{A_FILE, H_FILE, ALL};

pub trait Bitboard {
    fn south_shift(self) -> Self;
    fn north_shift(self) -> Self;
    fn east_shift(self) -> Self;
    fn west_shift(self) -> Self;
    fn north_east_shift(self) -> Self;
    fn north_west_shift(self) -> Self;
    fn south_east_shift(self) -> Self;
    fn south_west_shift(self) -> Self;

    fn file_fill(self) -> Self;
    fn south_fill(self) -> Self;
    fn north_fill(self) -> Self;
    fn east_fill(self) -> Self;
    fn west_fill(self) -> Self;
    fn north_east_fill(self) -> Self;
    fn north_west_fill(self) -> Self;
    fn south_east_fill(self) -> Self;
    fn south_west_fill(self) -> Self;

    fn south_occluded_fill(self, empty: Self) -> Self;
    fn north_occluded_fill(self, empty: Self) -> Self;
    fn east_occluded_fill(self, empty: Self) -> Self;
    fn west_occluded_fill(self, empty: Self) -> Self;
    fn north_east_occluded_fill(self, empty: Self) -> Self;
    fn north_west_occluded_fill(self, empty: Self) -> Self;
    fn south_east_occluded_fill(self, empty: Self) -> Self;
    fn south_west_occluded_fill(self, empty: Self) -> Self;
}

macro_rules! left_fill_masked {
    ($mask:ident, $column_mask:ident, $coefficient:expr) => {
        let mask_1 = $column_mask & ($column_mask << $coefficient);
        let mask_2 = mask_1 & (mask_1 << (2 * $coefficient));
        $mask |= $column_mask & ($mask << $coefficient);
        $mask |= mask_1 & ($mask << (2 * $coefficient));
        $mask |= mask_2 & ($mask << (4 * $coefficient));
    };
}

macro_rules! right_fill_masked {
    ($mask:ident, $column_mask:ident, $coefficient:expr) => {
        let mask_1 = $column_mask & ($column_mask >> $coefficient);
        let mask_2 = mask_1 & (mask_1 >> (2 * $coefficient));
        $mask |= $column_mask & ($mask >> $coefficient);
        $mask |= mask_1 & ($mask >> (2 * $coefficient));
        $mask |= mask_2 & ($mask >> (4 * $coefficient));
    };
}

macro_rules! left_fill_occluded_mask {
    ($mask:ident, $empty:ident, $column_mask:ident, $coefficient:expr) => {
        $empty  &= $column_mask;
        $mask   |= $empty & ($mask  << $coefficient);
        $empty  &=          ($empty << $coefficient);
        $mask   |= $empty & ($mask  << (2 * $coefficient));
        $empty  &=          ($empty << (2 * $coefficient));
        $mask   |= $empty & ($mask  << (4 * $coefficient));
    };
}

macro_rules! right_fill_occluded_mask {
    ($mask:ident, $empty:ident, $column_mask:ident, $coefficient:expr) => {
        $empty  &= $column_mask;
        $mask   |= $empty & ($mask  >> $coefficient);
        $empty  &=          ($empty >> $coefficient);
        $mask   |= $empty & ($mask  >> (2 * $coefficient));
        $empty  &=          ($empty >> (2 * $coefficient));
        $mask   |= $empty & ($mask  >> (4 * $coefficient));
    };
}

const NOT_A_FILE: u64 = !A_FILE;
const NOT_H_FILE: u64 = !H_FILE;

impl Bitboard for u64 {
    #[inline]
    fn south_shift(self) -> Self {
        self >> 8
    }
    #[inline]
    fn north_shift(self) -> Self {
        self << 8
    }
    #[inline]
    fn east_shift(self) -> Self {
        self << 1 & NOT_A_FILE
    }
    #[inline]
    fn west_shift(self) -> Self {
        self >> 1 & NOT_H_FILE
    }
    #[inline]
    fn north_east_shift(self) -> Self {
        self << 9 & NOT_A_FILE
    }
    #[inline]
    fn north_west_shift(self) -> Self {
        self << 7 & NOT_H_FILE
    }
    #[inline]
    fn south_east_shift(self) -> Self {
        self >> 7 & NOT_A_FILE
    }
    #[inline]
    fn south_west_shift(self) -> Self {
        self >> 9 & NOT_H_FILE
    }
    #[inline]
    fn file_fill(mut self) -> Self {
        self.south_fill() | self.north_fill()
    }
    #[inline]
    fn south_fill(mut self) -> Self {
        self |= self >> 8;
        self |= self >> 16;
        self |= self >> 32;

        self
    }
    #[inline]
    fn north_fill(mut self) -> Self {
        self |= self << 8;
        self |= self << 16;
        self |= self << 32;

        self
    }
    #[inline]
    fn east_fill(mut self) -> Self {
        left_fill_masked!(self, NOT_A_FILE, 1);
        self
    }
    #[inline]
    fn west_fill(mut self) -> Self {
        right_fill_masked!(self, NOT_H_FILE, 1);
        self
    }
    #[inline]
    fn north_east_fill(mut self) -> Self {
        left_fill_masked!(self, NOT_A_FILE, 9);
        self
    }
    #[inline]
    fn north_west_fill(mut self) -> Self {
        left_fill_masked!(self, NOT_H_FILE, 7);
        self
    }
    #[inline]
    fn south_east_fill(mut self) -> Self {
        right_fill_masked!(self, NOT_A_FILE, 7);
        self
    }
    #[inline]
    fn south_west_fill(mut self) -> Self {
        right_fill_masked!(self, NOT_H_FILE, 9);
        self
    }
    #[inline]
    fn south_occluded_fill(mut self, mut empty: Self) -> Self {
        right_fill_occluded_mask!(self, empty, ALL, 8);
        self
    }
    #[inline]
    fn north_occluded_fill(mut self, mut empty: Self) -> Self {
        left_fill_occluded_mask!(self, empty, ALL, 8);
        self
    }
    #[inline]
    fn east_occluded_fill(mut self, mut empty: Self) -> Self {
        left_fill_occluded_mask!(self, empty, NOT_A_FILE, 1);
        self
    }
    #[inline]
    fn west_occluded_fill(mut self, mut empty: Self) -> Self {
        right_fill_occluded_mask!(self, empty, NOT_H_FILE, 1);
        self
    }
    #[inline]
    fn north_east_occluded_fill(mut self, mut empty: Self) -> Self {
        left_fill_occluded_mask!(self, empty, NOT_A_FILE, 9);
        self
    }
    #[inline]
    fn north_west_occluded_fill(mut self, mut empty: Self) -> Self {
        left_fill_occluded_mask!(self, empty, NOT_H_FILE, 7);
        self
    }
    #[inline]
    fn south_east_occluded_fill(mut self, mut empty: Self) -> Self {
        right_fill_occluded_mask!(self, empty, NOT_A_FILE, 7);
        self
    }
    #[inline]
    fn south_west_occluded_fill(mut self, mut empty: Self) -> Self {
        right_fill_occluded_mask!(self, empty, NOT_H_FILE, 9);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shift_north_works() {
        assert_eq!(Bitboard::north_shift(0x3040a1024408800), 0x40a102440880000);
        assert_eq!(Bitboard::north_shift(0xffffffffffffffff), 0xffffffffffffff00);
        assert_eq!(Bitboard::north_shift(0x0), 0x0);
    }

    #[test]
    fn shift_south_works() {
        assert_eq!(Bitboard::south_shift(0x3040a1024408800), 0x3040a10244088);
        assert_eq!(Bitboard::south_shift(0xffffffffffffffff), 0xffffffffffffff);
        assert_eq!(Bitboard::south_shift(0x0), 0x0);
    }

    #[test]
    fn shift_east_works() {

        assert_eq!(Bitboard::east_shift(0x3040a1024408800), 0x608142048801000);
        assert_eq!(Bitboard::east_shift(0xffffffffffffffff), 0xfefefefefefefefe);
        assert_eq!(Bitboard::east_shift(0x0), 0x0);
    }

    #[test]
    fn shift_west_works() {
        assert_eq!(Bitboard::west_shift(0x3040a1024408800), 0x102050812204400);
        assert_eq!(Bitboard::west_shift(0xffffffffffffffff), 0x7f7f7f7f7f7f7f7f);
        assert_eq!(Bitboard::west_shift(0x0), 0x0);
    }

    #[test]
    fn shift_north_east_works() {
        assert_eq!(Bitboard::north_east_shift(0x3040a1024408800), 0x814204880100000);
        assert_eq!(Bitboard::north_east_shift(0xffffffffffffffff), 0xfefefefefefefe00);
        assert_eq!(Bitboard::north_east_shift(0x0), 0x0);
    }

    #[test]
    fn shift_north_west_works() {
        assert_eq!(Bitboard::north_west_shift(0x3040a1024408800), 0x205081220440000);
        assert_eq!(Bitboard::north_west_shift(0xffffffffffffffff), 0x7f7f7f7f7f7f7f00);
        assert_eq!(Bitboard::north_west_shift(0x0), 0x0);
    }

    #[test]
    fn shift_south_east_works() {
        assert_eq!(Bitboard::south_east_shift(0x3040a1024408800), 0x6081420488010);
        assert_eq!(Bitboard::south_east_shift(0xffffffffffffffff), 0xfefefefefefefe);
        assert_eq!(Bitboard::south_east_shift(0x0), 0x0);
    }

    #[test]
    fn shift_south_west_works() {
        assert_eq!(Bitboard::south_west_shift(0x3040a1024408800), 0x1020508122044);
        assert_eq!(Bitboard::south_west_shift(0xffffffffffffffff), 0x7f7f7f7f7f7f7f);
        assert_eq!(Bitboard::south_west_shift(0x0), 0x0);
    }

    #[test]
    fn fill_north_works() {
        assert_eq!(Bitboard::north_fill(0x3040a1024408800), 0xfffefefcecc88800);
        assert_eq!(Bitboard::north_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::north_fill(0x0), 0x0);
    }

    #[test]
    fn fill_south_works() {
        assert_eq!(Bitboard::south_fill(0x3040a1024408800), 0x3070f1f3f7fffff);
        assert_eq!(Bitboard::south_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::south_fill(0x0), 0x0);
    }

    #[test]
    fn fill_east_works() {
        assert_eq!(Bitboard::east_fill(0x3040a1024408800), 0xfffcfef0fcc0f800);
        assert_eq!(Bitboard::east_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::east_fill(0x0), 0x0);
    }

    #[test]
    fn fill_west_works() {
        assert_eq!(Bitboard::west_fill(0x3040a1024408800), 0x3070f1f3f7fff00);
        assert_eq!(Bitboard::west_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::west_fill(0x0), 0x0);
    }

    #[test]
    fn fill_north_east_works() {
        assert_eq!(Bitboard::north_east_fill(0x3040a1024408800), 0xeb74ba58a4508800);
        assert_eq!(Bitboard::north_east_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::north_east_fill(0x0), 0x0);
    }

    #[test]
    fn fill_north_west_works() {
        assert_eq!(Bitboard::north_west_fill(0x3040a1024408800), 0x3050b1326448800);
        assert_eq!(Bitboard::north_west_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::north_west_fill(0x0), 0x0);
    }

    #[test]
    fn fill_south_east_works() {
        assert_eq!(Bitboard::south_east_fill(0x3040a1024408800), 0x3060e1c3c78f8f0);
        assert_eq!(Bitboard::south_east_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::south_east_fill(0x0), 0x0);
    }

    #[test]
    fn fill_south_west_works() {
        assert_eq!(Bitboard::south_west_fill(0x3040a1024408800), 0x3050a152e57ab55);
        assert_eq!(Bitboard::south_west_fill(0xffffffffffffffff), 0xffffffffffffffff);
        assert_eq!(Bitboard::south_west_fill(0x0), 0x0);
    }

    #[test]
    fn fill_occluded_north_works() {
        assert_eq!(Bitboard::north_occluded_fill(0x100020004000800, 0xfcfbf5efdbbf77ff), 0x102060c0c080800);
        assert_eq!(Bitboard::north_occluded_fill(0x8142242418000000, 0x5abdc3dbe7ffffff), 0xc366243c18000000);
        assert_eq!(Bitboard::north_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::north_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn fill_occluded_south_works() {
        assert_eq!(Bitboard::south_occluded_fill(0x204081020408000, 0xfcfbf5efdbbf77ff), 0x2060c1c3878f0f0);
        assert_eq!(Bitboard::south_occluded_fill(0x8142242418000000, 0x7ebd5bdbe7bfc7fb), 0x81c367677f3f0703);
        assert_eq!(Bitboard::south_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::south_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn fill_occluded_east_works() {
        assert_eq!(Bitboard::east_occluded_fill(0x100020004000800, 0xfcfbf5efdbbf77ff), 0x10006001c007800);
        assert_eq!(Bitboard::east_occluded_fill(0x102040418000000, 0x7ebddbdbe7ffffff), 0x7f3e1c1cf8000000);
        assert_eq!(Bitboard::east_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::east_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn fill_occluded_west_works() {
        assert_eq!(Bitboard::west_occluded_fill(0x204081020408000, 0xfcfbf5efdbbf77ff), 0x2070c1f387ff000);
        assert_eq!(Bitboard::west_occluded_fill(0x8040202018000000, 0x7ebddbdbe7ffffff), 0xfe7c38381f000000);
        assert_eq!(Bitboard::west_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::west_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn fill_occluded_north_east_works() {
        assert_eq!(Bitboard::north_east_occluded_fill(0x100020004000800, 0xfcfbf5efdbbf77ff), 0x4120120804100800);
        assert_eq!(Bitboard::north_east_occluded_fill(0x102040418000000, 0x7ebddbdbe7ffffff), 0x351a0c1418000000);
        assert_eq!(Bitboard::north_east_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::north_east_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn fill_occluded_north_west_works() {
        assert_eq!(Bitboard::north_west_occluded_fill(0x20408000, 0xfefbfdffdbbf77ff), 0x81020408000);
        assert_eq!(Bitboard::north_west_occluded_fill(0x8040202018000000, 0x7ebddbdbe7ffffff), 0xac58302818000000);
        assert_eq!(Bitboard::north_west_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::north_west_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn fill_occluded_south_east_works() {
        assert_eq!(Bitboard::south_east_occluded_fill(0x284582000000000, 0xfd7ba7dfffbbffff), 0x28458b060800000);
        assert_eq!(Bitboard::south_east_occluded_fill(0x102040418000000, 0x7ebddbdbe7ffffff), 0x102040c183060c0);
        assert_eq!(Bitboard::south_east_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::south_east_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn fill_occluded_south_west_works() {
        assert_eq!(Bitboard::south_west_occluded_fill(0x284582000000000, 0xfd7ba7dfffb3ffff), 0x2855a2d16030100);
        assert_eq!(Bitboard::south_west_occluded_fill(0x8040202018000000, 0x7fbfdfdfe3fffdff), 0x80402030180c0402);
        assert_eq!(Bitboard::south_west_occluded_fill(0x0, 0x0), 0x0);
        assert_eq!(Bitboard::south_west_occluded_fill(0x0, 0xffffffffffffffff), 0x0);
    }

    #[test]
    fn file_fill_works() {
        assert_eq!(Bitboard::file_fill(0xff), 0xffffffffffffffff);
        assert_eq!(Bitboard::file_fill(0x55), 0x5555555555555555);
        assert_eq!(Bitboard::file_fill(0x4404004001041050), 0x5555555555555555);
        assert_eq!(Bitboard::file_fill(0x28200200000), 0xa2a2a2a2a2a2a2a2);
    }
}