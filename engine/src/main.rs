use crate::engine::moves::Move;

mod ataxx;
mod engine;
mod uai;

fn main() {
    //uai::handler::main();
    let mut pos = crate::ataxx::position::Position::default();
    println! {"{}", pos};
    let mv = Move::new(36, 36);
    pos.make_move(mv);
    println! {"{}", pos};
}
