
// Local imports
use crate::square::masks::*;

// Bitboard shifts, fills, occluded fills, and ray attacks
pub trait Bitboard {
    // Shifts
    fn north_shift(self) -> Self;
    fn south_shift(self) -> Self;
    fn east_shift(self) -> Self;
    fn west_shift(self) -> Self;
    fn north_east_shift(self) -> Self;
    fn north_west_shift(self) -> Self;
    fn south_east_shift(self) -> Self;
    fn south_west_shift(self) -> Self;
    // Fills
    fn file_fill(self) -> Self;
    fn rank_fill(self) -> Self;
    fn south_fill(self) -> Self;
    fn north_fill(self) -> Self;
    fn east_fill(self) -> Self;
    fn west_fill(self) -> Self;
    fn north_west_fill(self) -> Self;
    fn north_east_fill(self) -> Self;
    fn south_west_fill(self) -> Self;
    fn south_east_fill(self) -> Self;
    fn cardinal_fill(self) -> Self;
    fn diagonal_fill(self) -> Self;
    // Occluded Fills
    fn south_occluded_fill(self, empty: Self) -> Self;
    fn north_occluded_fill(self, empty: Self) -> Self;
    fn east_occluded_fill(self, empty: Self) -> Self;
    fn west_occluded_fill(self, empty: Self) -> Self;
    fn north_east_occluded_fill(self, empty: Self) -> Self;
    fn north_west_occluded_fill(self, empty: Self) -> Self;
    fn south_east_occluded_fill(self, empty: Self) -> Self;
    fn south_west_occluded_fill(self, empty: Self) -> Self;
    // Ray attacks
    fn south_ray_attacks(rooks: Self, empty: Self) -> Self;
    fn north_ray_attacks(rooks: Self, empty: Self) -> Self;
    fn east_ray_attacks(rooks: Self, empty: Self) -> Self;
    fn west_ray_attacks(rooks: Self, empty: Self) -> Self;
    fn north_west_ray_attacks(bishops: Self, empty: Self) -> Self;
    fn north_east_ray_attacks(bishops: Self, empty: Self) -> Self;
    fn south_west_ray_attacks(bishops: Self, empty: Self) -> Self;
    fn south_east_ray_attacks(bishops: Self, empty: Self) -> Self;
    fn cardinal_ray_attacks(rooks: Self, empty: Self) -> Self;
    fn diagonal_ray_attacks(bishops: Self, empty: Self) -> Self;
}

// Macros for reducing code duplications in occluded fills
+9macro_rules! fill_masked {
    ($mask:ident, $column_mask:ident << $coefficient:expr) => {{
        const MASK_1: u64 = $column_mask & ($column_mask << $coefficient);
        const MASK_2: u64  = MASK_1 & (MASK_1 << (2 * $coefficient));
        $mask |= $column_mask & ($mask << $coefficient);
        $mask |= MASK_1 & ($mask << (2 * $coefficient));
        $mask |= MASK_2 & ($mask << (4 * $coefficient));

        return $mask;
    }};
    ($mask:ident, $column_mask:ident >> $coefficient:expr) => {{
        const MASK_1: u64 = $column_mask & ($column_mask >> $coefficient);
        const MASK_2: u64  = MASK_1 & (MASK_1 >> (2 * $coefficient));
        $mask |= $column_mask & ($mask >> $coefficient);
        $mask |= MASK_1 & ($mask >> (2 * $coefficient));
        $mask |= MASK_2 & ($mask >> (4 * $coefficient));

        return $mask;
    }};
}

macro_rules! fill_occluded_mask {
    ($mask:ident, $empty:ident, $column_mask:ident << $coefficient:expr) => {{
        $empty  &= $column_mask;
        $mask   |= $empty & ($mask  << $coefficient);
        $empty  &=          ($empty << $coefficient);
        $mask   |= $empty & ($mask  << (2 * $coefficient));
        $empty  &=          ($empty << (2 * $coefficient));
        $mask   |= $empty & ($mask  << (4 * $coefficient));

        return $mask;
    }};
    ($mask:ident, $empty:ident, $column_mask:ident >> $coefficient:expr) => {{
        $empty  &= $column_mask;
        $mask   |= $empty & ($mask  >> $coefficient);
        $empty  &=          ($empty >> $coefficient);
        $mask   |= $empty & ($mask  >> (2 * $coefficient));
        $empty  &=          ($empty >> (2 * $coefficient));
        $mask   |= $empty & ($mask  >> (4 * $coefficient));

        return $mask;
    }};
}

