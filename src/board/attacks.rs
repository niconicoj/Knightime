use crate::{
    bitboard::Bitboard,
    defs::{Piece, Side, Square},
};

use super::Board;

impl Board {
    pub fn get_attacked_squares(&self, side: Side) -> Bitboard {
        let mut bitboard = Bitboard::default();
        for square in 0..64 {
            if self.is_square_attacked(square, side) {
                bitboard.set_square(square);
            }
        }
        bitboard
    }

    pub fn is_square_attacked(&self, square: Square, side: Side) -> bool {
        if (self.bitboards[side as usize][Piece::Pawn as usize]
            & self
                .move_generator
                .get_pawn_attacks(square, side.get_opposite_side()))
            != 0
        {
            return true;
        }
        if (self.bitboards[side as usize][Piece::Knight as usize]
            & self.move_generator.get_knight_attacks(square))
            != 0
        {
            return true;
        }
        if (self.bitboards[side as usize][Piece::King as usize]
            & self.move_generator.get_king_attacks(square))
            != 0
        {
            return true;
        }

        if (self.bitboards[side as usize][Piece::Bishop as usize]
            & self
                .move_generator
                .get_bishop_attacks(square, self.occupancies[2]))
            != 0
        {
            return true;
        }

        if (self.bitboards[side as usize][Piece::Rook as usize]
            & self
                .move_generator
                .get_rook_attacks(square, self.occupancies[2]))
            != 0
        {
            return true;
        }

        if (self.bitboards[side as usize][Piece::Queen as usize]
            & self
                .move_generator
                .get_queen_attacks(square, self.occupancies[2]))
            != 0
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::*;

    use super::*;

    #[test]
    fn is_square_attacked_tests() {
        let board = Board::from_fen("8/8/8/3P4/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(board.is_square_attacked(C6, Side::White), true);
        assert_eq!(board.is_square_attacked(E6, Side::White), true);
        assert_eq!(board.is_square_attacked(B6, Side::White), false);
    }
}
