use crate::ataxx::position::Position;

use super::state::State;

pub fn main() {
    let mut state = State::default();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let token: Vec<&str> = input.split_whitespace().collect();

        match token[0] {
            "uai" => {
                println!("id name Kurt");
                println!("id author Crippa");
                println!("uaiok");
            }

            "isready" => {
                println!("readyok");
            }

            "position" => {
                let mut fen = String::new();

                if token[1] == "fen" {
                    for f in token.iter().take(6).skip(2) {
                        fen.push(' ');
                        fen.push_str(f);
                    }
                } else if token[1] == "startpos" {
                    fen = "x5o/7/7/7/7/7/o5x x 0 1".to_string();
                } else {
                    continue;
                }

                println!("FEN: {}", fen);

                match Position::from_fen(&fen) {
                    Ok(_) => state.pos = Position::from_fen(&fen).unwrap(),
                    Err(_) => continue,
                }
            }

            _ => {}
        }
    }
}
