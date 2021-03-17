extern crate nightime;

use nightime::constants::*;
use nightime::bitboard::Bitboard;


fn main() {
    let mut bitboard: Bitboard = Bitboard::default();

    bitboard.set_square(A1);
    bitboard.set_square(B2);
    bitboard.set_square(C1);
    bitboard.set_square(C3);
    println!("{}", bitboard);
}