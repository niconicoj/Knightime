use crate::{bitboard::Bitboard, constants::*, defs::Square, mov::*};

#[derive(Clone, Copy, Debug)]
pub enum Side {
    White,
    Black,
}

pub struct Player {
    side: Side,
    pawn_attacks: Vec<Bitboard>,
    king_attacks: Vec<Bitboard>,
    knight_attacks: Vec<Bitboard>,
}

impl Player {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            pawn_attacks: Self::init_pawns_attacks(side),
            knight_attacks: Self::init_knights_attacks(),
            king_attacks: Self::init_king_attacks(),
        }
    }

    fn init_pawns_attacks(side: Side) -> Vec<Bitboard> {
        let mut pawn_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            pawn_attacks.push(Self::mask_pawn_attacks(side, square));
        }
        pawn_attacks
    }

    fn init_knights_attacks() -> Vec<Bitboard> {
        let mut knight_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            knight_attacks.push(Self::mask_knight_attacks(square));
        }
        knight_attacks
    }

    fn init_king_attacks() -> Vec<Bitboard> {
        let mut king_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            king_attacks.push(Self::mask_king_attacks(square));
        }
        king_attacks
    }

    fn mask_pawn_attacks(side: Side, square: Square) -> Bitboard {
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

    fn mask_knight_attacks(square: Square) -> Bitboard {
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

    pub fn get_pawn_attacks(&self) -> Vec<Bitboard> {
        self.pawn_attacks.clone()
    }

    pub fn get_knight_attacks(&self) -> Vec<Bitboard> {
        self.knight_attacks.clone()
    }

    pub fn get_king_attacks(&self) -> Vec<Bitboard> {
        self.king_attacks.clone()
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

    #[test]
    fn mask_knight_attacks_tests() {
        // general case
        assert_eq!(Player::mask_knight_attacks(D4), 0x0000142200221400);
        //check each corner moves
        assert_eq!(Player::mask_knight_attacks(A1), 0x0000000000020400);
        assert_eq!(Player::mask_knight_attacks(H1), 0x0000000000402000);
        assert_eq!(Player::mask_knight_attacks(A8), 0x0004020000000000);
        assert_eq!(Player::mask_knight_attacks(H8), 0x0020400000000000);
        // check square on diagonals after the corners
        assert_eq!(Player::mask_knight_attacks(B2), 0x0000000005080008);
        assert_eq!(Player::mask_knight_attacks(G2), 0x00000000a0100010);
        assert_eq!(Player::mask_knight_attacks(B7), 0x0800080500000000);
        assert_eq!(Player::mask_knight_attacks(G7), 0x100010a000000000);
    }

    #[test]
    fn mask_king_attacks_tests() {
        // general case
        assert_eq!(Player::mask_king_attacks(D4), 0x0000001c141c0000);
        assert_eq!(Player::mask_king_attacks(H5), 0x0000c040c0000000);
        assert_eq!(Player::mask_king_attacks(A8), 0x0203000000000000);
    }
}
