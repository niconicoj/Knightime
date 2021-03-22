mod constants;
mod defs;

use std::fmt;

use crate::{
    bitboard::Bitboard,
    constants::{SQUARE_NAME, UNICODE_PIECE},
    defs::{Side, Square},
};

use self::{
    constants::{
        EMPTY_OCCUPANCIES, EMPTY_POSITION, INITIAL_BLACK_POSITIONS, INITIAL_OCCUPANCIES,
        INITIAL_WHITE_POSITIONS,
    },
    defs::ParseFenError,
};

pub struct Board {
    bitboards: [[Bitboard; 6]; 2],
    occupancies: [Bitboard; 3],
    side_to_move: Side,
    en_passant_square: Option<Square>,
    castling_rights: u32,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side_to_move = match self.side_to_move {
            Side::White => "white",
            Side::Black => "black",
        };
        let en_passant_square = match self.en_passant_square {
            Some(square) => SQUARE_NAME[square as usize],
            None => "none",
        };

        let castling_rights = format!("{:04b}", self.castling_rights);
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
                            break;
                        }
                    }
                }
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "    a b c d e f g h")?;
        writeln!(f)?;
        writeln!(f, "side to move : {}", side_to_move)?;
        writeln!(f, "en passant square : {}", en_passant_square)?;
        writeln!(f, "castling rights : {}", castling_rights)?;
        Ok(())
    }
}

impl Board {
    pub fn default() -> Self {
        Self {
            bitboards: [INITIAL_WHITE_POSITIONS, INITIAL_BLACK_POSITIONS],
            side_to_move: Side::White,
            occupancies: INITIAL_OCCUPANCIES,
            en_passant_square: None,
            castling_rights: 15,
        }
    }

    pub fn from_fen(fen_string: &str) -> Result<Self, ParseFenError> {
        // first block is in regard to piece placement, it start from rank 8 all the way to rank 1
        let fen_parts: Vec<&str> = fen_string.split(' ').collect();

        assert_eq!(fen_parts.len(), 6, "bad fen string format");

        let bitboards = Self::parse_fen_positions(fen_parts[0])?;

        Err(ParseFenError::UnexpectedChar("sorry"))
    }

    fn parse_fen_positions(fen_position: &str) -> Result<[[Bitboard; 6]; 2], ParseFenError> {
        Err(ParseFenError::UnexpectedChar("sorry"))
    }
}
