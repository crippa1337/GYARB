use crate::ataxx::{bitboard::BitBoard, position::Position};

impl Position {
    #[allow(dead_code)]
    pub fn perft(&self, depth: i16) -> u64 {
        if depth == 0 {
            return 1;
        } else if self.game_over() {
            return 0;
        } else if depth == 1 {
            let mut num_moves = 0;
            let s2m = self.colored_squares(self.turn);
            let empty = self.empty_squares();

            num_moves += (s2m.singles() & empty).popcnt();

            for sq in s2m {
                let doubles = BitBoard::from_index(sq).doubles() & empty;
                num_moves += doubles.popcnt();
            }

            // pass
            if num_moves == 0 {
                return 1;
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

    pub fn split_perft(&self, depth: i16) {
        let mut nodes = 0;
        let start = std::time::Instant::now();
        let moves = self.generate_moves();

        for mv in moves.as_slice() {
            let mut new_pos = *self;
            new_pos.make_move(*mv);
            let branch_nodes = new_pos.perft(depth - 1);
            nodes += branch_nodes;
            println!("{mv}: {branch_nodes}");
        }

        let duration = start.elapsed();
        let nps = nodes as f64 / duration.as_secs_f64();

        println!(
            "moves {} nodes {} time {:?} nps {}",
            moves.len(),
            nodes,
            duration.as_millis(),
            nps as u64
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_test() {
        let fens = [
            (
                "x5o/7/7/7/7/7/o5x x 0 1",
                vec![1, 16, 256, 6460, 155888, 4752668],
            ),
            (
                "x5o/7/7/7/7/7/o5x o 0 1",
                vec![1, 16, 256, 6460, 155888, 4752668],
            ),
            (
                "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
                vec![1, 14, 196, 4184, 86528, 2266352],
            ),
            (
                "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
                vec![1, 14, 196, 4184, 86528, 2266352],
            ),
            (
                "x5o/7/2-1-2/3-3/2-1-2/7/o5x x 0 1",
                vec![1, 14, 196, 4100, 83104, 2114588],
            ),
            (
                "x5o/7/2-1-2/3-3/2-1-2/7/o5x o 0 1",
                vec![1, 14, 196, 4100, 83104, 2114588],
            ),
            ("7/7/7/7/7/7/7 x 0 1", vec![1, 0, 0, 0, 0, 0]),
            (
                "xxxxxxx/-------/-------/o6/7/7/7 x 0 1",
                vec![1, 1, 8, 8, 127, 127, 2626, 2626],
            ),
            (
                "xxxxxxx/ooooooo/ooooooo/7/7/7/7 x 0 1",
                vec![1, 1, 75, 249, 14270, 452980],
            ),
        ];

        for (fen, perfts) in fens.iter() {
            let pos = Position::from_fen(fen).unwrap();
            for (depth, nodes) in perfts.iter().enumerate() {
                assert_eq!(pos.perft(depth as i16), *nodes, "{}", fen);
            }
        }
    }
}
