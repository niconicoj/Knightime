extern crate nightime;

use nightime::{
    attack_tables::{mask_bishop_attacks, mask_rook_attacks},
    bitboard::Bitboard,
    constants::*,
};

fn main() {
    let bitboard = mask_bishop_attacks(G5);
    println!("{}", bitboard);
}
