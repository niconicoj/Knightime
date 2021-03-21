extern crate nightime;

use nightime::{constants::*, move_generator::MoveGenerator};

fn main() {
    let movegen = MoveGenerator::new();

    let king_attack = movegen.get_king_attacks(A2);
    println!("{}", king_attack);

    let white_pawn_attack = movegen.get_pawn_attacks(B5, nightime::constants::Side::White);
    println!("{}", white_pawn_attack);

    let black_pawn_attack = movegen.get_pawn_attacks(B5, nightime::constants::Side::Black);
    println!("{}", black_pawn_attack);

    let knight_attack = movegen.get_knight_attacks(G4);
    println!("{}", knight_attack);
}
