extern crate knightime;

use knightime::{board::Board, defs::Side};

fn main() {
    let board = Board::from_fen("8/8/8/3N4/4B1R1/1P6/8/6K1 w - - 0 1").unwrap();
    println!("{}", board);
    let attacked_squares = board.get_attacked_squares(Side::White);
    println!("{}", attacked_squares);
}
