use super::state::State;

pub fn main() {
    let mut state = State::default();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut tokens = input.split_whitespace();
    }
}
