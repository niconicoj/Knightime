extern crate nightime;

use nightime::{attack_tables::generate_rook_attacks_otf, bitboard::Bitboard, constants::*};

fn main() {
    let bitboard = generate_rook_attacks_otf(G5, Bitboard::default());
    println!("{}", bitboard);
    let bitboard = generate_rook_attacks_otf(D4, Bitboard::new(0x0008000012000800));
    println!("{}", bitboard);
    let bitboard = generate_rook_attacks_otf(A1, Bitboard::default());
    println!("{}", bitboard);
}
