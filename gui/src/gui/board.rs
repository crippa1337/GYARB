use tetra::graphics::Color;
use tetra::{graphics::DrawParams, Context};

use crate::moving::Move;

use super::{
    consts::TILE_SIZE,
    distance::{ChebyshevDistance, ChebyshevDistanceSelf},
    game::Assets,
    stone::Stone,
    tile::Tile,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Board<const S: usize> {
    tiles: [[Tile; S]; S],
    blocked: usize,
}

#[derive(Debug, Clone)]
pub enum FenError {
    InvalidCharacter { fen: String, pos: usize },
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FenError::InvalidCharacter { fen, pos } => {
                let before = &fen[..*pos];
                let after = &fen[*pos + 1..];
                let c = &fen[*pos..=*pos];
                write!(
                    f,
                    "Invalid character: '{}\x1b[93m{}\x1b[0m{}'",
                    before, c, after
                )
            }
        }
    }
}

impl<const S: usize> Board<S> {
    pub fn new() -> Board<S> {
        let mut tiles = [[Tile::new((0, 0)); S]; S];
        for (i, row) in tiles.iter_mut().enumerate() {
            for (j, tile) in row.iter_mut().enumerate() {
                *tile = Tile::new((i, j));
            }
        }
        Board { tiles, blocked: 0 }
    }

    pub fn from_fen(fen: &str) -> Result<Board<S>, FenError> {
        let mut board = Board::new();
        let mut x = 0;
        let mut y = 0;
        for (i, c) in fen.chars().enumerate() {
            match c {
                '1'..='9' => {
                    let count = c.to_digit(10).unwrap() as usize;
                    x += count;
                }
                '/' => {
                    x = 0;
                    y += 1;
                }
                'x' | 'o' => {
                    if let Some(row) = board.tiles.get_mut(x) {
                        if let Some(tile) = row.get_mut(y) {
                            tile.set_stone(Stone::from(c));
                        }
                    }
                    x += 1;
                }
                '-' => {
                    if let Some(row) = board.tiles.get_mut(x) {
                        if let Some(tile) = row.get_mut(y) {
                            tile.set_blocked(true);
                        }
                    }
                    board.blocked += 1;
                    x += 1;
                }
                _ => {
                    return Err(FenError::InvalidCharacter {
                        fen: fen.to_string(),
                        pos: i,
                    })
                }
            }
        }
        Ok(board)
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for i in 0..S {
            let mut count = 0;
            for j in 0..S {
                let tile = self.get_tile((j, i));
                if tile.is_blocked() {
                    if count > 0 {
                        fen.push_str(&count.to_string());
                        count = 0;
                    }
                    fen.push('-');
                } else if let Some(stone) = tile.get_stone() {
                    if count > 0 {
                        fen.push_str(&count.to_string());
                        count = 0;
                    }
                    let stone_char = match stone {
                        Stone::Dark => 'x',
                        Stone::Light => 'o',
                    };
                    fen.push(stone_char);
                } else {
                    count += 1;
                }
            }
            if count > 0 {
                fen.push_str(&count.to_string());
            }
            if i < S - 1 {
                fen.push('/');
            }
        }
        fen
    }

    pub fn get_tile(&self, pos: (usize, usize)) -> &Tile {
        &self.tiles[pos.0][pos.1]
    }

    pub fn get_tile_mut(&mut self, pos: (usize, usize)) -> &mut Tile {
        &mut self.tiles[pos.0][pos.1]
    }

    pub fn get_tiles(&self) -> &[[Tile; S]; S] {
        &self.tiles
    }

    pub fn get_tiles_mut(&mut self) -> &mut [[Tile; S]; S] {
        &mut self.tiles
    }

    pub fn count_stones(&self) -> (usize, usize) {
        let mut dark = 0;
        let mut light = 0;
        for row in &self.tiles {
            for tile in row {
                if let Some(stone) = tile.get_stone() {
                    match stone {
                        Stone::Dark => dark += 1,
                        Stone::Light => light += 1,
                    }
                }
            }
        }
        (dark, light)
    }

