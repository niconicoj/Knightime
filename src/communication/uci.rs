use std::convert::TryFrom;

use crate::{board::Board, defs::Promotion, move_generator::movelist::Move};

impl Move {
    pub fn from_uci_string(uci_string: &str, board: &Board) -> Result<Move, UciError> {
        let move_list = board.generate_moves();
        let mut uci_iter = uci_string.chars();
        let source_file = match uci_iter.next() {
            Some(char) => match char {
                'a'..='h' => char as u32 - 97,
                _ => return Err(UciError::BadMoveFormat),
            },
            None => return Err(UciError::BadMoveFormat),
        };
        let source_rank = match uci_iter.next() {
            Some(char) => match char {
                '1'..='8' => char.to_digit(10).unwrap() - 1,
                _ => return Err(UciError::BadMoveFormat),
            },
            None => return Err(UciError::BadMoveFormat),
        };
        let target_file = match uci_iter.next() {
            Some(char) => match char {
                'a'..='h' => char as u32 - 97,
                _ => return Err(UciError::BadMoveFormat),
            },
            None => return Err(UciError::BadMoveFormat),
        };
        let target_rank = match uci_iter.next() {
            Some(char) => match char {
                '1'..='8' => char.to_digit(10).unwrap() - 1,
                _ => return Err(UciError::BadMoveFormat),
            },
            None => return Err(UciError::BadMoveFormat),
        };

        let source_square = source_file + source_rank * 8;
        let target_square = target_file + target_rank * 8;

        let promotion = match uci_iter.next().map(|c| Promotion::try_from(c)) {
            Some(r) => match r {
                Ok(promotion) => Some(promotion),
                Err(_) => return Err(UciError::BadMoveFormat),
            },
            None => None,
        };

        for mv in move_list {
            if (mv.get_source_square() == source_square)
                && (mv.get_target_square() == target_square)
            {
                if promotion == mv.get_promotion() {
                    return Ok(mv);
                }
            }
        }

        Err(UciError::IllegalMove)
    }
}

pub fn parse_uci_position(uci_string: &str) -> Result<Board, UciError> {
    enum Tokens {
        Nothing,
        Fen,
        Moves,
    }

    let parts: Vec<String> = uci_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let mut fen = String::from("");
    let mut moves: Vec<String> = Vec::new();
    let mut skip_fen = false;
    let mut token = Tokens::Nothing;

    for p in parts {
        match p {
            t if t == "position" => (), // Skip. We know we're parsing "position".
            t if t == "startpos" => skip_fen = true, // "fen" is now invalidated.
            t if t == "fen" && !skip_fen => token = Tokens::Fen,
            t if t == "moves" => token = Tokens::Moves,
            _ => match token {
                Tokens::Nothing => (),
                Tokens::Fen => {
                    fen.push_str(&p[..]);
                    fen.push(' ');
                }
                Tokens::Moves => moves.push(p),
            },
        }
    }
    let mut board = match fen.is_empty() {
        true => Board::default(),
        false => match Board::from_fen(fen.as_str()) {
            Ok(b) => b,
            Err(_) => return Err(UciError::BadPositionFormat),
        },
    };

    for mv_string in moves {
        let mv = match Move::from_uci_string(&mv_string, &board) {
            Ok(mv) => mv,
            Err(_) => return Err(UciError::BadMoveFormat),
        };
        match board.make_move(mv, false) {
            Err(_) => return Err(UciError::IllegalMove),
            _ => {}
        };
    }
    Ok(board)
}

pub fn parse_uci_go(uci_string: &str, board: &Board) -> Result<Move, UciError> {
    enum Tokens {
        Nothing,
        Depth,
    }

    let parts: Vec<String> = uci_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut depth: Option<u32> = None;
    let mut token = Tokens::Nothing;

    for p in parts {
        match p {
            t if t == "depth" => token = Tokens::Depth,
            _ => match token {
                Tokens::Depth => {
                    depth = match p.parse::<u32>() {
                        Ok(d) => Some(d),
                        Err(_) => return Err(UciError::BadGoFormat),
                    };
                }

                Tokens::Nothing => (),
            },
        }
    }

    match depth {
        Some(d) => {
            let mv = board.search(d);
            match mv {
                Some(m) => return Ok(m),
                None => return Err(UciError::NoAvailableMove),
            }
        }
        None => {}
    }
    Err(UciError::BadGoFormat)
}

