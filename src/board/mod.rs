use std::fmt;

use crate::{
    bitboard::Bitboard,
    constants::{
        INITIAL_BLACK_POSITIONS, INITIAL_OCCUPANCIES, INITIAL_WHITE_POSITIONS, UNICODE_PIECE,
    },
    defs::{Side, Square},
};

pub struct Board {
    bitboards: [[Bitboard; 6]; 2],
    occupancies: [Bitboard; 3],
    side_to_move: Side,
    en_passant_square: u32,
    castling_rights: u32,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for rank in (0u64..8u64).rev() {
            for file in 0u64..8u64 {
                if file == 0 {
                    print!("{}   ", rank + 1);
                }
                let square = (rank * 8 + file) as Square;
                for (side, bitboards) in self.bitboards.iter().enumerate() {
                    for (piece, bitboard) in bitboards.iter().enumerate() {
                        if bitboard.get_square(square) {
                            write!(f, "{} ", UNICODE_PIECE[side as usize][piece as usize])?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "    a b c d e f g h")?;
        writeln!(f)?;
        Ok(())
    }
}

impl Board {
    pub fn default() -> Self {
        Self {
            bitboards: [INITIAL_WHITE_POSITIONS, INITIAL_BLACK_POSITIONS],
            side_to_move: Side::White,
            occupancies: INITIAL_OCCUPANCIES,
            en_passant_square: 0,
            castling_rights: 15,
        }
    }
}
