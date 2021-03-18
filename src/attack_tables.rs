use crate::bitboard::Bitboard;

pub fn mask_bishop_attacks(square: u64) -> Bitboard {
    let mut attacks = Bitboard::default();

    let target_rank = square / 8;
    let target_file = square % 8;

    for (rank, file) in ((target_rank + 1)..7).zip((target_file + 1)..7) {
        attacks.set_square(rank * 8 + file);
    }

    for (rank, file) in (1..target_rank).rev().zip((target_file + 1)..7) {
        attacks.set_square(rank * 8 + file);
    }

    for (rank, file) in ((target_rank + 1)..7).zip((1..target_file).rev()) {
        attacks.set_square(rank * 8 + file);
    }

    for (rank, file) in (1..target_rank).rev().zip((1..target_file).rev()) {
        attacks.set_square(rank * 8 + file);
    }

    return attacks;
}

pub fn generate_bishop_attacks() -> Vec<Bitboard> {
    let mut bishop_attacks = vec![];

    for square in 0u64..64 {
        bishop_attacks.push(mask_bishop_attacks(square));
    }
    bishop_attacks
}
