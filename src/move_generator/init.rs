use crate::{bitboard::Bitboard, constants::Side};

use super::MoveGenerator;

impl MoveGenerator {
    pub fn init_pawns_attacks(side: Side) -> Vec<Bitboard> {
        let mut pawn_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            pawn_attacks.push(Self::mask_pawn_attacks(side, square));
        }
        pawn_attacks
    }

    pub fn init_knights_attacks() -> Vec<Bitboard> {
        let mut knight_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            knight_attacks.push(Self::mask_knight_attacks(square));
        }
        knight_attacks
    }

    pub fn init_king_attacks() -> Vec<Bitboard> {
        let mut king_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            king_attacks.push(Self::mask_king_attacks(square));
        }
        king_attacks
    }
}
