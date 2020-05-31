
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
        self << 1 & !A_FILE
    }
    #[inline]
    fn west_shift(self) -> Self {
        self >> 1 & NOT_A_FILE
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
        todo!()
    }

    #[test]
    fn shift_east_works() {
        todo!()
    }

    #[test]
    fn shift_west_works() {
        todo!()
    }

    #[test]
    fn shift_north_east_works() {
        todo!()
    }

    #[test]
    fn shift_north_west_works() {
        todo!()
    }

    #[test]
    fn shift_south_east_works() {
        todo!()
    }

    #[test]
    fn shift_south_west_works() {
        todo!()
    }

    #[test]
    fn fill_north_works() {
        todo!()
    }

    #[test]
    fn fill_south_works() {
        todo!()
    }

    #[test]
    fn fill_east_works() {
        todo!()
    }

    #[test]
    fn fill_west_works() {
        todo!()
    }

    #[test]
    fn fill_north_east_works() {
        todo!()
    }

    #[test]
    fn fill_north_west_works() {
        todo!()
    }

    #[test]
    fn fill_south_east_works() {
        todo!()
    }

    #[test]
    fn fill_south_west_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_north_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_south_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_east_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_west_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_north_east_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_north_west_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_south_east_works() {
        todo!()
    }

    #[test]
    fn fill_occluded_south_west_works() {
        todo!()
    }
}