use crate::ataxx::position::Position;

pub struct State {
    pub pos: Position,
}

impl State {
    pub fn default() -> State {
        State {
            pos: Position::default(),
        }
    }
}
