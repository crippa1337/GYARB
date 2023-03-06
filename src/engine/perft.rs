use crate::ataxx::{bitboard::BitBoard, position::Position};

impl Position {
    pub fn perft(&self, depth: i16) -> u64 {
        if depth == 0 {
            return 1;
        } else if depth == 1 {
            let mut num_moves = 0;
            let s2m = self.colored_squares(self.turn);
            let empty = self.empty_squares();

            num_moves += (s2m.singles() & empty).popcnt();

            for sq in s2m {
                let doubles = BitBoard::from_index(sq).doubles() & empty;
                num_moves += doubles.popcnt();
            }

            return num_moves as u64;
        }

        let mut nodes = 0;
        let moves = self.generate_moves();

        for mv in moves.as_slice() {
            let mut new_pos = *self;
            new_pos.make_move(*mv);
            nodes += new_pos.perft(depth - 1);
        }

        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_test() {
        const FENS: [(&str, [u64; 6]); 7] = [
            (
                "x5o/7/7/7/7/7/o5x x 0 1",
                [1, 16, 256, 6460, 155888, 4752668],
            ),
            (
                "x5o/7/7/7/7/7/o5x o 0 1",
                [1, 16, 256, 6460, 155888, 4752668],
            ),
            (
                "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
                [1, 14, 196, 4184, 86528, 2266352],
            ),
            (
                "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
                [1, 14, 196, 4184, 86528, 2266352],
            ),
            (
                "x5o/7/2-1-2/3-3/2-1-2/7/o5x x 0 1",
                [1, 14, 196, 4100, 83104, 2114588],
            ),
            (
                "x5o/7/2-1-2/3-3/2-1-2/7/o5x o 0 1",
                [1, 14, 196, 4100, 83104, 2114588],
            ),
            ("7/7/7/7/7/7/7 x 0 1", [1, 0, 0, 0, 0, 0]),
        ];

        for (fen, perfts) in FENS.iter() {
            let pos = Position::from_fen(fen).unwrap();
            for (depth, nodes) in perfts.iter().enumerate() {
                assert_eq!(pos.perft(depth as i16), *nodes);
            }
        }
    }
}
