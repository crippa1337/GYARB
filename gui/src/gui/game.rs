use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::Write,
};

use tetra::{
    graphics::{
        mesh::{Mesh, ShapeStyle},
        Rectangle,
    },
    input::get_mouse_position,
    math::Vec2,
    window::quit,
    Context, State,
};

use crate::{moving::Move, uai::Handler};

use super::{
    board::Board,
    consts::{TILE_PADDING, TILE_SIZE},
    player::PlayerType,
};

#[derive(Debug, Clone)]
struct Game {
    board: Board<7>,
    current_player: usize,
    half_moves: u32,
    full_moves: u32,
    moves: Vec<Move>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::new(),
            current_player: 0,
            half_moves: 0,
            full_moves: 0,
            moves: Vec::new(),
        }
    }
}

impl Game {
    pub fn from_fen(fen: &str) -> Result<Game, super::board::FenError> {
        let mut game = Game::default();
        let fen = fen.split_whitespace().collect::<Vec<_>>();
        game.current_player = if fen[1] == "x" { 0 } else { 1 };
        game.half_moves = fen[2].parse().unwrap();
        game.full_moves = fen[3].parse().unwrap();

        game.board = Board::from_fen(fen[0])?;
        Ok(game)
    }

    pub fn play_move(&mut self, m: Move) -> Result<(), ()> {
        if self.board.play_move(m, self.current_player).is_ok() {
            self.moves.push(m);
            self.current_player += 1;
            self.current_player %= 2;
            self.half_moves += 1;
            if self.current_player == 0 {
                self.full_moves += 1;
            }
            let (stones_1, stones_2) = self.board.count_stones();
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct Assets {
    pub tile: Mesh,
    pub stone: Mesh,
    pub board: Mesh,
    pub dot: Mesh,
}

impl Assets {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            stone: Mesh::circle(
                ctx,
                ShapeStyle::Fill,
                Vec2::zero(),
                (TILE_SIZE - TILE_PADDING) / 3.0,
            )?,
            tile: Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0., 0., TILE_SIZE - TILE_PADDING, TILE_SIZE - TILE_PADDING),
            )?,
            board: Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0., 0., TILE_SIZE * 7., TILE_SIZE * 7.),
            )?,
            dot: Mesh::circle(
                ctx,
                ShapeStyle::Fill,
                Vec2::zero(),
                (TILE_SIZE - TILE_PADDING) / 6.0,
            )?,
        })
    }
}

pub struct AppState {
    current_game: Option<Game>,
    games: Vec<Game>,
    players: [PlayerType; 2],
    assets: Option<Assets>,
    to_play: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_game: None,
            games: Vec::new(),
            players: [
                PlayerType::new_human("Player 1".to_string()),
                PlayerType::new_human("Player 2".to_string()),
            ],
            assets: None,
            to_play: Vec::new(),
        }
    }
}

impl AppState {
    pub fn new() -> tetra::Result<Self> {
        Ok(Self::default())
    }

    pub fn new_player_vs_engine(
        player_name: String,
        engine_path: String,
        args: Vec<String>,
        to_play: Vec<String>,
    ) -> impl Fn(&mut Context) -> tetra::Result<Self> {
        move |ctx: &mut Context| -> Result<AppState, tetra::TetraError> {
            let engine = PlayerType::new_engine({
                let mut engine = Handler::new(engine_path.clone().into(), args.clone());
                engine.auth().expect("Failed to authenticate engine");

                engine
            });
            let state = Self {
                players: [PlayerType::new_human(player_name.clone()), engine],
                assets: Assets::new(ctx).ok(),
                to_play: to_play.clone(),
                ..Self::default()
            };
            Ok(state)
        }
    }

