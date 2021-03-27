extern crate knightime;

use knightime::{
    board::Board,
    constants::*,
    defs::Side,
    move_generator::{movelist::Move, MoveGenerator},
};

fn main() {
    let mv = Move::new(
        E3,
        F4,
        knightime::defs::Piece::Pawn,
        knightime::defs::Promotion::None,
        true,
        false,
        true,
        false,
    );

    println!("{}", SQUARE_NAME[mv.get_source_square() as usize]);
    println!("{}", SQUARE_NAME[mv.get_target_square() as usize]);
    println!("{:?}", mv.get_piece());
    println!("{:?}", mv.get_promotion());
    println!("{:?}", mv.get_capture());
    println!("{:?}", mv.get_double_push());
    println!("{:?}", mv.get_en_passant());
    println!("{:?}", mv.get_caslting());
}