impl const Bitboard for u64 {
    // Shifts
    fn north_shift(self) -> Self {
        self << 8
    }
    #[inline]
    fn south_shift(self) -> Self {
        self >> 8
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

    // Fills
    #[inline]
    fn file_fill(self) -> Self {
        Self::south_fill(self) | Self::north_fill(self)
    }
    #[inline]
    fn rank_fill(self) -> Self {
        Self::east_fill(self) | Self::west_fill(self)
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
        fill_masked!(self, NOT_A_FILE << 1);
    }
    #[inline]
    fn west_fill(mut self) -> Self {
        fill_masked!(self, NOT_H_FILE >> 1);
    }
    #[inline]
    fn north_west_fill(mut self) -> Self {
        fill_masked!(self, NOT_H_FILE << 7);
    }
    #[inline]
    fn north_east_fill(mut self) -> Self {
        fill_masked!(self, NOT_A_FILE << 9);
    }
    #[inline]
    fn south_west_fill(mut self) -> Self {
        fill_masked!(self, NOT_H_FILE >> 9);
    }
    #[inline]
    fn south_east_fill(mut self) -> Self {
        fill_masked!(self, NOT_A_FILE >> 7);
    }
    #[inline]
    fn cardinal_fill(self) -> Self {
        self.north_fill() | self.south_fill() | self.east_fill() | self.west_fill()
    }
    #[inline]
    fn diagonal_fill(self) -> Self {
        self.north_east_fill() | self.north_west_fill() | self.south_east_fill() | self.south_west_fill()
    }
    // Occluded fills
    #[inline]
    fn south_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, ALL >> 8);
    }
    #[inline]
    fn north_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, ALL << 8);
    }
    #[inline]
    fn east_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, NOT_A_FILE << 1);
    }
    #[inline]
    fn west_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, NOT_H_FILE >> 1);
    }
    #[inline]
    fn north_east_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, NOT_A_FILE << 9);
    }
    #[inline]
    fn north_west_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, NOT_H_FILE << 7);
    }
    #[inline]
    fn south_east_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, NOT_A_FILE >> 7);
    }
    #[inline]
    fn south_west_occluded_fill(mut self, mut empty: Self) -> Self {
        fill_occluded_mask!(self, empty, NOT_H_FILE >> 9);
    }

    // Ray attacks
    #[inline]
    fn south_ray_attacks(rooks: Self, empty: Self) -> Self {
        Self::south_shift(Self::south_occluded_fill(rooks, empty))
    }
    #[inline]
    fn north_ray_attacks(rooks: Self, empty: Self) -> Self {
        Self::north_shift(Self::north_occluded_fill(rooks, empty))
    }
    #[inline]
    fn east_ray_attacks(rooks: Self, empty: Self) -> Self {
        Self::east_shift(Self::east_occluded_fill(rooks, empty))
    }
    #[inline]
    fn west_ray_attacks(rooks: Self, empty: Self) -> Self {
        Self::west_shift(Self::west_occluded_fill(rooks, empty))
    }
    #[inline]
    fn north_west_ray_attacks(bishops: Self, empty: Self) -> Self {
        Self::north_west_shift(Self::north_west_occluded_fill(bishops, empty))
    }
    #[inline]
    fn north_east_ray_attacks(bishops: Self, empty: Self) -> Self {
        Self::north_east_shift(Self::north_east_occluded_fill(bishops, empty))
    }
    #[inline]
    fn south_west_ray_attacks(bishops: Self, empty: Self) -> Self {
        Self::south_west_shift(Self::south_west_occluded_fill(bishops, empty))
    }
    #[inline]
    fn south_east_ray_attacks(bishops: Self, empty: Self) -> Self {
        Self::south_east_shift(Self::south_east_occluded_fill(bishops, empty))
    }
    #[inline]
    fn cardinal_ray_attacks(rooks: Self, empty: Self) -> Self {
        Self::north_ray_attacks(rooks, empty) | Self::south_ray_attacks(rooks, empty) | Self::east_ray_attacks(rooks, empty) | Self::west_ray_attacks(rooks, empty)
    }
    #[inline]
    fn diagonal_ray_attacks(bishops: Self, empty: Self) -> Self {
        Self::north_west_ray_attacks(bishops, empty) | Self::north_east_ray_attacks(bishops, empty) | Self::south_west_ray_attacks(bishops, empty) | Self::south_east_ray_attacks(bishops, empty)
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

    #[test]
    fn rank_fill_works() {
        assert_eq!(Bitboard::rank_fill(0xff), 0xff);
        assert_eq!(Bitboard::rank_fill(0x1008), 0xffff);
        assert_eq!(Bitboard::rank_fill(0x4404004001041050), 0xffff00ffffffffff);
        assert_eq!(Bitboard::rank_fill(0x28200200000), 0xffff00ff0000);
    }

    // Ray attacks
    #[test]
    fn south_ray_attacks_works() {
        assert_eq!(Bitboard::south_ray_attacks(0x2000000000, 0xffffffdfffffffff), 0x20202020);
        assert_eq!(Bitboard::south_ray_attacks(0x40020000000, 0xfffffbffdfffffff), 0x404242424);
        assert_eq!(Bitboard::south_ray_attacks(0x2000400200000, 0xfffdfdfbffdffbdf), 0x20004042420);
        assert_eq!(Bitboard::south_ray_attacks(0x2000400200000, 0xfff9fffbfbdeffdf), 0x20206022222);
    }
    #[test]
    fn north_ray_attacks_works() {
        assert_eq!(Bitboard::north_ray_attacks(0x400, 0xfffffffffffffbff), 0x404040404040000);
        assert_eq!(Bitboard::north_ray_attacks(0x42000, 0xfffffffffffbdfff), 0x2424242424200000);
        assert_eq!(Bitboard::north_ray_attacks(0x4200100, 0xfbffdffffbdefeff), 0x404242420010000);
    }
    #[test]
    fn east_ray_attacks_works() {
        assert_eq!(Bitboard::east_ray_attacks(0x10000000000000, 0xffefffffffffffff), 0xe0000000000000);
        assert_eq!(Bitboard::east_ray_attacks(0x4000000080000, 0xfffbfffffff7ffff), 0xf8000000f00000);
        assert_eq!(Bitboard::east_ray_attacks(0x10080000100000, 0xffefe7ffff6fffff), 0xe0100000e00000);
        assert_eq!(Bitboard::east_ray_attacks(0x20000800000400, 0xdfdfffb7ffffebfb), 0xc0007000001800);
    }
    #[test]
    fn west_ray_attacks_works() {
        assert_eq!(Bitboard::west_ray_attacks(0x200000000000, 0xffffdfffffffffff), 0x1f0000000000);
        assert_eq!(Bitboard::west_ray_attacks(0x8000000400000, 0xfff7ffffffbfffff), 0x70000003f0000);
        assert_eq!(Bitboard::west_ray_attacks(0x800000000202000, 0xf6ffffffffcfdbff), 0x700000000101c00);
        assert_eq!(Bitboard::west_ray_attacks(0x20000400200000, 0xffdfeffaffd7ffff), 0x1f000300180000);
    }
    #[test]
    fn north_west_ray_attacks_works() {
        assert_eq!(Bitboard::north_west_ray_attacks(0x2000000000, 0xffffffdfffffffff), 0x408100000000000);
        assert_eq!(Bitboard::north_west_ray_attacks(0x2000080000, 0xffffffdffff7ffff), 0x408110204000000);
        assert_eq!(Bitboard::north_west_ray_attacks(0x40100000001000, 0xdfbfeffdffffefff), 0x2408000204080000);
        assert_eq!(Bitboard::north_west_ray_attacks(0x40100000001000, 0xbfbfefdffbffe7ff), 0x2408000004080000);
    }
    #[test]
    fn north_east_ray_attacks_works() {
        assert_eq!(Bitboard::north_east_ray_attacks(0x10000000, 0xffffffffefffffff), 0x80402000000000);
        assert_eq!(Bitboard::north_east_ray_attacks(0x200200000, 0xfffffffdffdfffff), 0x1008048040000000);
        assert_eq!(Bitboard::north_east_ray_attacks(0x40000042000, 0xeffffbeffffbdfff), 0x1008001088400000);
        assert_eq!(Bitboard::north_east_ray_attacks(0x20008000010, 0xffffd9fff7bfffcf), 0x804201000402000);
    }
    #[test]
    fn south_west_ray_attacks_works() {
        assert_eq!(Bitboard::south_west_ray_attacks(0x1000000000, 0xffffffefffffffff), 0x8040201);
        assert_eq!(Bitboard::south_west_ray_attacks(0x8000000100000, 0xfff7ffffffefffff), 0x40201000804);
        assert_eq!(Bitboard::south_west_ray_attacks(0x4002000002000, 0xfffbffdffff7dfef), 0x20110080010);
        assert_eq!(Bitboard::south_west_ray_attacks(0x4000088000000, 0xfffbfbff777fddff), 0x20100442200);
    }
    #[test]
    fn south_east_ray_attacks_works() {
        assert_eq!(Bitboard::south_east_ray_attacks(0x1000000000, 0xffffffefffffffff), 0x20408000);
        assert_eq!(Bitboard::south_east_ray_attacks(0x100200000000, 0xffffeffdffffffff), 0x2044881020);
        assert_eq!(Bitboard::south_east_ray_attacks(0x20040004000000, 0xffdffb7ffbdfffff), 0x408810281020);
        assert_eq!(Bitboard::south_east_ray_attacks(0x220000020000, 0xffffdddfffd9dfff), 0x4488102408);
    }
    #[test]
    fn cardinal_ray_attacks_works() {
        assert_eq!(Bitboard::cardinal_ray_attacks(0x200000000000, 0xffffdfffffffffff), 0x2020df2020202020);
        assert_eq!(Bitboard::cardinal_ray_attacks(0x200000040000, 0xffffdffffffbffff), 0x2424df2424fb2424);
        assert_eq!(Bitboard::cardinal_ray_attacks(0x200204000000, 0xffdddff5ebfffbff), 0x426df2d3b262622);
        assert_eq!(Bitboard::cardinal_ray_attacks(0x40000200100000, 0xffb7bff5fecfffff), 0x52ba521d122f1212);
    }
    #[test]
    fn diagonal_ray_attacks_works() {
        assert_eq!(Bitboard::diagonal_ray_attacks(0x80000000000, 0xfffff7ffffffffff), 0x2214001422418000);
        assert_eq!(Bitboard::diagonal_ray_attacks(0x80000400000, 0xfffff7ffffbfffff), 0x22140814a241a010);
        assert_eq!(Bitboard::diagonal_ray_attacks(0x42000002000, 0xffeffbdfeeffdfdf), 0x158b520ed9d00050);
        assert_eq!(Bitboard::diagonal_ray_attacks(0x10000200000080, 0xdfeff3f9fe57ff7f), 0x28002d4085284000);
    }
}