    pub fn count_empty(&self) -> usize {
        let (dark, light) = self.count_stones();
        let non_empty = dark + light + self.blocked;
        S * S - non_empty
    }

    pub fn game_over(&self) -> bool {
        let (dark, light) = self.count_stones();
        dark == 0 || light == 0 || dark + light == S * S - self.blocked
    }

    pub fn cant_move(&self, player: usize) -> bool {
        let mut cant_move = true;
        'outer: for row in &self.tiles {
            for tile in row {
                if tile.get_stone() == Some(Stone::from(player)) {
                    let can_move = self.can_tile_move(tile.get_grid_pos());
                    if can_move {
                        cant_move = false;
                        break 'outer;
                    }
                }
            }
        }
        cant_move
    }

    pub fn get_surrounding_tiles(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut tiles = Vec::with_capacity(8);
        let (x, y) = pos;
        for i in x.saturating_sub(1)..(x + 2).clamp(0, S) {
            for j in y.saturating_sub(1)..(y + 2).clamp(0, S) {
                if i == x && j == y {
                    continue;
                }
                tiles.push((i, j));
            }
        }
        tiles
    }

    pub fn can_tile_move(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        for i in x.saturating_sub(2)..(x + 3).clamp(0, S) {
            for j in y.saturating_sub(2)..(y + 3).clamp(0, S) {
                if i == x && j == y {
                    continue;
                }
                let tile = self.get_tile((i, j));
                if tile.get_stone().is_none() && !tile.is_blocked() {
                    return true;
                }
            }
        }
        false
    }

    pub fn play_move(&mut self, mv: Move, player: usize) -> Result<(), ()> {
        if mv == Move::NULL {
            return Ok(());
        }
        let distance = mv.chebyshev_distance();
        let from_usize = (mv.from.0 as usize, mv.from.1 as usize);
        let from_tile = self.get_tile_mut(from_usize);
        if from_tile.get_stone().is_none() && mv.from != mv.to {
            return Err(());
        }
        let stone = if distance > 1 {
            from_tile.pop_stone()
        } else {
            Some(Stone::from(player))
        };

        let to_usize = (mv.to.0 as usize, mv.to.1 as usize);
        let to_tile = self.get_tile_mut(to_usize);
        if to_tile.get_stone().is_some() || to_tile.is_blocked() {
            return Err(());
        }
        to_tile.set_stone(stone.unwrap());

        let neighbours = self.get_surrounding_tiles(to_usize);
        for pos in neighbours {
            let neighbour = self.get_tile_mut(pos);
            if neighbour.get_stone() == Some(Stone::from(!player)) {
                neighbour.set_stone(Stone::from(player));
            }
        }
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Option<Assets>, current_move: Option<Move>) {
        if let Some(assets) = assets {
            assets
                .board
                .draw(ctx, DrawParams::new().color(Color::BLACK));
            for (i, row) in self.tiles.iter().rev().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    tile.draw(ctx, assets);
                    if tile.is_blocked() || tile.get_stone().is_some() {
                        continue;
                    }
                    if let Some(mv) = current_move {
                        let pos = tile.get_pos();
                        let distance = mv.from.chebyshev_distance(((S - i - 1) as u8, j as u8));
                        let color = match distance {
                            1 => Color::rgba(0.0, 0.0, 0.0, 0.75),
                            2 => Color::rgba(0.0, 0.0, 0.0, 0.25),
                            _ => {
                                continue;
                            }
                        };
                        let params = DrawParams::new()
                            .color(color)
                            .position((pos.x + TILE_SIZE / 2.0, pos.y + TILE_SIZE / 2.0).into());

                        assets.dot.draw(ctx, params);
                    }
                }
            }
        }
    }
}
