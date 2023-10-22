mod ataxx;
mod engine;
mod uai;

fn main() {
    //uai::handler::main();
    let pos = crate::ataxx::position::Position::default();
    pos.split_perft(2)
}
