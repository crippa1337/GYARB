use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq)]
struct BitBoard(u64);

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
        BitBoard(!self.0 & ATAXX)
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

const ATAXX: u64 = 0x1ffffffffffff;
const A_FILE: u64 = 0x40810204081;
const H_FILE: u64 = 0x1020408102040;

impl BitBoard {
    #[allow(dead_code)]
    pub fn north(&self) -> BitBoard {
        BitBoard((self.0 << 7) & ATAXX)
    }

    #[allow(dead_code)]
    pub fn south(&self) -> BitBoard {
        BitBoard(self.0 >> 7)
    }

    #[allow(dead_code)]
    pub fn east(&self) -> BitBoard {
        BitBoard((self.0 << 1) & (!A_FILE))
    }

    #[allow(dead_code)]
    pub fn west(&self) -> BitBoard {
        BitBoard((self.0 >> 1) & (!H_FILE))
    }

    pub fn singles(&self) -> BitBoard {
        BitBoard(
            // Vertical
            ((self.0 << 7) | (self.0 >> 7) |
            // Right
            (((self.0 << 1) | (self.0 << 8) | (self.0 >> 6)) & (!A_FILE)) |
            // Left
            (((self.0 << 6) | (self.0 >> 1) | (self.0 >> 8)) & (!H_FILE)))
                & ATAXX,
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
        assert_eq!(!BitBoard(A_FILE), BitBoard(ATAXX) ^ BitBoard(A_FILE));
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
    }
}
