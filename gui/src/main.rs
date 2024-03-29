use ataxx::gui::consts::{BOARD_SIZE, TILE_SIZE};
use tetra::ContextBuilder;

fn gui_app() -> tetra::Result {
    use ataxx::gui::game::AppState;
    let size = TILE_SIZE as i32 * BOARD_SIZE as i32;
    let games = vec![
        //"x2-2o/3-3/3-3/---1---/3-3/3-3/o2-2x x 0 1".to_owned(),
        "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1".to_owned(),
        "x2-2o/7/7/-5-/7/7/o2-2x x 0 1".to_owned(),
        "x5o/7/7/7/7/7/o5x x 0 1".to_owned(),
    ];
    let mut player_name = String::new();
    std::io::stdin().read_line(&mut player_name).unwrap();
    player_name = player_name.trim().to_owned();
    ContextBuilder::new("Ataxx", size, size)
        .show_mouse(true)
        .quit_on_escape(true)
        .build()?
        .run(AppState::new_player_vs_engine(
            player_name,
            "./engine".to_owned(),
            Vec::new(),
            games,
        ))
}

fn main() {
    gui_app().unwrap();
}