#[derive(Debug, PartialEq)]
pub enum UciError {
    BadMoveFormat,
    BadGoFormat,
    BadPositionFormat,
    IllegalMove,
    NoAvailableMove,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;
    use crate::defs::*;

    #[test]
    fn move_from_uci_string_tests() {
        let board = Board::default();

        let uci_string = "e2e4";
        let mv = Move::from_uci_string(uci_string, &board).unwrap();
        assert_eq!(mv.get_source_square(), E2);
        assert_eq!(mv.get_target_square(), E4);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), None);
        assert_eq!(mv.get_capture(), false);
        assert_eq!(mv.get_double_push(), true);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_castling(), false);

        let uci_string = "e3e4";
        let mv = Move::from_uci_string(uci_string, &board);
        assert_eq!(mv, Err(UciError::IllegalMove));

        let uci_string = "3e4";
        let mv = Move::from_uci_string(uci_string, &board);
        assert_eq!(mv, Err(UciError::BadMoveFormat));
    }

    #[test]
    fn move_from_uci_string_promotion_tests() {
        let board = Board::from_fen("5k2/1p4p1/1R3p1p/5P1P/6b1/4K3/r2p4/8 b - - 1 44").unwrap();

        let uci_string = "d2d1q";
        let mv = Move::from_uci_string(uci_string, &board).unwrap();
        assert_eq!(mv.get_source_square(), D2);
        assert_eq!(mv.get_target_square(), D1);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), Some(Promotion::Queen));
        assert_eq!(mv.get_capture(), false);
        assert_eq!(mv.get_double_push(), false);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_castling(), false);

        let uci_string = "d2d1r";
        let mv = Move::from_uci_string(uci_string, &board).unwrap();
        assert_eq!(mv.get_source_square(), D2);
        assert_eq!(mv.get_target_square(), D1);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), Some(Promotion::Rook));
        assert_eq!(mv.get_capture(), false);
        assert_eq!(mv.get_double_push(), false);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_castling(), false);
    }
    #[test]
    fn parse_uci_position_tests() {
        let uci_position = "position startpos";

        let board = parse_uci_position(uci_position).unwrap();
        assert_eq!(board, Board::default());

        let uci_position =
            "position fen r1br2k1/p3ppbp/6p1/3pN3/8/1P2Q2P/P1q1RPP1/R1B3K1 b - - 2 16";
        let board = parse_uci_position(uci_position).unwrap();
        assert_eq!(
            board,
            Board::from_fen("r1br2k1/p3ppbp/6p1/3pN3/8/1P2Q2P/P1q1RPP1/R1B3K1 b - - 2 16").unwrap()
        );

        let uci_position = "position startpos moves e2e4";
        let uci_board = parse_uci_position(uci_position).unwrap();

        let mut target_board = Board::default();
        let mv = Move::new(E2, E4, Piece::Pawn, None, false, true, false, false);
        target_board.make_move(mv, false).unwrap();
        assert_eq!(uci_board, target_board);

        let uci_position =
            "position fen r1bqkbnr/pp1p1ppp/2n1p3/2p5/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq d3 0 4 moves d7d5 c4b5";
        let uci_board = parse_uci_position(uci_position).unwrap();

        let mut target_board =
            Board::from_fen("r1bqkbnr/pp1p1ppp/2n1p3/2p5/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq d3 0 4")
                .unwrap();
        let mv = Move::new(D7, D5, Piece::Pawn, None, false, true, false, false);
        target_board.make_move(mv, false).unwrap();
        let mv = Move::new(C4, B5, Piece::Bishop, None, false, false, false, false);
        target_board.make_move(mv, false).unwrap();
        assert_eq!(uci_board, target_board);
    }
}
