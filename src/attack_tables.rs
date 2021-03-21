use crate::{bitboard::Bitboard, constants::*, defs::Square};

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

pub fn generate_bishop_attacks() -> Vec<Bitboard> {
    let mut bishop_attacks = vec![];

    for square in 0..64 {
        bishop_attacks.push(mask_bishop_attacks(square));
    }
    bishop_attacks
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

pub fn generate_rook_attacks() -> Vec<Bitboard> {
    let mut rook_attacks = vec![];

    for square in 0..64 {
        rook_attacks.push(mask_rook_attacks(square));
    }
    rook_attacks
}

pub fn generate_rook_attacks_otf(square: Square, blockers: Bitboard) -> Bitboard {
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

pub fn set_occupancy(index: u64, mut attack_mask: Bitboard) -> Bitboard {
    let mut occupancy = Bitboard::default();

    let bit_count = attack_mask.count_occupied_squares();

    for count in 0..bit_count {
        let square = attack_mask.get_ls1b_index();
        attack_mask.clear_square(square);
        if (index & (1u64 << count)) != 0 {
            occupancy |= 1 << square;
        }
    }
    occupancy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_bishop_attacks_tests() {
        assert_eq!(mask_bishop_attacks(A1), 0x0040201008040200);
        assert_eq!(mask_bishop_attacks(D4), 0x0040221400142200);
        assert_eq!(mask_bishop_attacks(B3), 0x0020100804000400);
        assert_eq!(mask_bishop_attacks(G5), 0x0010200020100800);
    }

    #[test]
    fn generate_bishop_attacks_otf_tests() {
        assert_eq!(
            generate_bishop_attacks_otf(G5, Bitboard::default()),
            0x0810a000a0100804
        );
        assert_eq!(
            generate_bishop_attacks_otf(A1, Bitboard::default()),
            0x8040201008040200
        );

        assert_eq!(
            generate_bishop_attacks_otf(D4, Bitboard::new(0x0040020000040040)),
            0x0040221400142040
        );
    }

    #[test]
    fn generate_rook_attacks_otf_tests() {
        assert_eq!(
            generate_rook_attacks_otf(G5, Bitboard::default()),
            0x404040bf40404040
        );
        assert_eq!(
            generate_rook_attacks_otf(A1, Bitboard::default()),
            0x01010101010101fe
        );

        assert_eq!(
            generate_rook_attacks_otf(D4, Bitboard::new(0x0008000012000800)),
            0x0008080816080800
        );
    }

    #[test]
    fn mask_rook_attacks_tests() {
        assert_eq!(mask_rook_attacks(A4), 0x000101017e010100);
        assert_eq!(mask_rook_attacks(D4), 0x0008080876080800);
        assert_eq!(mask_rook_attacks(A1), 0x000101010101017e);
        assert_eq!(mask_rook_attacks(H8), 0x7e80808080808000);
        assert_eq!(mask_rook_attacks(H4), 0x008080807e808000);
    }
}
