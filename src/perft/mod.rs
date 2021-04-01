use crate::{board::Board, move_generator::movelist::Move};

pub struct Perft {
    board: Board,
    pub nodes: u64,
    pub captures: u64,
    pub en_passants: u64,
    pub castles: u64,
    pub promotions: u64,
}

impl Perft {
    pub fn default() -> Self {
        Self {
            board: Board::default(),
            nodes: 0,
            captures: 0,
            en_passants: 0,
            castles: 0,
            promotions: 0,
        }
    }

    pub fn new(board: Board) -> Self {
        Self {
            board,
            nodes: 0,
            captures: 0,
            en_passants: 0,
            castles: 0,
            promotions: 0,
        }
    }

    pub fn run(&mut self, depth: u64) {
        if depth == 0 {
            // end of iteration reach
            self.nodes += 1;
            return;
        }

        let moves = self.board.generate_moves();

        for mv in moves.into_iter() {
            match self.board.make_move(mv, false) {
                Err(_) => {
                    continue;
                }
                Ok(_) => {
                    self.run(depth - 1);
                    self.board.take_back_move();
                }
            }
        }
    }

    pub fn detailed_run(&mut self, depth: u64, leaf_node: Option<Move>) {
        if depth == 0 {
            // end of iteration reach
            let mv = leaf_node.unwrap();
            self.nodes += 1;
            self.captures += mv.get_capture() as u64;
            self.en_passants += mv.get_en_passant() as u64;
            self.castles += mv.get_castling() as u64;
            self.promotions += mv.get_promotion().is_some() as u64;
            return;
        }

        let moves = self.board.generate_moves();

        for mv in moves.into_iter() {
            match self.board.make_move(mv, false) {
                Err(_) => {
                    continue;
                }
                Ok(_) => {
                    self.detailed_run(depth - 1, Some(mv));
                    self.board.take_back_move();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_position_perft_test() {
        let board = Board::default();
        let mut perft = Perft::new(board);
        perft.detailed_run(4, None);
        assert_eq!(perft.nodes, 197_281);
        assert_eq!(perft.captures, 1_576);
        assert_eq!(perft.en_passants, 0);
        assert_eq!(perft.castles, 0);
        assert_eq!(perft.promotions, 0);
    }
    #[test]
    fn position_2_perft_test() {
        let board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
                .unwrap();
        let mut perft = Perft::new(board);
        perft.detailed_run(3, None);
        assert_eq!(perft.nodes, 97_862);
        assert_eq!(perft.captures, 17_102);
        assert_eq!(perft.en_passants, 45);
        assert_eq!(perft.castles, 3162);
        assert_eq!(perft.promotions, 0);
    }

    #[test]
    fn position_3_perft_test() {
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();
        let mut perft = Perft::new(board);
        perft.detailed_run(5, None);
        assert_eq!(perft.nodes, 674_624);
        assert_eq!(perft.captures, 52_051);
        assert_eq!(perft.en_passants, 1165);
        assert_eq!(perft.castles, 0);
        assert_eq!(perft.promotions, 0);
    }

    #[test]
    fn position_4_perft_test() {
        let board =
            Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
                .unwrap();
        let mut perft = Perft::new(board);
        perft.detailed_run(4, None);
        assert_eq!(perft.nodes, 422_333);
        assert_eq!(perft.captures, 131_393);
        assert_eq!(perft.en_passants, 0);
        assert_eq!(perft.castles, 7795);
        assert_eq!(perft.promotions, 60_032);
    }

    #[test]
    fn position_5_perft_test() {
        let board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        let mut perft = Perft::new(board);
        perft.run(3);
        assert_eq!(perft.nodes, 62_379);
    }
}
