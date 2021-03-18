extern crate nightime;

use nightime::player::Player;
use nightime::{constants::*, player::Side};

fn main() {
    let white = Player::new(Side::White);

    let king_attacks = white.get_king_attacks();

    for (index, king_attack) in king_attacks.iter().enumerate() {
        println!("square index : {}", index);
        println!("{}", king_attack);
    }
}
