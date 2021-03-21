use crate::{bitboard::Bitboard, defs::Square};

use super::MoveGenerator;

impl MoveGenerator {
    pub fn generate_rook_attack_boards(square: Square, blockers: &[Bitboard]) -> Vec<Bitboard> {
        let mut attack_boards = vec![];
        for b in blockers.iter() {
            attack_boards.push(Self::generate_rook_attacks_otf(square, *b));
        }
        attack_boards
    }

    fn generate_rook_attacks_otf(square: Square, blockers: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::default();

        let target_rank = square / 8;
        let target_file = square % 8;

        for rank in (target_rank + 1)..8 {
            attacks.set_square(rank * 8 + target_file);
            if blockers.get_square(rank * 8 + target_file) {
                break;
            }
        }

        for rank in (0..target_rank).rev() {
            attacks.set_square(rank * 8 + target_file);
            if blockers.get_square(rank * 8 + target_file) {
                break;
            }
        }

        for file in (target_file + 1)..8 {
            attacks.set_square(target_rank * 8 + file);
            if blockers.get_square(target_rank * 8 + file) {
                break;
            }
        }

        for file in (0..target_file).rev() {
            attacks.set_square(target_rank * 8 + file);
            if blockers.get_square(target_rank * 8 + file) {
                break;
            }
        }

        attacks
    }

    pub fn generate_bishop_attack_boards(square: Square, blockers: &[Bitboard]) -> Vec<Bitboard> {
        let mut attack_boards = vec![];
        for b in blockers.iter() {
            attack_boards.push(Self::generate_bishop_attacks_otf(square, *b));
        }
        attack_boards
    }

    pub fn generate_bishop_attacks_otf(square: Square, blockers: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::default();

        let target_rank: u32 = square / 8;
        let target_file: u32 = square % 8;

        for (rank, file) in ((target_rank + 1)..8).zip((target_file + 1)..8) {
            attacks.set_square(rank * 8 + file);
            if blockers.get_square(rank * 8 + file) {
                break;
            }
        }

        for (rank, file) in (0..target_rank).rev().zip((target_file + 1)..8) {
            attacks.set_square(rank * 8 + file);
            if blockers.get_square(rank * 8 + file) {
                break;
            }
        }

        for (rank, file) in ((target_rank + 1)..8).zip((0..target_file).rev()) {
            attacks.set_square(rank * 8 + file);
            if blockers.get_square(rank * 8 + file) {
                break;
            }
        }

        for (rank, file) in (0..target_rank).rev().zip((0..target_file).rev()) {
            attacks.set_square(rank * 8 + file);
            if blockers.get_square(rank * 8 + file) {
                break;
            }
        }

        return attacks;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn generate_bishop_attacks_otf_tests() {
        assert_eq!(
            MoveGenerator::generate_bishop_attacks_otf(G5, Bitboard::default()),
            0x0810a000a0100804
        );
        assert_eq!(
            MoveGenerator::generate_bishop_attacks_otf(A1, Bitboard::default()),
            0x8040201008040200
        );

        assert_eq!(
            MoveGenerator::generate_bishop_attacks_otf(D4, Bitboard::new(0x0040020000040040)),
            0x0040221400142040
        );
    }

    #[test]
    fn generate_rook_attacks_otf_tests() {
        assert_eq!(
            MoveGenerator::generate_rook_attacks_otf(G5, Bitboard::default()),
            0x404040bf40404040
        );
        assert_eq!(
            MoveGenerator::generate_rook_attacks_otf(A1, Bitboard::default()),
            0x01010101010101fe
        );

        assert_eq!(
            MoveGenerator::generate_rook_attacks_otf(D4, Bitboard::new(0x0008000012000800)),
            0x0008080816080800
        );
    }
}
