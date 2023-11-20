mod ataxx;
mod engine;
mod uai;

fn main() {
    let mut pos = ataxx::position::Position::default();
    let mut tree = engine::mcts::Tree::new(pos);
    tree.select_expand_simulate();
    println!("{}", tree.best_move());

    println!("Before:");
    println!("{}", pos);
    pos.make_move(tree.best_move());
    println!("After:");
    println!("{}", pos);
}
