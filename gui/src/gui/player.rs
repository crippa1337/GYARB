use tetra::input::get_mouse_position;

use super::{consts::TILE_SIZE, distance::ChebyshevDistance};
use crate::{moving::Move, uai::Handler};

pub enum PlayerType {
    Human(Human),
    Engine(Engine),
}

pub struct Human {
    _name: String,
    pub(super) selected_tile: Option<(u8, u8)>,
}

impl Human {
    pub fn make_move(&mut self, ctx: &tetra::Context) -> Option<Move> {
        let clicked_pos =
            if tetra::input::is_mouse_button_pressed(ctx, tetra::input::MouseButton::Left) {
                get_mouse_position(ctx)
            } else {
                return None;
            };
        let mouse_grid_pos = (
            (clicked_pos.x / TILE_SIZE) as u8,
            (clicked_pos.y / TILE_SIZE) as u8,
        );
        if let Some(pos) = self.selected_tile {
            // Check if same tile is clicked
            let distance = pos.chebyshev_distance(mouse_grid_pos);
            // Change selected tile
            if distance > 2 {
                self.selected_tile = Some(mouse_grid_pos);
            } else if distance > 0 {
                return Some(Move {
                    from: pos,
                    to: mouse_grid_pos,
                });
            } else {
                self.selected_tile = None;
            }
        } else {
            self.selected_tile = Some(mouse_grid_pos);
        }

        None
    }
}

pub struct Engine {
    uai: Handler,
}

impl Engine {
    pub fn make_move(&mut self) -> Option<Move> {
        use crate::uai::MessageKind as MK;
        while let Some(message) = self.uai.read() {
            match message {
                MK::BestMove {
                    bestmove,
                    ponder: _,
                } => {
                    return Some(bestmove);
                }
                _ => (),
            }
        }
        None
    }

    pub fn new_pos(&mut self, fen: &str) {
        self.uai
            .write(crate::uai::MessageKind::Position {
                fen: fen.into(),
                moves: Vec::new(),
            })
            .unwrap();
        self.uai
            .write(crate::uai::MessageKind::Go(Vec::new()))
            .unwrap();
    }
}

impl PlayerType {
    pub fn new_human(name: String) -> PlayerType {
        PlayerType::Human(Human {
            _name: name,
            selected_tile: None,
        })
    }

    pub fn new_engine(uai: Handler) -> PlayerType {
        PlayerType::Engine(Engine { uai })
    }
}
