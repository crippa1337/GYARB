use super::statvec::StaticVec;
use crate::ataxx::{
    bitboard::BitBoard,
    position::{Position, Side},
};
use std::fmt::Display;

const MAX_MOVES: usize = 256;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

impl Move {
    pub fn new(from: u8, to: u8) -> Move {
        Move { from, to }
    }

    const fn null() -> Move {
        Move { from: 49, to: 50 }
    }

    const fn pass() -> Move {
        Move { from: 50, to: 51 }
    }

    const fn is_single(&self) -> bool {
        self.from == self.to
    }
}

// Shamelessely stolen from Rustaxx (kz04px)
impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_single() {
            write!(
                f,
                "{}{}",
                (97 + (self.from % 7)) as char,
                (49 + (self.from / 7)) as char
            )
        } else if *self == Move::pass() {
            write!(f, "0000")
        } else {
            write!(
                f,
                "{}{}{}{}",
                (97 + (self.from % 7)) as char,
                (49 + (self.from / 7)) as char,
                (97 + (self.to % 7)) as char,
                (49 + (self.to / 7)) as char
            )
        }
    }
}

impl Position {
    pub fn generate_moves(&self) -> StaticVec<Move, MAX_MOVES> {
        let mut moves: StaticVec<Move, MAX_MOVES> = StaticVec::new(Move::null());
        if self.must_pass() {
            moves.push(Move::pass());
            return moves;
        }

        let s2m = self.colored_squares(self.turn);
        let empty = self.empty_squares();

        let singles = s2m.singles() & empty;
        for sq in singles {
            let mv = Move::new(sq, sq);
            moves.push(mv);
        }

        for sq in s2m {
            let doubles = BitBoard::from_index(sq).doubles() & empty;
            for sq2 in doubles {
                let mv = Move::new(sq, sq2);
                moves.push(mv);
            }
        }

        moves
    }

    pub fn make_move(&mut self, mv: Move) {
        debug_assert!(mv != Move::null());

        // Info
        self.turn = !self.turn;
        self.half_moves += 1;
        if self.turn == Side::White {
            self.full_moves += 1;
        }

        if mv == Move::pass() {
            return;
        }

        // Move stone
        let from = BitBoard::from_index(mv.from);
        let to = BitBoard::from_index(mv.to);
        let (s2m, opponent) = self.colored_squares_mut(self.turn);

        *s2m ^= from | to;

        // Captures
        let captured = to.singles() & *opponent;
        *opponent ^= captured;
        *s2m |= captured;
    }

    fn must_pass(&self) -> bool {
        if self.game_over() {
            return false;
        }

        let s2m = self.colored_squares(self.turn);
        let empty = self.empty_squares();
        s2m.reach() & empty == BitBoard(0)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_moves() {
        let pos = Position::from_fen("x5o/7/7/7/7/7/o5x x 0 1").unwrap();
        let moves = pos.generate_moves();
        assert_eq!(moves.len(), 16);
    }

    #[test]
    fn make_move() {
        let mut pos = Position::from_fen("x5o/7/7/7/7/7/o5x x 0 1").unwrap();
        let mv = Move::new(43, 43);
        pos.make_move(mv);
        assert_eq!(pos.black, BitBoard(0xc0000000040));

        let mv = Move::new(0, 14);
        pos.make_move(mv);
        assert_eq!(pos.white, BitBoard(0x1000000004000));

        let mv = Move::new(42, 28);
        pos.make_move(mv);
        assert_eq!(pos.black, BitBoard(0x80010000040));

        let mv = Move::new(21, 21);
        pos.make_move(mv);
        assert_eq!(pos.white, BitBoard(0x1000010204000));
    }
}
