use crate::{bitboard::Bitboard, constants::*};

pub fn mask_bishop_attacks(square: u8) -> Bitboard {
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

pub fn generate_bishop_attacks() -> Vec<Bitboard> {
    let mut bishop_attacks = vec![];

    for square in 0u8..64 {
        bishop_attacks.push(mask_bishop_attacks(square));
    }
    bishop_attacks
}

pub fn mask_rook_attacks(square: u8) -> Bitboard {
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
    fn mask_rook_attacks_tests() {
        assert_eq!(mask_rook_attacks(A4), 0x000101017e010100);
        assert_eq!(mask_rook_attacks(D4), 0x0008080876080800);
        assert_eq!(mask_rook_attacks(A1), 0x000101010101017e);
        assert_eq!(mask_rook_attacks(H8), 0x7e80808080808000);
        assert_eq!(mask_rook_attacks(H4), 0x008080807e808000);
    }
}
