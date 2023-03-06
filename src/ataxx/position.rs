use super::bitboard::BitBoard;
use crate::engine::moves::Move;
use std::fmt::Display;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Side {
    Black,
    White,
}

#[derive(Debug, Copy, Clone)]
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

    pub fn make_move(mv: Move) {
        assert!(mv != Move::null());
        assert!(mv.to != 49)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..49u8 {
            let idx = BitBoard::from_index(i);

            if self.black & idx != BitBoard(0) {
                write!(f, "x")?;
            } else if self.white & idx != BitBoard(0) {
                write!(f, "o")?;
            } else if self.gaps & idx != BitBoard(0) {
                write!(f, "#")?;
            } else {
                write!(f, "-")?;
            }

            if i % 7 == 6 {
                writeln!(f)?;
            }
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
        let p = Position::from_fen(fen).unwrap();
    }
}
