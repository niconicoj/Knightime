extern crate nightime;

use nightime::{attack_tables::mask_bishop_attacks, constants::*};

fn main() {
    let bitboard = mask_bishop_attacks(G5);
    println!("{}", bitboard);
}
