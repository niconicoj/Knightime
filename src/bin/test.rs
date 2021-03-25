extern crate knightime;

use knightime::{board::Board, defs::Side};

fn main() {
    let board =
        Board::from_fen("r2qk2r/2Pb1pbp/4p1p1/1p1pn3/P2P1Bn1/3B4/p2N1PPP/1Q2K2R w Kkq - 0 19")
            .unwrap();
    println!("{}", board);
    board.generate_moves(Side::White);
    board.generate_moves(Side::Black);
}
