use std::{convert::TryFrom, fmt, hint::unreachable_unchecked};

use crate::constants::{SQUARE_NAME, UNICODE_PIECE};
use crate::defs::{Piece, Promotion, Square};
use crate::move_generator::defs::*;

#[derive(Debug)]
pub struct MoveList(Vec<Move>);

impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for mv in &self.0 {
            write!(f, "{} ", mv)?;
        }
        Ok(())
    }
}

impl IntoIterator for MoveList {
    type Item = Move;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn get(&self, index: usize) -> Option<Move> {
        let el = self.0.get(index);

        match el {
            Some(e) => return Some(e.clone()),
            None => return None,
        }
    }

    pub fn add_move(&mut self, mv: Move) {
        self.0.push(mv);
    }

    pub fn append_moves(&mut self, movelist: &mut MoveList) {
        self.0.append(&mut movelist.0);
    }

    pub fn contains(&self, mv: Move) -> bool {
        self.0.contains(&mv)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Move(u32);

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_piece() {
            Piece::Pawn => {}
            _ => {
                write!(f, "{}", UNICODE_PIECE[0][self.get_piece() as usize])?;
            }
        };
        write!(f, "{}", SQUARE_NAME[self.get_source_square() as usize])?;
        if self.get_capture() {
            write!(f, "x")?;
        }
        write!(f, "{}", SQUARE_NAME[self.get_target_square() as usize])?;
        match self.get_promotion() {
            Some(promotion) => {
                write!(f, "={}", UNICODE_PIECE[0][promotion as usize])?;
            }
            None => {}
        }
        Ok(())
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Move {
    pub fn new(
        source_square: Square,
        target_square: Square,
        piece: Piece,
        promotion: Option<Promotion>,
        capture: bool,
        double_push: bool,
        en_passant: bool,
        castling: bool,
    ) -> Self {
        Self(
            source_square
                | (target_square << TARGET_SQUARE_SHIFT)
                | ((piece as u32) << PIECE_SHIFT)
                | ((match promotion {
                    Some(promotion) => promotion as u32,
                    None => 0,
                }) << PROMOTION_SHIFT)
                | ((capture as u32) << CAPTURE_SHIFT)
                | ((double_push as u32) << DOUBLE_PUSH_SHIFT)
                | ((en_passant as u32) << EN_PASSANT_SHIFT)
                | ((castling as u32) << CASTLING_SHIFT),
        )
    }

    pub fn get_source_square(&self) -> Square {
        self.0 & SOURCE_SQUARE_MASK
    }

    pub fn get_target_square(&self) -> Square {
        (self.0 & TARGET_SQUARE_MASK) >> TARGET_SQUARE_SHIFT
    }

    pub fn get_piece(&self) -> Piece {
        Piece::try_from((self.0 & PIECE_MASK) >> PIECE_SHIFT)
            .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
    }

    pub fn get_promotion(&self) -> Option<Promotion> {
        match Promotion::try_from((self.0 & PROMOTION_MASK) >> PROMOTION_SHIFT) {
            Ok(promotion) => Some(promotion),
            Err(_) => None,
        }
    }

    pub fn get_capture(&self) -> bool {
        ((self.0 & CAPTURE_MASK) >> CAPTURE_SHIFT) != 0
    }

    pub fn get_double_push(&self) -> bool {
        ((self.0 & DOUBLE_PUSH_MASK) >> DOUBLE_PUSH_SHIFT) != 0
    }

    pub fn get_en_passant(&self) -> bool {
        ((self.0 & EN_PASSANT_MASK) >> EN_PASSANT_SHIFT) != 0
    }

    pub fn get_castling(&self) -> bool {
        ((self.0 & CASTLING_MASK) >> CASTLING_SHIFT) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn move_tests() {
        let mv = Move::new(E3, F4, Piece::Pawn, None, true, false, false, false);
        assert_eq!(mv.get_source_square(), E3);
        assert_eq!(mv.get_target_square(), F4);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), None);
        assert_eq!(mv.get_capture(), true);
        assert_eq!(mv.get_double_push(), false);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_castling(), false);
        let mv = Move::new(
            G7,
            G8,
            Piece::Pawn,
            Some(Promotion::Queen),
            false,
            false,
            false,
            false,
        );
        assert_eq!(mv.get_source_square(), G7);
        assert_eq!(mv.get_target_square(), G8);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), Some(Promotion::Queen));
        assert_eq!(mv.get_capture(), false);
        assert_eq!(mv.get_double_push(), false);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_castling(), false);
    }
}
