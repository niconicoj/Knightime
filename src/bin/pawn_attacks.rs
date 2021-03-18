extern crate nightime;

use nightime::player::Player;
use nightime::{constants::*, player::Side};

fn main() {
    let white = Player::new(Side::White);

    for (index, knight_attacks) in white.get_knight_attacks().iter().enumerate() {
        println!("square index {}", index);
        println!("{}", knight_attacks);
    }

}
