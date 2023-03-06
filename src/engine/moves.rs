use super::statvec::StaticVec;
use crate::ataxx::position::{Position, Side};

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
            let doubles = s2m.doubles() & empty;
            for sq2 in doubles {
                let mv = Move::new(sq, sq2, MoveType::Double);
                moves.push(mv);
            }
        }

        for mv in moves.as_slice() {
            println!("{:?}", mv)
        }

        moves
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
