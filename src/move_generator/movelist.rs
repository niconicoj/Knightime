use std::{convert::TryFrom, hint::unreachable_unchecked};

use crate::defs::{Piece, Promotion, Square};
use crate::move_generator::defs::*;

pub struct Move(u32);

impl Move {
    pub fn new(
        source_square: Square,
        target_square: Square,
        piece: Piece,
        promotion: Promotion,
        capture: bool,
        double_push: bool,
        en_passant: bool,
        castling: bool,
    ) -> Self {
        Self(
            source_square
                | (target_square << TARGET_SQUARE_SHIFT)
                | ((piece as u32) << PIECE_SHIFT)
                | ((promotion as u32) << PROMOTION_SHIFT)
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

    pub fn get_promotion(&self) -> Promotion {
        Promotion::try_from((self.0 & PROMOTION_MASK) >> PROMOTION_SHIFT)
            .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
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

    pub fn get_caslting(&self) -> bool {
        ((self.0 & CASTLING_MASK) >> CASTLING_SHIFT) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn move_tests() {
        let mv = Move::new(
            E3,
            F4,
            Piece::Pawn,
            Promotion::None,
            true,
            false,
            false,
            false,
        );
        assert_eq!(mv.get_source_square(), E3);
        assert_eq!(mv.get_target_square(), F4);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), Promotion::None);
        assert_eq!(mv.get_capture(), true);
        assert_eq!(mv.get_double_push(), false);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_caslting(), false);
        let mv = Move::new(
            G7,
            G8,
            Piece::Pawn,
            Promotion::Queen,
            false,
            false,
            false,
            false,
        );
        assert_eq!(mv.get_source_square(), G7);
        assert_eq!(mv.get_target_square(), G8);
        assert_eq!(mv.get_piece(), Piece::Pawn);
        assert_eq!(mv.get_promotion(), Promotion::Queen);
        assert_eq!(mv.get_capture(), false);
        assert_eq!(mv.get_double_push(), false);
        assert_eq!(mv.get_en_passant(), false);
        assert_eq!(mv.get_caslting(), false);
    }
}
