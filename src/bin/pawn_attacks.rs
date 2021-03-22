extern crate nightime;

use nightime::{
    bitboard::Bitboard,
    board::Board,
    constants::*,
    defs::{Piece, Side},
    move_generator::MoveGenerator,
};

fn main() {
    let board = Board::default();

    println!("{}", board);
}
