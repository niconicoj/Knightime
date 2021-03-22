extern crate nightime;

use nightime::{bitboard::Bitboard, constants::*, defs::Side, move_generator::MoveGenerator};

fn main() {
    let movgen = MoveGenerator::new();

    let mut mv = movgen.get_knight_attacks(A4);
    println!("{}", mv);
    mv = movgen.get_knight_attacks(D1);
    println!("{}", mv);
    mv = movgen.get_knight_attacks(H5);
    println!("{}", mv);
    mv = movgen.get_knight_attacks(E8);
    println!("{}", mv);

    mv = movgen.get_knight_attacks(A1);
    println!("{}", mv);
    mv = movgen.get_knight_attacks(A8);
    println!("{}", mv);
    mv = movgen.get_knight_attacks(H1);
    println!("{}", mv);
    mv = movgen.get_knight_attacks(H8);
    println!("{}", mv);

    mv = movgen.get_knight_attacks(D4);
    println!("{}", mv);
}
