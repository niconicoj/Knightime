use crate::move_generator::movelist::Move;

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
    }
}
