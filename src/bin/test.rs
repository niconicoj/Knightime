extern crate knightime;

use knightime::{
    board::Board,
    constants::*,
    defs::Side,
    move_generator::{movelist::Move, MoveGenerator},
};

fn main() {
    let board = Board::from_fen("8/8/8/8/8/8/P7/8 w - - 0 1").unwrap();
}
