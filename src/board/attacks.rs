use crate::{
    bitboard::Bitboard,
    constants::*,
    defs::{CastleRights, Piece, Promotion, Side, Square},
    move_generator::movelist::{Move, MoveList},
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

    pub fn generate_moves(&self, side: Side) -> MoveList {
        let mut movelist = MoveList::new();
        movelist.append_moves(&mut self.generate_pawn_moves(side));
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

        movelist
    }

    fn generate_pawn_moves(&self, side: Side) -> MoveList {
        let mut movelist = MoveList::new();
        let bitboard = self.bitboards[side as usize][Piece::Pawn as usize];

        for square in bitboard.into_iter() {
            movelist.append_moves(&mut self.generate_quiet_pawn_move(square, side));
            movelist.append_moves(&mut self.generate_noisy_pawn_move(square, side));
        }
        movelist
    }

    fn generate_quiet_pawn_move(&self, square: Square, side: Side) -> MoveList {
        let mut movelist = MoveList::new();
        let target_square = match side {
            Side::White => square + 8,
            Side::Black => square - 8,
        };
        if !(A2..=H7).contains(&target_square) && !self.occupancies[2].get_square(target_square) {
            #[rustfmt::skip]
            movelist.add_move(Move::new(square, target_square, Piece::Pawn, Promotion::Queen, false, false, false, false,));
            #[rustfmt::skip]
            movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::Rook, false, false, false, false,));
            #[rustfmt::skip]
            movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::Bishop, false, false, false, false,));
            #[rustfmt::skip]
            movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::Knight, false, false, false, false,));
        } else {
            if !self.occupancies[2].get_square(target_square) {
                #[rustfmt::skip]
                movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::None, false, false, false, false,));
            }
            let two_squares_target = match side {
                Side::White => square + 16,
                Side::Black => square - 16,
            };
            match side {
                Side::White => {
                    if (A2..=H2).contains(&square)
                        && !self.occupancies[2].get_square(target_square)
                        && !self.occupancies[2].get_square(two_squares_target)
                    {
                        #[rustfmt::skip]
                        movelist.add_move(Move::new( square, two_squares_target, Piece::Pawn, Promotion::None, false, true, false, false,));
                    }
                }
                Side::Black => {
                    if (A7..=H7).contains(&square)
                        && !self.occupancies[2].get_square(target_square)
                        && !self.occupancies[2].get_square(two_squares_target)
                    {
                        #[rustfmt::skip]
                        movelist.add_move(Move::new( square, two_squares_target, Piece::Pawn, Promotion::None, false, true, false, false,));
                    }
                }
            }
        }
        movelist
    }

    fn generate_noisy_pawn_move(&self, square: Square, side: Side) -> MoveList {
        let mut movelist = MoveList::new();
        let attacks = self.move_generator.get_pawn_attacks(square, side)
            & self.occupancies[side.get_opposite_side() as usize];

        for target_square in attacks.into_iter() {
            if !(A2..=H7).contains(&target_square) {
                // promotion happens
                #[rustfmt::skip]
                movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::Queen, true, false, false, false,));
                #[rustfmt::skip]
                movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::Rook, true, false, false, false,));
                #[rustfmt::skip]
                movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::Bishop, true, false, false, false,));
                #[rustfmt::skip]
                movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::Knight, true, false, false, false,));
            } else {
                #[rustfmt::skip]
                movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::None, true, false, false, false,));
            }
        }
        match self.en_passant_square {
            Some(en_passant_square) => {
                let en_passant_attacks = self.move_generator.get_pawn_attacks(square, side)
                    & (Bitboard(1u64) << en_passant_square);

                for target_square in en_passant_attacks.into_iter() {
                    #[rustfmt::skip]
                    movelist.add_move(Move::new( square, target_square, Piece::Pawn, Promotion::None, true, false, true, false,));
                }
            }
            None => {}
        }
        movelist
    }

    fn generate_castling_moves(&self, side: Side) -> MoveList {
        self.generate_castling_move(side, self.castling_rights[side as usize])
    }

    fn generate_castling_move(&self, side: Side, castle: CastleRights) -> MoveList {
        let mut movelist = MoveList::new();
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
                    movelist.add_move(Move::new(
                        CASTLE_SQUARE[side as usize][(castle as usize) - 1].0,
                        CASTLE_SQUARE[side as usize][(castle as usize) - 1].1,
                        Piece::King,
                        Promotion::None,
                        false,
                        false,
                        false,
                        true,
                    ));
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
                    movelist.add_move(Move::new(
                        CASTLE_SQUARE[side as usize][(castle as usize) - 1].0,
                        CASTLE_SQUARE[side as usize][(castle as usize) - 1].1,
                        Piece::King,
                        Promotion::None,
                        false,
                        false,
                        false,
                        true,
                    ));
                }
            }
            CastleRights::Both => {
                movelist
                    .append_moves(&mut self.generate_castling_move(side, CastleRights::KingSide));
                movelist
                    .append_moves(&mut self.generate_castling_move(side, CastleRights::QueenSide));
            }
        }
        movelist
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

    #[test]
    fn generate_simple_pawn_push_tests() {
        let board = Board::from_fen("8/3p4/8/8/8/8/3P4/8 w - - 0 1").unwrap();
        let pawn_moves = board.generate_quiet_pawn_move(D2, Side::White);
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(0).unwrap(),
            Move::new(D2, D3, Piece::Pawn, Promotion::None, false, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(1).unwrap(),
            Move::new(D2, D4, Piece::Pawn, Promotion::None, false, true, false, false)
        );
        assert_eq!(pawn_moves.get(2), None);
        let pawn_moves = board.generate_quiet_pawn_move(D7, Side::Black);
        println!("{}", pawn_moves);
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(0).unwrap(),
            Move::new(D7, D6, Piece::Pawn, Promotion::None, false, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(1).unwrap(),
            Move::new(D7, D5, Piece::Pawn, Promotion::None, false, true, false, false)
        );
        assert_eq!(pawn_moves.get(2), None);
    }

    #[test]
    fn generate_pawn_capture_moves_tests() {
        let board = Board::from_fen("3k2r1/5P2/8/6Pp/8/1p1p4/2P1P3/3K4 w - h6 0 1").unwrap();
        let pawn_moves = board.generate_noisy_pawn_move(C2, Side::White);
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(0).unwrap(),
            Move::new(C2, B3, Piece::Pawn, Promotion::None, true, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(1).unwrap(),
            Move::new(C2, D3, Piece::Pawn, Promotion::None, true, false, false, false)
        );
        assert_eq!(pawn_moves.get(2), None);
        let pawn_moves = board.generate_noisy_pawn_move(E2, Side::White);
        println!("{}", pawn_moves);
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(0).unwrap(),
            Move::new(E2, D3, Piece::Pawn, Promotion::None, true, false, false, false)
        );
        assert_eq!(pawn_moves.get(1), None);
        let pawn_moves = board.generate_noisy_pawn_move(G5, Side::White);
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(0).unwrap(),
            Move::new(G5, H6, Piece::Pawn, Promotion::None, true, false, true, false)
        );
        assert_eq!(pawn_moves.get(2), None);
        let pawn_moves = board.generate_noisy_pawn_move(F7, Side::White);
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(0).unwrap(),
            Move::new(F7, G8, Piece::Pawn, Promotion::Queen, true, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(1).unwrap(),
            Move::new(F7, G8, Piece::Pawn, Promotion::Rook, true, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(2).unwrap(),
            Move::new(F7, G8, Piece::Pawn, Promotion::Bishop, true, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(3).unwrap(),
            Move::new(F7, G8, Piece::Pawn, Promotion::Knight, true, false, false, false)
        );
        assert_eq!(pawn_moves.get(4), None);
    }

    #[test]
    fn generate_pawn_push_promotion_tests() {
        let board = Board::from_fen("1k6/1P1pP3/8/8/8/8/3P1K2/8 w - - 0 1").unwrap();
        let pawn_moves = board.generate_quiet_pawn_move(E7, Side::White);
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(0).unwrap(),
            Move::new(E7, E8, Piece::Pawn, Promotion::Queen, false, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(1).unwrap(),
            Move::new(E7, E8, Piece::Pawn, Promotion::Rook, false, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(2).unwrap(),
            Move::new(E7, E8, Piece::Pawn, Promotion::Bishop, false, false, false, false)
        );
        #[rustfmt::skip]
        assert_eq!(
            *pawn_moves.get(3).unwrap(),
            Move::new(E7, E8, Piece::Pawn, Promotion::Knight, false, false, false, false)
        );
        assert_eq!(pawn_moves.get(4), None);
        let pawn_moves = board.generate_quiet_pawn_move(B7, Side::White);
        println!("{}", pawn_moves);
        assert_eq!(pawn_moves.get(0), None);
    }

    #[test]
    fn generate_blocked_pawn_push_tests() {
        let board = Board::from_fen("8/3p4/3B4/8/8/3b4/3P4/8 w - - 0 1").unwrap();
        let pawn_moves = board.generate_quiet_pawn_move(D2, Side::White);
        assert_eq!(pawn_moves.get(0), None);
        let pawn_moves = board.generate_quiet_pawn_move(D7, Side::Black);
        assert_eq!(pawn_moves.get(0), None);
    }

    #[test]
    fn generate_castling_move_tests() {
        // White is in check in this position and should not be able to castle, black on the other hand can castle king side.
        let board =
            Board::from_fen("r1bqk2r/ppp2ppp/2n1pn2/3p4/1b1P4/3BPN2/PPP2PPP/RNBQK2R w KQkq - 0 1")
                .unwrap();
        let castling_moves = board.generate_castling_moves(Side::White);
        assert_eq!(castling_moves.get(0), None);
        let castling_moves = board.generate_castling_moves(Side::Black);
        #[rustfmt::skip]
        assert_eq!(
            *castling_moves.get(0).unwrap(),
            Move::new(E8, G8, Piece::King, Promotion::None, false, false, false, true)
        );
        assert_eq!(castling_moves.get(1), None);
        // here black is in check, but white can castle both sides
        let board =
            Board::from_fen("rnb1kbnr/ppp3pp/5p2/1BqpP1B1/8/2N1PN2/PPP1QPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        let castling_moves = board.generate_castling_moves(Side::White);
        #[rustfmt::skip]
        assert_eq!(
            *castling_moves.get(0).unwrap(),
            Move::new(E1, G1, Piece::King, Promotion::None, false, false, false, true)
        );
        #[rustfmt::skip]
        assert_eq!(
            *castling_moves.get(1).unwrap(),
            Move::new(E1, C1, Piece::King, Promotion::None, false, false, false, true)
        );
        assert_eq!(castling_moves.get(2), None);
        let castling_moves = board.generate_castling_moves(Side::Black);
        assert_eq!(castling_moves.get(1), None);
    }
}
