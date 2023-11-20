mod ataxx;
mod engine;
mod uai;

fn main() {
    let mut pos = ataxx::position::Position::from_fen(
        "xx1xx1o/xxxxxoo/xxxoooo/xxxxooo/xxxoooo/oxooooo/oxxxxxx o 0 1",
    )
    .unwrap();
    let mut tree = engine::mcts::Tree::new(pos);
    tree.select_expand_simulate();
    println!("{}", tree.best_move());

    println!("Before:");
    println!("{}", pos);
    pos.make_move(tree.best_move());
    println!("After:");
    println!("{}", pos);
}
