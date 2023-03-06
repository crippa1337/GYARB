use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoard(pub u64);

#[derive(Debug, Clone)]
pub struct BitBoardIter(u64);

impl Iterator for BitBoardIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Some(index)
        }
    }
}

impl IntoIterator for BitBoard {
    type Item = u8;
    type IntoIter = BitBoardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter(self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0 & FULL)
    }
}

// Board structure
// 42 43 44 45 46 47 48
// 35 36 37 38 39 40 41
// 28 29 30 31 32 33 34
// 21 22 23 24 25 26 27
// 14 15 16 17 18 19 20
// 07 08 09 10 11 12 13
// 00 01 02 03 04 05 06

pub const FULL: u64 = 0x1ffffffffffff;
const FILE_A: u64 = 0x40810204081;
const FILE_B: u64 = 0x81020408102;
const FILE_C: u64 = 0x102040810204;
const FILE_D: u64 = 0x204081020408;
const FILE_E: u64 = 0x408102040810;
const FILE_F: u64 = 0x10204081020;
const FILE_G: u64 = 0x1020408102040;

const RANK_1: u64 = 0x7f;
const RANK_2: u64 = 0x3f80;
const RANK_3: u64 = 0x1fc000;
const RANK_4: u64 = 0xfe00000;
const RANK_5: u64 = 0x7f0000000;
const RANK_6: u64 = 0x3f800000000;
const RANK_7: u64 = 0x1fc0000000000;

const FILES: [u64; 7] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G];
const RANKS: [u64; 7] = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7];

impl BitBoard {
    pub const fn from_index(sq: u8) -> BitBoard {
        BitBoard(1u64 << sq)
    }

    pub const fn from_square(file: usize, rank: usize) -> BitBoard {
        BitBoard(FILES[file] & RANKS[rank])
    }

    pub const fn popcnt(&self) -> u32 {
        self.0.count_ones()
    }

    pub const fn full() -> BitBoard {
        BitBoard(FULL)
    }

    #[allow(dead_code)]
    pub fn north(&self) -> BitBoard {
        BitBoard((self.0 << 7) & FULL)
    }

    #[allow(dead_code)]
    pub fn south(&self) -> BitBoard {
        BitBoard(self.0 >> 7)
    }

    #[allow(dead_code)]
    pub fn east(&self) -> BitBoard {
        BitBoard((self.0 << 1) & (!FILE_A))
    }

    #[allow(dead_code)]
    pub fn west(&self) -> BitBoard {
        BitBoard((self.0 >> 1) & (!FILE_G))
    }

    pub fn singles(&self) -> BitBoard {
        BitBoard(
            // U             // D
            ((self.0 << 7) | (self.0 >> 7) |
            // R              // RU           // RD
            (((self.0 << 1) | (self.0 << 8) | (self.0 >> 6)) & !FILE_A) |
            // L              // LU           // LD
            (((self.0 >> 1) | (self.0 << 6) | (self.0 >> 8)) & !FILE_G))
                & FULL,
        )
    }

