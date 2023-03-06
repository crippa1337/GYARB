use super::{
    bitboard::BitBoard,
    position::{Position, Side},
};

#[derive(Debug, PartialEq)]
pub enum FenError {
    Illegal,
    Turn,
    HalfMoves,
    FullMoves,
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Position, FenError> {
        let mut black = BitBoard(0);
        let mut white = BitBoard(0);
        let mut gaps = BitBoard(0);
        let fen: Vec<&str> = fen.split(' ').collect();

        if fen.len() != 4 {
            return Err(FenError::Illegal);
        }

        let mut x = 0;
        let mut y = 6;

        for c in fen[0].chars() {
            match c {
                'x' => {
                    black |= BitBoard::from_square(x, y);
                    x += 1;
                }
                'o' => {
                    white |= BitBoard::from_square(x, y);
                    x += 1;
                }
                '-' => {
                    gaps |= BitBoard::from_square(x, y);
                    x += 1;
                }
                z if z.is_ascii_digit() => {
                    x += z.to_digit(10).unwrap() as usize;
                }
                '/' => {
                    assert!(x % 7 == 0);
                    x = 0;
                    y -= 1;
                }
                _ => return Err(FenError::Illegal),
            }
        }

        let turn = match fen[1] {
            "x" => Side::Black,
            "o" => Side::White,
            _ => return Err(FenError::Turn),
        };

        let half_moves = match fen[2].parse::<u8>() {
            Ok(half_moves) => {
                if half_moves > 100 {
                    return Err(FenError::HalfMoves);
                }
                half_moves
            }
            Err(_) => return Err(FenError::HalfMoves),
        };

        let full_moves = match fen[3].parse::<u8>() {
            Ok(full_moves) => full_moves,
            Err(_) => return Err(FenError::FullMoves),
        };

        Ok(Position {
            black,
            white,
            gaps,
            turn,
            half_moves,
            full_moves,
        })
    }

    pub fn get_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty = 0;

        for i in 0..49u8 {
            let idx = BitBoard::from_index(i);

            if self.black & idx != BitBoard(0) {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push('x');
            } else if self.white & idx != BitBoard(0) {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push('o');
            } else if self.gaps & idx != BitBoard(0) {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push('-');
            } else {
                empty += 1;
            }

            // Slash at the end of a row, but not at the end
            if i % 7 == 6 && i != 48 {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push('/');
            }
        }

        if empty > 0 {
            fen.push_str(&empty.to_string());
        }

        fen.push(' ');
        fen.push_str(match self.turn {
            Side::Black => "x",
            Side::White => "o",
        });

        fen.push(' ');
        fen.push_str(&self.half_moves.to_string());

        fen.push(' ');
        fen.push_str(&self.full_moves.to_string());

        fen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_fen() {
        let fen = "x5o/7/7/7/7/7/o5x x 0 1";
        let p = Position::from_fen(fen).unwrap();
        assert_eq!(p.turn, Side::Black);
        assert_eq!(p.black | p.white, BitBoard(0x1040000000041));
        assert_eq!(p.white, BitBoard(0x1000000000001));
        assert_eq!(p.black, BitBoard(0x40000000040));
        assert_eq!(p.gaps, BitBoard(0));
        assert_eq!(p.half_moves, 0);
        assert_eq!(p.full_moves, 1);
        assert_eq!(p.get_fen(), fen);
    }

    #[test]
    fn get_fen() {
        let fens = [
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/3-3/2-1-2/3-3/7/o5x x 0 1",
            "7/7/7/7/7/7/7 x 0 1",
            "7/7/7/7/7/7/7 o 0 1",
            "7/7/7/7/7/7/7 x 100 1",
            "7/7/7/7/7/7/7 o 100 1",
            "7/7/7/7/7/7/7 x 0 100",
            "7/7/7/7/7/7/7 o 0 100",
            "7/7/7/7/7/7/7 x 100 200",
            "7/7/7/7/7/7/7 o 100 200",
        ];

        for fen in fens.iter() {
            let p = Position::from_fen(fen).unwrap();
            assert_eq!(p.get_fen(), *fen);
        }
    }

    #[test]
    fn fen_error() {
        let fens = [
            "x5o/7/7/7/7/7/o5x",
            "x5o/7/7/7/7/7/o5x x",
            "x5o/7/7/7/7/7/o5x x 0",
            "x5o/7/2-1-2/7/2-1-2/7/o5x",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0",
        ];

        for fen in fens.iter() {
            assert!(Position::from_fen(fen).is_err());
        }
    }
}
