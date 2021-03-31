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
    }
}