    #[rustfmt::skip]
    pub fn doubles(&self) -> BitBoard {
        BitBoard(
            (
                // UU            // DD
                (self.0 << 14) | (self.0 >> 14) |
                // RUU             // RDD
                (((self.0 << 15) | (self.0 >> 13)) & !FILE_A) |
                // LUU             // LDD
                (((self.0 << 13) | (self.0 >> 15)) & !FILE_G) |
                // RR              // RRUU         // RRDD          // RRU          // RRD
                (((self.0 << 2) | (self.0 << 16) | (self.0 >> 12) | (self.0 << 9) | (self.0 >> 5)) & !(FILE_A | FILE_B)) |
                // LL              // LLUU         // LLDD          // LLU          // LLD
                (((self.0 >> 2) | (self.0 << 12) | (self.0 >> 16) | (self.0 << 5) | (self.0 >> 9)) & !(FILE_F | FILE_G))
            ) & FULL,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitxor() {
        assert!(BitBoard(0) ^ BitBoard(0) == BitBoard(0));
        assert!(BitBoard(1) ^ BitBoard(2) == BitBoard(3));
    }

    #[test]
    fn bitor() {
        assert!(BitBoard(1) | BitBoard(2) == BitBoard(3));
    }

    #[test]
    fn bitand() {
        assert!(BitBoard(1) & BitBoard(2) == BitBoard(0));
    }

    #[test]
    fn bitnot() {
        assert_eq!(!BitBoard(0), BitBoard(0x1ffffffffffff));
        assert_eq!(!BitBoard(0x1ffffffffffff), BitBoard(0));
        assert_eq!(!BitBoard(FILE_A), BitBoard(FULL) ^ BitBoard(FILE_A));
    }

    #[test]
    fn north() {
        assert_eq!(BitBoard(0x20000000).north(), BitBoard(0x1000000000));
        assert_eq!(BitBoard(0x10000000000).north(), BitBoard(0x800000000000));
        assert_eq!(BitBoard(0x800000000000).north(), BitBoard(0));
        assert_eq!(BitBoard(0x0).north(), BitBoard(0x0));
        assert_eq!(BitBoard(0x1).north(), BitBoard(0x80));
        assert_eq!(BitBoard(0x7f).north(), BitBoard(0x3f80));
        assert_eq!(BitBoard(0x3f800000000).north(), BitBoard(0x1fc0000000000));
        assert_eq!(BitBoard(0x1fc0000000000).north(), BitBoard(0x0));
    }

    #[test]
    fn south() {
        assert_eq!(BitBoard(1).south(), BitBoard(0));
        assert_eq!(BitBoard(0x200).south(), BitBoard(4));
        assert_eq!(BitBoard(0x2000000).south(), BitBoard(0x40000));
        assert_eq!(BitBoard(0x0).south(), BitBoard(0x0));
        assert_eq!(BitBoard(0x80).south(), BitBoard(0x1));
        assert_eq!(BitBoard(0x1fc000).south(), BitBoard(0x3f80));
        assert_eq!(BitBoard(0x3f80).south(), BitBoard(0x7f));
        assert_eq!(BitBoard(0x7f).south(), BitBoard(0x0));
    }

    #[test]
    fn east() {
        assert_eq!(BitBoard(1).east(), BitBoard(2));
        assert_eq!(BitBoard(0x8000).east(), BitBoard(0x10000));
        assert_eq!(BitBoard(0x40).east(), BitBoard(0));
        assert_eq!(BitBoard(0x0).east(), BitBoard(0x0));
        assert_eq!(BitBoard(0x1).east(), BitBoard(0x2));
    }

    #[test]
    fn west() {
        assert_eq!(BitBoard(1).west(), BitBoard(0));
        assert_eq!(BitBoard(0x400000).west(), BitBoard(0x200000));
        assert_eq!(BitBoard(0x1000000).west(), BitBoard(0x800000));
        assert_eq!(BitBoard(0x0).west(), BitBoard(0x0));
        assert_eq!(BitBoard(0x1).west(), BitBoard(0x0));
    }

    #[test]
    fn singles() {
        assert_eq!(BitBoard(0x200).singles(), BitBoard(0x3850e));
        assert_eq!(BitBoard(0x0).singles(), BitBoard(0x0));
        assert_eq!(BitBoard(0x1).singles(), BitBoard(0x182));
        assert_eq!(BitBoard(0x100).singles(), BitBoard(0x1c287));
        assert_eq!(
            BitBoard(0x1000000000000).singles(),
            BitBoard(0x830000000000)
        );
        assert_eq!(BitBoard(1).singles(), BitBoard(0x182));
    }

    #[test]
    fn doubles() {
        assert_eq!(BitBoard(0x0).doubles(), BitBoard(0x0));
        assert_eq!(BitBoard(0x1).doubles(), BitBoard(0x1c204));
        assert_eq!(BitBoard(0x100).doubles(), BitBoard(0x1e20408));
        assert_eq!(
            BitBoard(0x400000000000).doubles(),
            BitBoard(0x11227c0000000)
        );
    }

    #[test]
    fn from_index() {
        assert_eq!(BitBoard::from_index(0), BitBoard(1));
        assert_eq!(BitBoard::from_index(25), BitBoard(0x2000000));
        assert_eq!(BitBoard::from_index(47), BitBoard(0x800000000000));
    }

    #[test]
    fn from_square() {
        assert_eq!(BitBoard::from_square(0, 0), BitBoard(1));
        assert_eq!(BitBoard::from_square(3, 3), BitBoard(0x1000000));
    }
}
