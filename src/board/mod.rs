mod constants;
mod defs;

use std::fmt;

use crate::{
    bitboard::Bitboard,
    constants::{SQUARE_NAME, UNICODE_PIECE},
    defs::{Piece, Side, Square},
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
        let mut bitboards = [EMPTY_POSITION, EMPTY_POSITION];

        let mut rank: u32 = 7;
        let mut file: u32 = 0;

        for char in fen_position.chars() {
            let square = rank * 8 + file;

            match char {
                '/' => {
                    file = 0;
                    rank -= 1;
                }
                '1'..='8' => match char.to_digit(10) {
                    Some(incr) => {
                        file += incr;
                    }
                    None => {
                        return Err(ParseFenError::UnexpectedChar(
                            "failed to convert char to digit",
                        ));
                    }
                },
                'K' => {
                    bitboards[Side::White as usize][Piece::King as usize].set_square(square);
                    file += 1;
                }
                'k' => {
                    bitboards[Side::Black as usize][Piece::King as usize].set_square(square);
                    file += 1;
                }
                'Q' => {
                    bitboards[Side::White as usize][Piece::Queen as usize].set_square(square);
                    file += 1;
                }
                'q' => {
                    bitboards[Side::Black as usize][Piece::Queen as usize].set_square(square);
                    file += 1;
                }
                'P' => {
                    bitboards[Side::White as usize][Piece::Pawn as usize].set_square(square);
                    file += 1;
                }
                'p' => {
                    bitboards[Side::Black as usize][Piece::Pawn as usize].set_square(square);
                    file += 1;
                }
                'B' => {
                    bitboards[Side::White as usize][Piece::Bishop as usize].set_square(square);
                    file += 1;
                }
                'b' => {
                    bitboards[Side::Black as usize][Piece::Bishop as usize].set_square(square);
                    file += 1;
                }
                'N' => {
                    bitboards[Side::White as usize][Piece::Knight as usize].set_square(square);
                    file += 1;
                }
                'n' => {
                    bitboards[Side::Black as usize][Piece::Knight as usize].set_square(square);
                    file += 1;
                }
                'R' => {
                    bitboards[Side::White as usize][Piece::Rook as usize].set_square(square);
                    file += 1;
                }
                'r' => {
                    bitboards[Side::Black as usize][Piece::Rook as usize].set_square(square);
                    file += 1;
                }
                _ => return Err(ParseFenError::UnexpectedChar("sorry")),
            }
        }

        Ok(bitboards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn parse_fen_positions_tests() {
        assert_eq!(
            Board::parse_fen_positions("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            Ok([INITIAL_WHITE_POSITIONS, INITIAL_BLACK_POSITIONS])
        );
        assert_eq!(
            Board::parse_fen_positions("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R"),
            Ok([
                [
                    Bitboard(0x0000000000000010),
                    Bitboard(0x0000000000200000),
                    Bitboard(0x000000081000e700),
                    Bitboard(0x0000001000040000),
                    Bitboard(0x0000000000001800),
                    Bitboard(0x0000000000000081),
                ],
                [
                    Bitboard(0x1000000000000000),
                    Bitboard(0x0010000000000000),
                    Bitboard(0x002d500002800000),
                    Bitboard(0x0000220000000000),
                    Bitboard(0x0040010000000000),
                    Bitboard(0x8100000000000000),
                ],
            ])
        );
    }
}
