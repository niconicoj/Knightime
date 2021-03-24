use crate::{
    bitboard::Bitboard,
    constants::*,
    defs::{Side, Square},
    mov::*,
};

use super::MoveGenerator;

impl MoveGenerator {
    pub fn mask_pawn_attacks(side: Side, square: Square) -> Bitboard {
        let mut attacks: Bitboard = Bitboard::default();
        let mut bitboard: Bitboard = Bitboard::default();

        bitboard.set_square(square);

        match side {
            Side::White => {
                attacks |= move_nw(bitboard) | move_ne(bitboard);
            }
            Side::Black => {
                attacks |= move_sw(bitboard) | move_se(bitboard);
            }
        }
        return attacks;
    }

    pub fn mask_knight_attacks(square: Square) -> Bitboard {
        let mut attacks: Bitboard = Bitboard::default();
        let mut bitboard: Bitboard = Bitboard::default();

        bitboard.set_square(square);

        if (bitboard & (FILE_H | RANK_78)) == 0 {
            attacks |= bitboard << 17u32;
        }
        if (bitboard & (FILE_A | RANK_78)) == 0 {
            attacks |= bitboard << 15u32;
        }
        if (bitboard & (FILE_HG | RANK_8)) == 0 {
            attacks |= bitboard << 10u32;
        }
        if (bitboard & (FILE_AB | RANK_8)) == 0 {
            attacks |= bitboard << 6u32;
        }

        if (bitboard & (FILE_A | RANK_12)) == 0 {
            attacks |= bitboard >> 17u32;
        }
        if (bitboard & (FILE_H | RANK_12)) == 0 {
            attacks |= bitboard >> 15u32;
        }
        if (bitboard & (FILE_AB | RANK_1)) == 0 {
            attacks |= bitboard >> 10u32;
        }
        if (bitboard & (FILE_HG | RANK_1)) == 0 {
            attacks |= bitboard >> 6u32;
        }

        return attacks;
    }

    pub fn mask_king_attacks(square: Square) -> Bitboard {
        let mut bitboard = Bitboard::default();
        bitboard.set_square(square);

        let mut attacks = move_e(bitboard) | move_w(bitboard);
        bitboard |= attacks;
        attacks |= move_n(bitboard) | move_s(bitboard);

        return attacks;
    }

    pub fn mask_rook_attacks(square: Square) -> Bitboard {
        let mut bitboard = Bitboard::default();

        bitboard.set_square(square);

        let target_rank = square / 8;
        let target_file = square % 8;

        let mut attacks = Bitboard::new((FILE_A << target_file) | (RANK_1 << target_rank * 8));

        attacks &= !bitboard
            & !(bitboard >> target_file)
            & !(bitboard << (7 - target_file))
            & !(bitboard >> (target_rank * 8))
            & !(bitboard << ((7 - target_rank) * 8));

        attacks
    }

    pub fn mask_bishop_attacks(square: Square) -> Bitboard {
        let mut bitboard = Bitboard::default();

        bitboard.set_square(square);

        let target_rank: i32 = (square / 8) as i32;
        let target_file: i32 = (square % 8) as i32;

        let offset_ah: i32 = target_file - target_rank;
        let offset_ha: i32 = target_file - (8 - target_rank) + 1;

        let mut attacks = match offset_ah.cmp(&0i32) {
            std::cmp::Ordering::Less => Bitboard::new(DIAGONAL_AH << offset_ah.abs() * 8),
            _ => Bitboard::new(DIAGONAL_AH >> offset_ah * 8),
        };

        match offset_ha.cmp(&0i32) {
            std::cmp::Ordering::Less => attacks |= DIAGONAL_HA >> offset_ha.abs() * 8,
            _ => attacks |= DIAGONAL_HA << offset_ha * 8,
        };

        return attacks & !bitboard & !FILE_A & !FILE_H & !RANK_1 & !RANK_8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_pawn_attacks_tests() {
        // tests white
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::White, A2),
            0x0000000000020000
        );
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::White, B2),
            0x0000000000050000
        );
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::White, H2),
            0x0000000000400000
        );
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::White, D8),
            0x0000000000000000
        );
        // tests black
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::Black, A7),
            0x0000020000000000
        );
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::Black, B7),
            0x0000050000000000
        );
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::Black, H7),
            0x0000400000000000
        );
        assert_eq!(
            MoveGenerator::mask_pawn_attacks(Side::Black, D1),
            0x0000000000000000
        );
    }

    #[test]
    fn mask_knight_attacks_tests() {
        // general case
        assert_eq!(MoveGenerator::mask_knight_attacks(D4), 0x0000142200221400);
        //check each corner moves
        assert_eq!(MoveGenerator::mask_knight_attacks(A1), 0x0000000000020400);
        assert_eq!(MoveGenerator::mask_knight_attacks(H1), 0x0000000000402000);
        assert_eq!(MoveGenerator::mask_knight_attacks(A8), 0x0004020000000000);
        assert_eq!(MoveGenerator::mask_knight_attacks(H8), 0x0020400000000000);
        // check square on diagonals after the corners
        assert_eq!(MoveGenerator::mask_knight_attacks(B2), 0x0000000005080008);
        assert_eq!(MoveGenerator::mask_knight_attacks(G2), 0x00000000a0100010);
        assert_eq!(MoveGenerator::mask_knight_attacks(B7), 0x0800080500000000);
        assert_eq!(MoveGenerator::mask_knight_attacks(G7), 0x100010a000000000);
    }

    #[test]
    fn mask_king_attacks_tests() {
        // general case
        assert_eq!(MoveGenerator::mask_king_attacks(D4), 0x0000001c141c0000);
        assert_eq!(MoveGenerator::mask_king_attacks(H5), 0x0000c040c0000000);
        assert_eq!(MoveGenerator::mask_king_attacks(A8), 0x0203000000000000);
    }

    #[test]
    fn mask_bishop_attacks_tests() {
        assert_eq!(MoveGenerator::mask_bishop_attacks(A1), 0x0040201008040200);
        assert_eq!(MoveGenerator::mask_bishop_attacks(D4), 0x0040221400142200);
        assert_eq!(MoveGenerator::mask_bishop_attacks(B3), 0x0020100804000400);
        assert_eq!(MoveGenerator::mask_bishop_attacks(G5), 0x0010200020100800);
    }

    #[test]
    fn mask_rook_attacks_tests() {
        assert_eq!(MoveGenerator::mask_rook_attacks(A4), 0x000101017e010100);
        assert_eq!(MoveGenerator::mask_rook_attacks(D4), 0x0008080876080800);
        assert_eq!(MoveGenerator::mask_rook_attacks(A1), 0x000101010101017e);
        assert_eq!(MoveGenerator::mask_rook_attacks(H8), 0x7e80808080808000);
        assert_eq!(MoveGenerator::mask_rook_attacks(H4), 0x008080807e808000);
    }
}