#[cfg(test)]
mod bench {
    // Local imports
    use super::*;

    // External test for benchmarking
    extern crate test;
    use test::Bencher;

    #[bench]
    fn diagonal_ray_attacks_bench(bencher: &mut Bencher) {
        let bishops = test::black_box(0x8000000000000000);
        let empty = 0x7fffffffffffffff;
        bencher.iter(|| Bitboard::diagonal_ray_attacks(bishops, empty));
    }

    #[bench]
    fn cardinal_ray_attacks_bench(bencher: &mut Bencher) {
        let rooks = test::black_box(0x100000);
        let empty = 0xffffeffffb6fdfef;
        bencher.iter(|| Bitboard::cardinal_ray_attacks(rooks, empty));
    }

    #[bench]
    fn cardinal_fill_bench(bencher: &mut Bencher) {
        let rooks = test::black_box(0x100000);
        bencher.iter(|| Bitboard::cardinal_fill(rooks));
    }

    #[bench]
    fn diagonal_fill_bench(bencher: &mut Bencher) {
        let bishops = test::black_box(0x8000000000000000);
        bencher.iter(|| Bitboard::diagonal_fill(bishops));
    }

    #[bench]
    fn north_fill_bench(bencher: &mut Bencher) {
        let rooks = test::black_box(0x100000);
        bencher.iter(|| Bitboard::north_fill(rooks));
    }

