use crate::{bitboard::Bitboard, constants::*, defs::Square, mov::*};

mod init;
mod mask;

pub struct MoveGenerator {
    white_pawns: Vec<Bitboard>,
    black_pawns: Vec<Bitboard>,
    king: Vec<Bitboard>,
    knights: Vec<Bitboard>,
    // rooks: Vec<Bitboard>,
    // rook_magics: Vec<Magic>,
    // bishops: Vec<Bitboard>,
    // bishop_magics: Vec<Magic>,
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        Self {
            white_pawns: Self::init_pawns_attacks(Side::White),
            black_pawns: Self::init_pawns_attacks(Side::Black),
            king: Self::init_king_attacks(),
            knights: Self::init_knights_attacks(),
        }
    }

    pub fn get_pawn_attacks(&self, square: Square, side: Side) -> Bitboard {
        match side {
            Side::White => self.white_pawns[square as usize],
            Side::Black => self.black_pawns[square as usize],
        }
    }

    pub fn get_king_attacks(&self, square: Square) -> Bitboard {
        self.king[square as usize]
    }

    pub fn get_knight_attacks(&self, square: Square) -> Bitboard {
        self.knights[square as usize]
    }
}
