use super::statvec::StaticVec;
use crate::ataxx::{
    bitboard::BitBoard,
    position::{Position, Side},
};

const MAX_MOVES: usize = 256;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveType {
    Null,
    Single,
    Double,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(from: u8, to: u8, move_type: MoveType) -> Move {
        Move {
            from,
            to,
            move_type,
        }
    }

    pub fn null() -> Move {
        Move {
            from: 49,
            to: 49,
            move_type: MoveType::Null,
        }
    }
}

impl Position {
    pub fn generate_moves(&self) -> StaticVec<Move, MAX_MOVES> {
        let mut moves: StaticVec<Move, MAX_MOVES> = StaticVec::new(Move::null());
        let s2m = self.colored_squares(self.turn);
        let empty = self.empty_squares();

        let singles = s2m.singles() & empty;
        for sq in singles {
            let mv = Move::new(49, sq, MoveType::Single);
            moves.push(mv);
        }

        for sq in s2m {
            let doubles = BitBoard::from_index(sq).doubles() & empty;
            for sq2 in doubles {
                let mv = Move::new(sq, sq2, MoveType::Double);
                moves.push(mv);
            }
        }

        moves
    }

    pub fn make_move(&mut self, mv: Move) {
        // Info
        self.turn = match self.turn {
            Side::Black => Side::White,
            Side::White => Side::Black,
        };

        self.half_moves += 1;
        if self.turn == Side::White {
            self.full_moves += 1;
        }

        // Move stone
        let from = BitBoard::from_index(mv.from);
        let to = BitBoard::from_index(mv.to);
        let mut s2m = self.colored_squares(self.turn);
        let mut opponent = self.colored_squares(!self.turn);

        match mv.move_type {
            MoveType::Single => {
                s2m |= to;
            }
            MoveType::Double => {
                s2m ^= from;
                s2m |= to;
            }
            MoveType::Null => panic!("Make move called with null move"),
        }

        // Captures
        let captured = to.singles() & opponent;
        opponent ^= captured;
        s2m |= captured;
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
}
