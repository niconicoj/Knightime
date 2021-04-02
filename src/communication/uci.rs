use std::convert::TryFrom;

use crate::{board::Board, defs::Promotion, move_generator::movelist::Move};

impl Move {
    pub fn from_uci_string(uci_string: String, board: &Board) -> Result<Move, UciError> {
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

#[derive(Debug, PartialEq)]
pub enum UciError {
    BadMoveFormat,
    IllegalMove,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;
    use crate::defs::*;

    #[test]
    fn move_from_uci_string_tests() {
        let board = Board::default();

        let uci_string = String::from("e2e4");
        let mv = Move::from_uci_string(uci_string, &board).unwrap();
        assert_eq!(mv.get_source_square(), E2);
        assert_eq!(mv.get_target_square(), E4);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), None);
        assert_eq!(mv.get_capture(), false);
        assert_eq!(mv.get_double_push(), true);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_castling(), false);

        let uci_string = String::from("e3e4");
        let mv = Move::from_uci_string(uci_string, &board);
        assert_eq!(mv, Err(UciError::IllegalMove));

        let uci_string = String::from("3e4");
        let mv = Move::from_uci_string(uci_string, &board);
        assert_eq!(mv, Err(UciError::BadMoveFormat));
    }

    #[test]
    fn move_from_uci_string_promotion_tests() {
        let board = Board::from_fen("5k2/1p4p1/1R3p1p/5P1P/6b1/4K3/r2p4/8 b - - 1 44").unwrap();

        let uci_string = String::from("d2d1q");
        let mv = Move::from_uci_string(uci_string, &board).unwrap();
        assert_eq!(mv.get_source_square(), D2);
        assert_eq!(mv.get_target_square(), D1);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), Some(Promotion::Queen));
        assert_eq!(mv.get_capture(), false);
        assert_eq!(mv.get_double_push(), false);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_castling(), false);

        let uci_string = String::from("d2d1r");
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
}
