extern crate nightime;

use nightime::{attack_tables::mask_rook_attacks, constants::*};

fn main() {
    let rook_attacks = mask_rook_attacks(A4);
    println!("{}", rook_attacks);

    let rook_attacks = mask_rook_attacks(D4);
    println!("{}", rook_attacks);

    let rook_attacks = mask_rook_attacks(A1);
    println!("{}", rook_attacks);

    let rook_attacks = mask_rook_attacks(H8);
    println!("{}", rook_attacks);

    let rook_attacks = mask_rook_attacks(H4);
    println!("{}", rook_attacks);
}
