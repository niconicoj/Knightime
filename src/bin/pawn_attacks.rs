extern crate nightime;

use nightime::{
    attack_tables::{generate_rook_attacks_otf, mask_rook_attacks, set_occupancy},
    bitboard::Bitboard,
    constants::*,
};

fn main() {
    let mut rook_attack_mask = mask_rook_attacks(A1);

    let occupancy = set_occupancy(2048, &mut rook_attack_mask);
    println!("{}", occupancy);
}
