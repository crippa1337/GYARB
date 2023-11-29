use tetra::{
    graphics::{Color, DrawParams},
    math::Vec2,
    Context,
};

use super::{
    consts::{TILE_PADDING, TILE_SIZE},
    game::Assets,
    stone::Stone,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pos: (usize, usize),
    stone: Option<Stone>,
    blocked: bool,
}

impl Tile {
    pub fn new(pos: (usize, usize)) -> Tile {
        Tile {
            pos,
            stone: None,
            blocked: false,
        }
    }

    pub fn get_stone(&self) -> Option<Stone> {
        self.stone
    }

    pub fn pop_stone(&mut self) -> Option<Stone> {
        self.stone.take()
    }

    pub fn set_stone(&mut self, stone: Stone) {
        self.stone = Some(stone);
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked
    }

    pub fn set_blocked(&mut self, blocked: bool) {
        self.blocked = blocked;
    }

    pub fn get_grid_pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn get_pos(&self) -> Vec2<f32> {
        Vec2::new(self.pos.0 as f32 * TILE_SIZE, self.pos.1 as f32 * TILE_SIZE)
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) {
        let pos = self.get_pos();
        let color = if self.blocked {
            Color::rgb(0.5, 0.5, 0.5)
        } else {
            Color::rgb(1.0, 1.0, 1.0)
        };
        assets.tile.draw(
            ctx,
            DrawParams::new()
                .color(color)
                .position((pos.x + TILE_PADDING / 2., pos.y + TILE_PADDING / 2.).into()),
        );
        if let Some(stone) = self.stone {
            assets.stone.draw(
                ctx,
                DrawParams::new()
                    .color(stone.color())
                    .position((pos.x + TILE_SIZE / 2.0, pos.y + TILE_SIZE / 2.0).into()),
            );
        }
    }
}
