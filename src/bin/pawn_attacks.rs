extern crate nightime;

use nightime::{
    bitboard::Bitboard,
    constants::*,
    defs::{Piece, Side},
    move_generator::MoveGenerator,
};

fn main() {
    let side = Side::Black;
    let piece = Piece::King;

    println!("{}", UNICODE_PIECE[side as usize][piece as usize]);
}
