use crate::{bitboard::Bitboard, constants::*};

#[derive(Clone, Copy, Debug)]
pub enum Side {
    White,
    Black,
}

pub struct Player {
    side: Side,
    pawn_attacks: Vec<Bitboard>,
    knight_attacks: Vec<Bitboard>,
}

impl Player {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            pawn_attacks: Self::init_pawns_attacks(side),
            knight_attacks: Self::init_knights_attacks(),
        }
    }

    fn init_pawns_attacks(side: Side) -> Vec<Bitboard> {
        let mut pawn_attacks: Vec<Bitboard> = vec![];
        for square in 0u64..64 {
            pawn_attacks.push(Self::mask_pawn_attacks(side, square));
        }
        pawn_attacks
    }

    fn init_knights_attacks() -> Vec<Bitboard> {
        let mut pawn_attacks: Vec<Bitboard> = vec![];
        for square in 0u64..64 {
            pawn_attacks.push(Self::mask_knight_attacks(square));
        }
        pawn_attacks
    }

    fn mask_pawn_attacks(side: Side, square: u64) -> Bitboard {
        let mut attacks: Bitboard = Bitboard::default();
        let mut bitboard: Bitboard = Bitboard::default();

        bitboard.set_square(square);

        match side {
            Side::White => {
                if (bitboard & A_FILE) == 0 {
                    attacks |= bitboard << 7;
                }
                if (bitboard & H_FILE) == 0 {
                    attacks |= bitboard << 9;
                }
            }
            Side::Black => {
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

    fn mask_knight_attacks(square: u64) -> Bitboard {

        let mut attacks: Bitboard = Bitboard::default();
        let mut bitboard: Bitboard = Bitboard::default();

        bitboard.set_square(square);

        if (bitboard & (H_FILE | RANK_78)) == 0 { attacks |= bitboard << 17 ;}
        if (bitboard & (A_FILE | RANK_78)) == 0 { attacks |= bitboard << 15 ;}
        if (bitboard & (HG_FILE | RANK_8)) == 0 { attacks |= bitboard << 10 ;}
        if (bitboard & (AB_FILE | RANK_8)) == 0 { attacks |= bitboard << 6 ;}

        if (bitboard & (A_FILE | RANK_12)) == 0 { attacks |= bitboard >> 17 ;}
        if (bitboard & (H_FILE | RANK_12)) == 0 { attacks |= bitboard >> 15 ;}
        if (bitboard & (AB_FILE | RANK_1)) == 0 { attacks |= bitboard >> 10 ;}
        if (bitboard & (HG_FILE | RANK_1)) == 0 { attacks |= bitboard >> 6 ;}

        return attacks;
    }

    pub fn get_pawn_attacks(&self) -> Vec<Bitboard> {
        self.pawn_attacks.clone()
    }

    pub fn get_knight_attacks(&self) -> Vec<Bitboard> {
        self.knight_attacks.clone()
    }

    pub fn get_side(&self) -> Side {
        self.side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_pawn_attacks_tests() {
        // tests white
        assert_eq!(
            Player::mask_pawn_attacks(Side::White, A2),
            0x0000000000020000
        );
        assert_eq!(
            Player::mask_pawn_attacks(Side::White, B2),
            0x0000000000050000
        );
        assert_eq!(
            Player::mask_pawn_attacks(Side::White, H2),
            0x0000000000400000
        );
        assert_eq!(
            Player::mask_pawn_attacks(Side::White, D8),
            0x0000000000000000
        );
        // tests black
        assert_eq!(
            Player::mask_pawn_attacks(Side::Black, A7),
            0x0000020000000000
        );
        assert_eq!(
            Player::mask_pawn_attacks(Side::Black, B7),
            0x0000050000000000
        );
        assert_eq!(
            Player::mask_pawn_attacks(Side::Black, H7),
            0x0000400000000000
        );
        assert_eq!(
            Player::mask_pawn_attacks(Side::Black, D1),
            0x0000000000000000
        );
    }
}
