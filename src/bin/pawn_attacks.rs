extern crate nightime;

use nightime::{attack::mask_pawn_attacks};
use nightime::constants::*;

fn main() {
    let attacks = mask_pawn_attacks(Player::Black, A7);
    println!("{}",attacks);
    let attacks = mask_pawn_attacks(Player::Black, B7);
    println!("{}",attacks);
    let attacks = mask_pawn_attacks(Player::Black, H7);
    println!("{}",attacks);
    let attacks = mask_pawn_attacks(Player::Black, D1);
    println!("{}",attacks);
}