extern crate nightime;

use nightime::{constants::ROOK_RELEVANT_BITS, defs::Piece, misc::find_magic};

fn main() {
    for square in 0..64 {
        match find_magic(square, ROOK_RELEVANT_BITS[square as usize], Piece::Rook) {
            Some(magic) => println!("square {} magic : {}", square, magic),
            None => println!("square {} no magic found", square),
        }
    }
}