    #[bench]
    fn south_fill_bench(bencher: &mut Bencher) {
        let rooks = test::black_box(0x100000);
        bencher.iter(|| Bitboard::south_fill(rooks));
    }

    #[bench]
    fn east_fill_bench(bencher: &mut Bencher) {
        let rooks = test::black_box(0x100000);
        bencher.iter(|| Bitboard::east_fill(rooks));
    }

    #[bench]
    fn west_fill_bench(bencher: &mut Bencher) {
        let rooks = test::black_box(0x100000);
        bencher.iter(|| Bitboard::west_fill(rooks));
    }

    #[bench]
    fn north_east_fill_bench(bencher: &mut Bencher) {
        let bishops = test::black_box(0x8000000000000000);
        bencher.iter(|| Bitboard::north_east_fill(bishops));
    }

    #[bench]
    fn north_west_fill_bench(bencher: &mut Bencher) {
        let bishops = test::black_box(0x8000000000000000);
        bencher.iter(|| Bitboard::north_west_fill(bishops));
    }

    #[bench]
    fn south_east_fill_bench(bencher: &mut Bencher) {
        let bishops = test::black_box(0x8000000000000000);
        bencher.iter(|| Bitboard::south_east_fill(bishops));
    }

    #[bench]
    fn south_west_fill_bench(bencher: &mut Bencher) {
        let bishops = test::black_box(0x8000000000000000);
        bencher.iter(|| Bitboard::south_west_fill(bishops));
    }
}

