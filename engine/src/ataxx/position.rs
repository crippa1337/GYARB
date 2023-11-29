use super::bitboard::BitBoard;
use std::{cmp::Ordering, fmt::Display, ops::Not};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Side {
    Black,
    White,
}

#[derive(PartialEq)]
pub enum Outcome {
    BlackWin,
    WhiteWin,
    Draw,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Black => Side::White,
            Side::White => Side::Black,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub black: BitBoard,
    pub white: BitBoard,
    pub gaps: BitBoard,
    pub turn: Side,
    pub half_moves: u8,
    pub full_moves: u8,
}

impl Position {
    pub fn empty_squares(&self) -> BitBoard {
        !(self.black | self.white | self.gaps)
    }

    pub fn colored_squares(&self, side: Side) -> BitBoard {
        match side {
            Side::Black => self.black,
            Side::White => self.white,
        }
    }

    pub fn colored_squares_mut(&mut self, side: Side) -> (&mut BitBoard, &mut BitBoard) {
        match side {
            Side::Black => (&mut self.black, &mut self.white),
            Side::White => (&mut self.white, &mut self.black),
        }
    }

    #[allow(dead_code)]
    pub fn default() -> Position {
        Position {
            black: BitBoard(0x40000000040),
            white: BitBoard(0x1000000000001),
            gaps: BitBoard(0),
            turn: Side::Black,
            half_moves: 0,
            full_moves: 1,
        }
    }

    fn both_sides(&self) -> BitBoard {
        self.black | self.white
    }

    pub fn game_over(&self) -> bool {
        self.black.is_empty()
            || self.white.is_empty()
            || self.half_moves >= 100
            || (self.both_sides().reach() & self.empty_squares()).is_empty()
    }

    pub fn winner(&self) -> Option<Outcome> {
        if !self.game_over() {
            return None;
        }

        if self.black.is_empty() {
            return Some(Outcome::WhiteWin);
        } else if self.white.is_empty() {
            return Some(Outcome::BlackWin);
        }

        let black_score = self.black.popcnt();
        let white_score = self.white.popcnt();

        match black_score.cmp(&white_score) {
            Ordering::Greater => Some(Outcome::BlackWin),
            Ordering::Less => Some(Outcome::WhiteWin),
            Ordering::Equal => Some(Outcome::Draw),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 42;
        loop {
            let idx = BitBoard::from_index(i);

            if self.black & idx != BitBoard(0) {
                write!(f, "x")?;
            } else if self.white & idx != BitBoard(0) {
                write!(f, "o")?;
            } else if self.gaps & idx != BitBoard(0) {
                write!(f, " ")?;
            } else {
                write!(f, "-")?;
            }

            if i == 6 {
                break;
            } else if i % 7 == 6 {
                writeln!(f)?;
                i -= 13;
                continue;
            }

            i += 1
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_squares() {
        let fen = "x5o/7/7/7/7/7/o5x x 0 1";
        let p = Position::from_fen(fen).unwrap();
        assert_eq!(p.empty_squares(), BitBoard(0xfbffffffffbe));

        let fen = "7/7/7/7/7/7/7 x 0 1";
        let p = Position::from_fen(fen).unwrap();
        assert_eq!(p.empty_squares(), BitBoard(0x1ffffffffffff));

        let fen = "-5o/7/7/7/7/7/-5x o 0 1";
        let p = Position::from_fen(fen).unwrap();
        assert_eq!(p.empty_squares(), BitBoard(0xfbffffffffbe));
    }

    #[test]
    fn colored_squares() {
        let fen = "x5o/7/7/7/7/7/o5x x 0 1";
        let mut p = Position::from_fen(fen).unwrap();
        let black = p.colored_squares(Side::Black);
        assert_eq!(black, BitBoard(0x40000000040));

        let white = p.colored_squares_mut(Side::White);
        *white.0 &= BitBoard(0);
        assert_eq!(*white.0, BitBoard(0));
    }

    #[test]
    fn default() {
        let fen = "x5o/7/7/7/7/7/o5x x 0 1";
        let p = Position::from_fen(fen).unwrap();
        let d = Position::default();
        assert_eq!(d, p);

        assert_eq!(d.perft(1), 16);
        assert_eq!(d.perft(2), 256);
        assert_eq!(d.perft(3), 6460);
    }

    #[test]
    fn gameover_true() {
        let tests: [&str; 10] = [
            "7/7/7/7/7/7/7 x 0 1",
            "7/7/7/7/7/7/7 o 0 1",
            "7/7/7/7/7/7/x6 x 0 1",
            "7/7/7/7/7/7/x6 o 0 1",
            "7/7/7/7/7/7/o6 x 0 1",
            "7/7/7/7/7/7/o6 o 0 1",
            "x5o/7/7/7/7/7/o5x x 100 1",
            "x5o/7/7/7/7/7/o5x o 100 1",
            "7/7/7/7/-------/-------/ooooxxx x 0 1",
            "7/7/7/7/-------/-------/ooooxxx o 0 1",
        ];

        for fen in tests {
            let pos = Position::from_fen(fen).unwrap();
            assert!(pos.game_over());
        }
    }

    #[test]
    fn gameover_false() {
        let tests: [&str; 10] = [
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/7/7/7/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 20 40",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 20 40",
            "7/7/7/7/ooooooo/ooooooo/xxxxxxx x 0 1",
            "7/7/7/7/ooooooo/ooooooo/xxxxxxx o 0 1",
            "7/7/7/7/xxxxxxx/xxxxxxx/ooooooo x 0 1",
            "7/7/7/7/xxxxxxx/xxxxxxx/ooooooo o 0 1",
        ];

        for fen in tests {
            let pos = Position::from_fen(fen).unwrap();
            assert!(!pos.game_over());
        }
    }
}
