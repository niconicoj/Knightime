extern crate nightime;

use nightime::player::Player;
use nightime::{constants::*, player::Side};

fn main() {
    let white = Player::new(Side::White);

    for (index, pawn_attack) in white.get_pawn_attacks().iter().enumerate() {
        println!("square index {}", index);
        println!("{}", pawn_attack);
    }
}
