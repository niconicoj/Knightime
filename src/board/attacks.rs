use crate::{
    bitboard::Bitboard,
    constants::*,
    defs::{CastleRights, Piece, Promotion, Side, Square},
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

    pub fn generate_moves(&self, side: Side) {
        println!("moves for {} :", SIDE_NAME[side as usize]);
        self.generate_pawn_moves(side);
        println!();
        self.generate_castling_moves(side);
        println!();
        self.generate_knight_moves(side);
        println!();
        self.generate_king_moves(side);
        println!();
        self.generate_bishop_moves(side);
        println!();
        self.generate_rook_moves(side);
        println!();
    }

    fn generate_pawn_moves(&self, side: Side) {
        let bitboard = self.bitboards[side as usize][Piece::Pawn as usize];

        for square in bitboard.into_iter() {
            self.generate_quiet_pawn_move(square, side);
            self.generate_noisy_pawn_move(square, side);
        }
    }

    fn generate_quiet_pawn_move(&self, square: Square, side: Side) {
        let target_square = match side {
            Side::White => square + 8,
            Side::Black => square - 8,
        };
        if !(A2..=H7).contains(&target_square) {
            // promotion happens
            print!(
                "{}={} ",
                SQUARE_NAME[target_square as usize],
                UNICODE_PIECE[side as usize][Promotion::Queen as usize]
            );
            print!(
                "{}={} ",
                SQUARE_NAME[target_square as usize],
                UNICODE_PIECE[side as usize][Promotion::Rook as usize]
            );
            print!(
                "{}={} ",
                SQUARE_NAME[target_square as usize],
                UNICODE_PIECE[side as usize][Promotion::Bishop as usize]
            );
            print!(
                "{}={} ",
                SQUARE_NAME[target_square as usize],
                UNICODE_PIECE[side as usize][Promotion::Knight as usize]
            );
        } else {
            if !self.occupancies[2].get_square(target_square) {
                print!("{} ", SQUARE_NAME[target_square as usize]);
            }
            let two_squares_target = match side {
                Side::White => square + 16,
                Side::Black => square - 16,
            };
            match side {
                Side::White => {
                    if (A2..=H2).contains(&square)
                        && !self.occupancies[2].get_square(two_squares_target)
                    {
                        print!("{} ", SQUARE_NAME[two_squares_target as usize]);
                    }
                }
                Side::Black => {
                    if (A7..=H7).contains(&square)
                        && !self.occupancies[2].get_square(two_squares_target)
                    {
                        print!("{} ", SQUARE_NAME[two_squares_target as usize]);
                    }
                }
            }
        }
    }

    fn generate_noisy_pawn_move(&self, square: Square, side: Side) {
        let attacks = self.move_generator.get_pawn_attacks(square, side)
            & self.occupancies[side.get_opposite_side() as usize];

        for target_square in attacks.into_iter() {
            if !(A2..=H7).contains(&target_square) {
                // promotion happens
                print!(
                    "{}×{}={} ",
                    SQUARE_NAME[square as usize].chars().next().unwrap(),
                    SQUARE_NAME[target_square as usize],
                    UNICODE_PIECE[side as usize][Promotion::Queen as usize]
                );
                print!(
                    "{}×{}={} ",
                    SQUARE_NAME[square as usize].chars().next().unwrap(),
                    SQUARE_NAME[target_square as usize],
                    UNICODE_PIECE[side as usize][Promotion::Rook as usize]
                );
                print!(
                    "{}×{}={} ",
                    SQUARE_NAME[square as usize].chars().next().unwrap(),
                    SQUARE_NAME[target_square as usize],
                    UNICODE_PIECE[side as usize][Promotion::Bishop as usize]
                );
                print!(
                    "{}×{}={} ",
                    SQUARE_NAME[square as usize].chars().next().unwrap(),
                    SQUARE_NAME[target_square as usize],
                    UNICODE_PIECE[side as usize][Promotion::Knight as usize]
                );
            } else {
                print!(
                    "{}×{} ",
                    SQUARE_NAME[square as usize].chars().next().unwrap(),
                    SQUARE_NAME[target_square as usize]
                );
            }
        }
        match self.en_passant_square {
            Some(en_passant_square) => {
                let en_passant_attacks = self.move_generator.get_pawn_attacks(square, side)
                    & (Bitboard(1u64) << en_passant_square);

                for target_square in en_passant_attacks.into_iter() {
                    print!(
                        "{}×{} ",
                        SQUARE_NAME[square as usize].chars().next().unwrap(),
                        SQUARE_NAME[target_square as usize]
                    );
                }
            }
            None => {}
        }
    }

    fn generate_castling_moves(&self, side: Side) {
        self.generate_castling_move(side, self.castling_rights[side as usize]);
    }

    fn generate_castling_move(&self, side: Side, castle: CastleRights) {
        match castle {
            CastleRights::None => {}
            CastleRights::KingSide => {
                let blockers = match side {
                    Side::White => (Bitboard(0x60), (E1, F1)),
                    Side::Black => (Bitboard(0x6000000000000000), (E8, F8)),
                };
                if ((blockers.0 & self.occupancies[2]) == 0)
                    && (!self.is_square_attacked(blockers.1 .0, side.get_opposite_side()))
                    && (!self.is_square_attacked(blockers.1 .1, side.get_opposite_side()))
                {
                    print!("O-O ");
                }
            }
            CastleRights::QueenSide => {
                let blockers = match side {
                    Side::White => (Bitboard(0xe), (E1, D1)),
                    Side::Black => (Bitboard(0x0e00000000000000), (E8, D8)),
                };
                if ((blockers.0 & self.occupancies[2]) == 0)
                    && (!self.is_square_attacked(blockers.1 .0, side.get_opposite_side()))
                    && (!self.is_square_attacked(blockers.1 .1, side.get_opposite_side()))
                {
                    print!("O-O-O ");
                }
            }
            CastleRights::Both => {
                self.generate_castling_move(side, CastleRights::KingSide);
                self.generate_castling_move(side, CastleRights::QueenSide);
            }
        }
    }

    fn generate_knight_moves(&self, side: Side) {
        for square in self.bitboards[side as usize][Piece::Knight as usize].into_iter() {
            let quiet_moves = self.move_generator.get_knight_attacks(square) & !self.occupancies[2];

            for target_square in quiet_moves.into_iter() {
                print!(
                    "{}{} ",
                    UNICODE_PIECE[side as usize][Piece::Knight as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }

            let captures = self.move_generator.get_knight_attacks(square)
                & self.occupancies[side.get_opposite_side() as usize];

            for target_square in captures.into_iter() {
                print!(
                    "{}×{} ",
                    UNICODE_PIECE[side as usize][Piece::Knight as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }
        }
    }

    fn generate_king_moves(&self, side: Side) {
        for square in self.bitboards[side as usize][Piece::King as usize].into_iter() {
            let quiet_moves = self.move_generator.get_king_attacks(square) & !self.occupancies[2];

            for target_square in quiet_moves.into_iter() {
                print!(
                    "{}{} ",
                    UNICODE_PIECE[side as usize][Piece::King as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }

            let captures = self.move_generator.get_king_attacks(square)
                & self.occupancies[side.get_opposite_side() as usize];

            for target_square in captures.into_iter() {
                print!(
                    "{}×{} ",
                    UNICODE_PIECE[side as usize][Piece::King as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }
        }
    }

    fn generate_bishop_moves(&self, side: Side) {
        for square in self.bitboards[side as usize][Piece::Bishop as usize].into_iter() {
            let quiet_moves = self
                .move_generator
                .get_bishop_attacks(square, self.occupancies[2])
                & !self.occupancies[2];

            for target_square in quiet_moves.into_iter() {
                print!(
                    "{}{} ",
                    UNICODE_PIECE[side as usize][Piece::Bishop as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }

            let captures = self
                .move_generator
                .get_bishop_attacks(square, self.occupancies[2])
                & self.occupancies[side.get_opposite_side() as usize];

            for target_square in captures.into_iter() {
                print!(
                    "{}×{} ",
                    UNICODE_PIECE[side as usize][Piece::Bishop as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }
        }
    }

    fn generate_rook_moves(&self, side: Side) {
        for square in self.bitboards[side as usize][Piece::Rook as usize].into_iter() {
            let quiet_moves = self
                .move_generator
                .get_rook_attacks(square, self.occupancies[2])
                & !self.occupancies[2];

            for target_square in quiet_moves.into_iter() {
                print!(
                    "{}{} ",
                    UNICODE_PIECE[side as usize][Piece::Rook as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }

            let captures = self
                .move_generator
                .get_rook_attacks(square, self.occupancies[2])
                & self.occupancies[side.get_opposite_side() as usize];

            for target_square in captures.into_iter() {
                print!(
                    "{}×{} ",
                    UNICODE_PIECE[side as usize][Piece::Rook as usize],
                    SQUARE_NAME[target_square as usize],
                );
            }
        }
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
