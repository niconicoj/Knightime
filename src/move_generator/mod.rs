use std::usize;

use crate::{
    bitboard::Bitboard,
    defs::{Piece, Side, Square},
    magic::{Magic, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE},
};

mod defs;
mod generate;
mod init;
mod mask;
pub mod movelist;

#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn get_pawn_attacks_tests() {
        let movgen = MoveGenerator::new();

        assert_eq!(movgen.get_pawn_attacks(A1, Side::White), 0x0000000000000200);
        assert_eq!(movgen.get_pawn_attacks(B2, Side::White), 0x0000000000050000);
        assert_eq!(movgen.get_pawn_attacks(D8, Side::White), 0x0000000000000000);

        assert_eq!(movgen.get_pawn_attacks(D8, Side::Black), 0x0014000000000000);
        assert_eq!(movgen.get_pawn_attacks(A7, Side::Black), 0x0000020000000000);
        assert_eq!(movgen.get_pawn_attacks(D1, Side::Black), 0x0000000000000000);
    }

    #[test]
    fn get_king_attacks_tests() {
        let movgen = MoveGenerator::new();

        assert_eq!(movgen.get_king_attacks(A4), 0x0000000302030000);
        assert_eq!(movgen.get_king_attacks(D1), 0x0000000000001c14);
        assert_eq!(movgen.get_king_attacks(H5), 0x0000c040c0000000);
        assert_eq!(movgen.get_king_attacks(E8), 0x2838000000000000);

        assert_eq!(movgen.get_king_attacks(A1), 0x0000000000000302);
        assert_eq!(movgen.get_king_attacks(A8), 0x0203000000000000);
        assert_eq!(movgen.get_king_attacks(H1), 0x000000000000c040);
        assert_eq!(movgen.get_king_attacks(H8), 0x40c0000000000000);

        assert_eq!(movgen.get_king_attacks(D4), 0x0000001c141c0000);
    }

    #[test]
    fn get_knight_attacks_tests() {
        let movgen = MoveGenerator::new();

        assert_eq!(movgen.get_knight_attacks(A4), 0x0000020400040200);
        assert_eq!(movgen.get_knight_attacks(D1), 0x0000000000142200);
        assert_eq!(movgen.get_knight_attacks(H5), 0x0040200020400000);
        assert_eq!(movgen.get_knight_attacks(E8), 0x0044280000000000);

        assert_eq!(movgen.get_knight_attacks(A1), 0x0000000000020400);
        assert_eq!(movgen.get_knight_attacks(A8), 0x0004020000000000);
        assert_eq!(movgen.get_knight_attacks(H1), 0x0000000000402000);
        assert_eq!(movgen.get_knight_attacks(H8), 0x0020400000000000);

        assert_eq!(movgen.get_knight_attacks(D4), 0x0000142200221400);
    }
}
