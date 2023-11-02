use crate::engine::moves::Move;

mod ataxx;
mod engine;
mod uai;

fn main() {
    //uai::handler::main();
    //let mut pos = crate::ataxx::position::Position::default();
    //pos.split_perft(2);
    let mut pos =
        crate::ataxx::position::Position::from_fen("xxxxxxx/ooooooo/ooooooo/7/7/7/7 x 0 1")
            .unwrap();
    println!("{}", pos);

    pos.make_move(Move::pass());
    println!("{}", pos);

    pos.make_move(Move { from: 21, to: 21 });
    println!("{}", pos);
}
