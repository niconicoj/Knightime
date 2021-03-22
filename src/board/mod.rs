use crate::{
    bitboard::Bitboard,
    defs::{Side, Sides},
};

pub struct Board {
    bitboards: [[Bitboard; 6]; 2],
    occupancies: [Bitboard; 3],
    side_to_move: Side,
    en_passant_square: u32,
    castling_rights: u32,
}
