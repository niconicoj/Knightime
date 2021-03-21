use std::usize;

use crate::{
    bitboard::Bitboard,
    constants::*,
    defs::{Piece, Square},
    magic::{Magic, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE},
};

mod generate;
mod init;
mod mask;

pub struct MoveGenerator {
    white_pawns: Vec<Bitboard>,
    black_pawns: Vec<Bitboard>,
    king: Vec<Bitboard>,
    knights: Vec<Bitboard>,
    rooks: Vec<Bitboard>,
    rook_magics: [Magic; 64],
    bishops: Vec<Bitboard>,
    bishop_magics: [Magic; 64],
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        let magics: Magic = Magic::default();

        let mut move_generator = Self {
            white_pawns: Self::init_pawns_attacks(Side::White),
            black_pawns: Self::init_pawns_attacks(Side::Black),
            king: Self::init_king_attacks(),
            knights: Self::init_knights_attacks(),
            rooks: vec![Bitboard::default(); ROOK_TABLE_SIZE],
            bishops: vec![Bitboard::default(); BISHOP_TABLE_SIZE],
            rook_magics: [magics; 64],
            bishop_magics: [magics; 64],
        };

        move_generator.init_magics(Piece::Rook);
        move_generator.init_magics(Piece::Bishop);

        move_generator
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

    pub fn get_rook_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let index = self.rook_magics[square as usize].get_index(occupancy);
        self.rooks[index]
    }

    pub fn get_bishop_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let index = self.bishop_magics[square as usize].get_index(occupancy);
        self.bishops[index]
    }

    pub fn get_queen_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let r_index = self.rook_magics[square as usize].get_index(occupancy);
        let b_index = self.bishop_magics[square as usize].get_index(occupancy);
        self.rooks[r_index] ^ self.bishops[b_index]
    }
}
