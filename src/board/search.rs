use crate::move_generator::movelist::Move;

use super::Board;

impl Board {
    pub fn search(&self, depth: u32) -> Option<Move> {
        let moves = self.generate_moves();
        moves.get(0)
    }
}
