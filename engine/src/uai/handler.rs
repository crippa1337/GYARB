use crate::ataxx::position::Position;
use crate::engine::mcts::Tree;

pub fn main_loop() {
    let mut pos = Position::default();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let token: Vec<&str> = input.split_whitespace().collect();

        match token[0] {
            "uai" => {
                println!("id name Kurt");
                println!("id author Cristopher Torgrip");
                println!("uaiok");
            }

            "isready" => {
                println!("readyok");
            }

            "position" => {
                let mut fen = String::new();

                if token[1] == "fen" {
                    for f in token.iter().skip(2) {
                        fen.push_str(format!("{} ", f).as_str());
                    }
                } else if token[1] == "startpos" {
                    fen = "x5o/7/7/7/7/7/o5x x 0 1".to_string();
                } else {
                    continue;
                }

                println!("{}", fen);
                fen = fen.trim().to_owned();

                match Position::from_fen(&fen) {
                    Ok(_) => pos = Position::from_fen(&fen).unwrap(),
                    Err(_) => {
                        println!("Failed to read fen");
                        continue;
                    }
                }
            }

            "go" => {
                let mut tree = Tree::new();
                let mv = tree.uct(pos, 5000);
                println!("bestmove {}", mv);
            }

            _ => continue,
        }
    }
}
