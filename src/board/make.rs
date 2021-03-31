use std::hint::unreachable_unchecked;

use crate::{
    constants::*,
    defs::{CastleRights, Piece, Side},
    move_generator::movelist::Move,
};

use super::Board;

impl Board {
    pub fn make_move(&mut self, mv: Move, only_capture: bool) {
        if mv.get_capture() && only_capture {
            return;
        }
        self.store_state();

        // clear source square and set target square on the correct bitboard
        self.state.bitboards[self.state.side_to_move as usize][mv.get_piece() as usize]
            .clear_square(mv.get_source_square());
        self.state.bitboards[self.state.side_to_move as usize][mv.get_piece() as usize]
            .set_square(mv.get_target_square());

        // clear target square on opposite bitboard if move is a capture
        if mv.get_capture() {
            for bitboard in self.state.bitboards
                [self.state.side_to_move.get_opposite_side() as usize]
                .iter_mut()
            {
                bitboard.clear_square(mv.get_target_square());
            }
        }

        // handle promotion
        match mv.get_promotion() {
            Some(promotion) => {
                // clear pawn from target square
                self.state.bitboards[self.state.side_to_move as usize][mv.get_piece() as usize]
                    .clear_square(mv.get_target_square());
                // add new bit on the correct piece bitboard
                self.state.bitboards[self.state.side_to_move as usize][promotion as usize]
                    .set_square(mv.get_target_square())
            }
            None => {}
        }

        // handle en passant
        if mv.get_en_passant() {
            // clear the correct pawn bit
            match self.state.side_to_move {
                Side::White => self.state.bitboards[Side::Black as usize][Piece::Pawn as usize]
                    .clear_square(mv.get_target_square() - 8),
                Side::Black => self.state.bitboards[Side::White as usize][Piece::Pawn as usize]
                    .clear_square(mv.get_target_square() + 8),
            };
        }
        // in any case, reset en passant square
        self.state.en_passant_square = None;

        // in case of double push, set an en passant target square
        if mv.get_double_push() {
            match self.state.side_to_move {
                Side::White => self.state.en_passant_square = Some(mv.get_target_square() - 8),
                Side::Black => self.state.en_passant_square = Some(mv.get_target_square() + 8),
            };
        }

        if mv.get_castling() {
            match mv.get_target_square() {
                G1 => {
                    self.state.bitboards[Side::White as usize][Piece::Rook as usize]
                        .clear_square(H1);
                    self.state.bitboards[Side::White as usize][Piece::Rook as usize].set_square(F1);
                }
                C1 => {
                    self.state.bitboards[Side::White as usize][Piece::Rook as usize]
                        .clear_square(A1);
                    self.state.bitboards[Side::White as usize][Piece::Rook as usize].set_square(D1);
                }
                G8 => {
                    self.state.bitboards[Side::Black as usize][Piece::Rook as usize]
                        .clear_square(H8);
                    self.state.bitboards[Side::Black as usize][Piece::Rook as usize].set_square(F8);
                }
                C8 => {
                    self.state.bitboards[Side::Black as usize][Piece::Rook as usize]
                        .clear_square(A8);
                    self.state.bitboards[Side::Black as usize][Piece::Rook as usize].set_square(D8);
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }

        self.state.castling_rights[self.state.side_to_move as usize] = self.state.castling_rights
            [self.state.side_to_move as usize]
            & CASTLING_RIGHTS_UPDATE_TABLE[mv.get_source_square() as usize];
        self.state.castling_rights[self.state.side_to_move.get_opposite_side() as usize] =
            self.state.castling_rights[self.state.side_to_move.get_opposite_side() as usize]
                & CASTLING_RIGHTS_UPDATE_TABLE[mv.get_target_square() as usize];
    }
}
