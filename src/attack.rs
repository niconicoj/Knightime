use crate::{bitboard::Bitboard, constants::*};

pub fn mask_pawn_attacks(side: Player, square: u64) -> Bitboard {

    let mut attacks: Bitboard = Bitboard::default();
    let mut bitboard: Bitboard = Bitboard::default();

    bitboard.set_square(square);


    match side {
        Player::White => {
            if (bitboard & A_FILE) == 0 {
                attacks |= bitboard << 7;
            }
            if (bitboard & H_FILE) == 0 {
                attacks |= bitboard << 9;
            }
        },
        Player::Black => {
            if (bitboard & H_FILE) == 0 {
                attacks |= bitboard >> 7;
            }
            if (bitboard & A_FILE) == 0 {
                attacks |= bitboard >> 9;
            }
        }
    }

    return attacks;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_pawn_attacks_tests() {
        // tests white
        assert_eq!(mask_pawn_attacks(Player::White, A2),0x0000000000020000);
        assert_eq!(mask_pawn_attacks(Player::White, B2),0x0000000000050000);
        assert_eq!(mask_pawn_attacks(Player::White, H2),0x0000000000400000);
        assert_eq!(mask_pawn_attacks(Player::White, D8),0x0000000000000000);
        // tests black
        assert_eq!(mask_pawn_attacks(Player::Black, A7),0x0000020000000000);
        assert_eq!(mask_pawn_attacks(Player::Black, B7),0x0000050000000000);
        assert_eq!(mask_pawn_attacks(Player::Black, H7),0x0000400000000000);
        assert_eq!(mask_pawn_attacks(Player::Black, D1),0x0000000000000000);
    }
}