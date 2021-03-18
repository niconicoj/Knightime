extern crate nightime;

use nightime::attack_tables::generate_bishop_attacks;

fn main() {
    let bishop_attacks = generate_bishop_attacks();

    for (index, bishop_attack) in bishop_attacks.iter().enumerate() {
        println!("square index {}", index);
        println!("{}", bishop_attack);
    }
}
