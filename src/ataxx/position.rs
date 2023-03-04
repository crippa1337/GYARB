use super::bitboard::BitBoard;

#[derive(Debug, PartialEq)]
pub enum Side {
    Black,
    White,
}

pub struct Position {
    pub occupied: BitBoard,
    pub black: BitBoard,
    pub white: BitBoard,
    pub gaps: BitBoard,
    pub turn: Side,
    pub half_moves: u8,
    pub full_moves: u8,
}

impl Position {
    pub fn from_fen(fen: &str) -> Position {
        let mut black = BitBoard(0);
        let mut white = BitBoard(0);
        let mut gaps = BitBoard(0);
        let fen: Vec<&str> = fen.split(' ').collect();

        let mut i = 0;
        for c in fen[0].chars() {
            match c {
                'x' => {
                    black |= BitBoard::from_index(i as u8);
                    i += 1;
                }
                'o' => {
                    white |= BitBoard::from_index(i as u8);
                    i += 1;
                }
                '/' => {
                    assert!(i % 7 == 0)
                }
                '-' => {
                    gaps |= BitBoard::from_index(i as u8);
                    i += 1;
                }
                z if z.is_ascii_digit() => {
                    i += z.to_digit(10).unwrap() as usize;
                }
                _ => panic!("{c} does not belong in this FEN String *facepalm*"),
            }
        }

        assert!(i == 49);
        let occupied = black | white;

        let turn = match fen[1] {
            "x" => Side::Black,
            "o" => Side::White,
            _ => panic!(
                "FEN String is invalid! Expected 'x' or 'o' for turn, got {}",
                fen[1]
            ),
        };

        let half_moves: u8 = fen[2].parse().unwrap();
        assert!(
            half_moves <= 100,
            "A half-move clock of {half_moves} is illegal"
        );

        let full_moves: u8 = fen[3].parse().unwrap();

        Position {
            occupied,
            black,
            white,
            gaps,
            turn,
            half_moves,
            full_moves,
        }
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

            // Slash at the end of a row, but not in the beginning or end
            if i % 7 == 6 && i != 0 && i != 48 {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push('/');
            }
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
    fn fens() {
        let fen = "x5o/7/7/7/7/7/o5x x 0 1";
        let p = Position::from_fen(fen);
        assert_eq!(p.turn, Side::Black);
        assert_eq!(p.occupied, BitBoard(0x1040000000041));
        assert_eq!(p.black, BitBoard(0x1000000000001));
        assert_eq!(p.white, BitBoard(0x40000000040));
        assert_eq!(p.gaps, BitBoard(0));
        assert_eq!(p.half_moves, 0);
        assert_eq!(p.full_moves, 1);
        assert_eq!(p.get_fen(), fen);
    }
}
