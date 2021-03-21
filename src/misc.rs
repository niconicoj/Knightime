use rand::prelude::*;

use crate::{
    attack_tables::{
        generate_bishop_attacks_otf, generate_rook_attacks_otf, mask_bishop_attacks,
        mask_rook_attacks, set_occupancy,
    },
    bitboard::Bitboard,
    defs::{Piece, Square},
};

/// generate candidate magic numbers with a low numbers of active bits
pub fn generate_magic_number() -> u64 {
    let mut rng = rand::thread_rng();

    let seed1: u64 = rng.gen();
    let seed2: u64 = rng.gen();
    let seed3: u64 = rng.gen();

    seed1 & seed2 & seed3
}

pub fn find_magic(square: Square, relevant_bits: u32, piece: Piece) -> Option<u64> {
    let mut occupancies: Vec<Bitboard> = vec![];
    let mut attacks: Vec<Bitboard> = vec![];
    let mut used_attacks: Vec<Bitboard> = vec![];

    let attack_mask = match piece {
        Piece::Rook => mask_rook_attacks(square),
        Piece::Bishop => mask_bishop_attacks(square),
    };

    let occupancy_index = 1 << relevant_bits;

    for index in 0..occupancy_index {
        let occupancy = set_occupancy(index, attack_mask);
        occupancies.push(occupancy);

        let attack = match piece {
            Piece::Rook => generate_rook_attacks_otf(square, occupancy),
            Piece::Bishop => generate_bishop_attacks_otf(square, occupancy),
        };

        attacks.push(attack);
    }

    for _count in 0..1000000000 {
        let magic_number = generate_magic_number();

        // skip inappropriate magic
        if ((magic_number.overflowing_mul(attack_mask.get_value()).0) & 0xFF00000000000000)
            .count_ones()
            < 6
        {
            continue;
        };

        let mut fail_flag = false;

        for index in 0..occupancy_index {
            let magic_index: u64 = (magic_number
                .overflowing_mul(occupancies.get(index as usize).unwrap().get_value())
                .0)
                >> (64 - relevant_bits);

            match used_attacks.get(magic_index as usize) {
                Some(used_attack) => {
                    if used_attack != attacks.get(magic_index as usize).unwrap() {
                        // magic index doesn't work, on to the next one
                        fail_flag = true;
                        break;
                    }
                }
                None => {
                    used_attacks.push(*attacks.get(magic_index as usize).unwrap());
                }
            }
        }

        if !fail_flag {
            return Some(magic_number);
        }
    }

    return None;
}
