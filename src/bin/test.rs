extern crate knightime;

use knightime::{board::Board, defs::Side};

fn main() {
    let board =
        Board::from_fen("r3k2r/3b2bp/4p1p1/1p1p1pP1/P1nP1B2/3n1P2/p2N3P/1Q4KR w kq f6 0 23")
            .unwrap();
    println!("{}", board);
    board.generate_moves(Side::White);
    board.generate_moves(Side::Black);
}
