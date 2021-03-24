extern crate knightime;

use knightime::{board::Board, defs::Side};

fn main() {
    let board = Board::default();
    println!("{}", board);
    let attacked_squares = board.get_attacked_squares(Side::White);
    println!("{}", attacked_squares);
}