    pub fn write_results(&self) {
        // Check if file exists
        let player_name = match &self.players[0] {
            PlayerType::Human(player) => player._name.clone(),
            _ => "Engine".to_string(),
        };
        let mut file = if std::fs::metadata(format!("{}.csv", player_name)).is_ok() {
            std::fs::OpenOptions::new()
                .append(true)
                .open(format!("{}.csv", player_name))
                .unwrap()
        } else {
            let mut file = std::fs::File::create(format!("{}.csv", player_name)).unwrap();
            file.write_all("Result,Player,Engine,Moves\n".as_bytes())
                .unwrap();
            file
        };
        let (player, engine) = self.games.last().unwrap().board.count_stones();
        let result = match player.cmp(&engine) {
            std::cmp::Ordering::Greater => "W",
            std::cmp::Ordering::Equal => "D",
            std::cmp::Ordering::Less => "L",
        };
        let moves = self
            .games
            .last()
            .unwrap()
            .moves
            .iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join(" ");
        let write = format!("{},{},{},{}\n", result, player, engine, moves);
        file.write_all(write.as_bytes()).unwrap();
    }
}

impl State for AppState {
    fn update(&mut self, ctx: &mut tetra::Context) -> tetra::Result {
        if let Some(mut game) = self.current_game.take() {
            if game.board.game_over() {
                self.games.push(game);
                self.write_results();
            } else if game.board.cant_move(game.current_player) {
                let other_player = (game.current_player + 1) % 2;
                if game.board.cant_move(other_player) {
                    self.games.push(game);
                    self.write_results();
                } else {
                    game.moves.push(Move::NULL);
                    game.current_player = other_player;
                    self.current_game = Some(game);
                }
            } else {
                self.current_game = Some(game);
            }
        }
        if let Some(game) = &mut self.current_game {
            let player = &mut self.players[game.current_player];
            match player {
                PlayerType::Human(player) => {
                    if let Some(m) = player.make_move(ctx) {
                        if game.play_move(m).is_ok() {
                            player.selected_tile = None;
                            if let PlayerType::Engine(engine) =
                                &mut self.players[game.current_player]
                            {
                                if game.board.cant_move(game.current_player)
                                    || game.board.game_over()
                                {
                                    return Ok(());
                                }
                                let mut fen = game.board.to_fen();
                                fen.push_str(
                                    format!(
                                        " {} {} {}",
                                        if game.current_player == 0 { "x" } else { "o" },
                                        game.half_moves,
                                        game.full_moves
                                    )
                                    .as_str(),
                                );
                                engine.new_pos(&fen);
                            }
                        }
                    } else if let Some(sel) = player.selected_tile {
                        let stone = game
                            .board
                            .get_tile((sel.0 as usize, sel.1 as usize))
                            .get_stone();
                        if let Some(stone) = stone {
                            if usize::from(stone) != game.current_player {
                                player.selected_tile = None;
                            }
                        } else {
                            player.selected_tile = None;
                        }
                    }
                }
                PlayerType::Engine(engine) => {
                    if let Some(m) = engine.make_move() {
                        game.play_move(m).unwrap();
                    }
                }
            }
        } else if let Some(fen) = self.to_play.pop() {
            self.current_game = Some(Game::from_fen(&fen).unwrap());
        } else {
            quit(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut tetra::Context) -> tetra::Result {
        let mut has_drawn = false;
        if let Some(game) = &self.current_game {
            for p in &self.players {
                if let PlayerType::Human(player) = p {
                    let from = if let Some(from) = player.selected_tile {
                        from
                    } else {
                        continue;
                    };
                    let clicked_pos = get_mouse_position(ctx);
                    let to = (
                        (clicked_pos.x / TILE_SIZE) as u8,
                        (clicked_pos.y / TILE_SIZE) as u8,
                    );
                    let mv = Move { from, to };
                    game.board.draw(ctx, &self.assets, Some(mv));
                    has_drawn = true;
                }
            }
            if !has_drawn {
                game.board.draw(ctx, &self.assets, None);
            }
        }
        Ok(())
    }
}
