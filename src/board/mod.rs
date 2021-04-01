mod attacks;
mod constants;
mod defs;
mod make;

use std::fmt;

use crate::{
    bitboard::Bitboard,
    constants::{SQUARE_NAME, UNICODE_PIECE},
    defs::{CastleRights, Piece, Side, Square},
    move_generator::MoveGenerator,
};

use self::{
    constants::{
        EMPTY_POSITION, INITIAL_BLACK_POSITIONS, INITIAL_OCCUPANCIES, INITIAL_WHITE_POSITIONS,
    },
    defs::ParseFenError,
};

pub use make::MakeMoveError;

#[derive(Clone, Copy)]
pub struct BoardState {
    pub bitboards: [[Bitboard; 6]; 2],
    pub occupancies: [Bitboard; 3],
    pub side_to_move: Side,
    pub en_passant_square: Option<Square>,
    pub castling_rights: [CastleRights; 2],
}

pub type BoardHistory = Vec<BoardState>;

pub struct Board {
    state: BoardState,
    history: BoardHistory,
    move_generator: MoveGenerator,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side_to_move = match self.state.side_to_move {
            Side::White => "white",
            Side::Black => "black",
        };
        let en_passant_square = match self.state.en_passant_square {
            Some(square) => SQUARE_NAME[square as usize],
            None => "none",
        };

        let castling_rights = format!(
            "{} {}",
            self.state.castling_rights[0].to_string(Side::White),
            self.state.castling_rights[1].to_string(Side::Black)
        );
        writeln!(f)?;
        for rank in (0u64..8u64).rev() {
            for file in 0u64..8u64 {
                if file == 0 {
                    print!("{}   ", rank + 1);
                }

                let mut occupied = false;
                let square = (rank * 8 + file) as Square;
                for (side, bitboards) in self.state.bitboards.iter().enumerate() {
                    for (piece, bitboard) in bitboards.iter().enumerate() {
                        if bitboard.get_square(square) {
                            occupied = true;
                            write!(f, "{} ", UNICODE_PIECE[side as usize][piece as usize])?;
                            break;
                        }
                    }
                }

                if !occupied {
                    write!(f, ". ")?;
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
            state: BoardState {
                bitboards: [INITIAL_WHITE_POSITIONS, INITIAL_BLACK_POSITIONS],
                side_to_move: Side::White,
                occupancies: INITIAL_OCCUPANCIES,
                en_passant_square: None,
                castling_rights: [CastleRights::Both; 2],
            },
            history: vec![],
            move_generator: MoveGenerator::new(),
        }
    }

    pub fn from_fen(fen_string: &str) -> Result<Self, ParseFenError> {
        // first block is in regard to piece placement, it start from rank 8 all the way to rank 1
        let fen_parts: Vec<&str> = fen_string.trim().split(' ').collect();

        match fen_parts.len() == 6 {
            false => return Err(ParseFenError::BadFenFormat("wrong number of args")),
            true => {}
        };

        let bitboards = Self::parse_fen_positions(fen_parts[0])?;
        let side_to_move = Self::parse_fen_side_to_move(fen_parts[1])?;
        let castling_rights = Self::parse_fen_castling_rights(fen_parts[2])?;
        let en_passant_square = Self::parse_fen_en_passant_square(fen_parts[3])?;

        Ok(Self {
            state: BoardState {
                bitboards: bitboards,
                occupancies: Self::compute_occupancies(bitboards),
                side_to_move: side_to_move,
                en_passant_square: en_passant_square,
                castling_rights: castling_rights,
            },
            history: vec![],
            move_generator: MoveGenerator::new(),
        })
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
                        return Err(ParseFenError::UnexpectedChar);
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
                _ => return Err(ParseFenError::UnexpectedChar),
            }
        }

        Ok(bitboards)
    }

    fn compute_occupancies(bitboards: [[Bitboard; 6]; 2]) -> [Bitboard; 3] {
        let mut occupancies = [Bitboard(0); 3];

        occupancies[0] = bitboards[0].iter().fold(Bitboard(0), |acc, &bb| acc | bb);
        occupancies[1] = bitboards[1].iter().fold(Bitboard(0), |acc, &bb| acc | bb);
        occupancies[2] = occupancies[1] | occupancies[0];

        occupancies
    }

    fn parse_fen_side_to_move(fen_str: &str) -> Result<Side, ParseFenError> {
        return match fen_str.chars().nth(0) {
            Some(char) => match char {
                'w' => Ok(Side::White),
                'b' => Ok(Side::Black),
                _ => Err(ParseFenError::UnexpectedChar),
            },
            None => Err(ParseFenError::EmptyString),
        };
    }

    fn parse_fen_castling_rights(fen_str: &str) -> Result<[CastleRights; 2], ParseFenError> {
        let mut castling_rights = [CastleRights::None; 2];
        for char in fen_str.chars() {
            match char {
                '-' => return Ok(castling_rights),
                'K' => castling_rights[0] = castling_rights[0].add(CastleRights::KingSide),
                'k' => castling_rights[1] = castling_rights[1].add(CastleRights::KingSide),
                'Q' => castling_rights[0] = castling_rights[0].add(CastleRights::QueenSide),
                'q' => castling_rights[1] = castling_rights[1].add(CastleRights::QueenSide),
                _ => return Err(ParseFenError::UnexpectedChar),
            };
        }
        Ok(castling_rights)
    }

    fn parse_fen_en_passant_square(fen_str: &str) -> Result<Option<Square>, ParseFenError> {
        let mut rank = 0;
        let mut file = 0;
        for char in fen_str.chars() {
            match char {
                '-' => return Ok(None),
                'a'..='h' => {
                    println!("{}", char as u32);
                    file = char as u32 - 97;
                }
                '3' | '6' => rank = char.to_digit(10).unwrap() - 1,
                _ => return Err(ParseFenError::UnexpectedChar),
            };
        }
        Ok(Some(rank * 8 + file))
    }

    fn store_state(&mut self) {
        self.history.push(BoardState {
            bitboards: self.state.bitboards,
            occupancies: self.state.occupancies,
            side_to_move: self.state.side_to_move,
            en_passant_square: self.state.en_passant_square,
            castling_rights: self.state.castling_rights,
        });
    }
    pub fn take_back_move(&mut self) {
        self.state = self.history.pop().unwrap_or(self.state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn parse_fen_side_to_move_tests() {
        assert_eq!(Board::parse_fen_side_to_move("w"), Ok(Side::White));
        assert_eq!(Board::parse_fen_side_to_move("b"), Ok(Side::Black));
        assert_eq!(
            Board::parse_fen_side_to_move(""),
            Err(ParseFenError::EmptyString)
        );
        assert_eq!(
            Board::parse_fen_side_to_move("g"),
            Err(ParseFenError::UnexpectedChar)
        );
    }

    #[test]
    fn parse_fen_castling_rights_tests() {
        assert_eq!(
            Board::parse_fen_castling_rights("-"),
            Ok([CastleRights::None; 2])
        );
        assert_eq!(
            Board::parse_fen_castling_rights("KQkq"),
            Ok([CastleRights::Both; 2])
        );
        assert_eq!(
            Board::parse_fen_castling_rights("Kkq"),
            Ok([CastleRights::KingSide, CastleRights::Both])
        );
    }

    #[test]
    fn parse_fen_en_passant_square_tests() {
        assert_eq!(Board::parse_fen_en_passant_square("-"), Ok(None));
        assert_eq!(Board::parse_fen_en_passant_square("a6"), Ok(Some(40)));
        assert_eq!(
            Board::parse_fen_en_passant_square("g5"),
            Err(ParseFenError::UnexpectedChar)
        );
    }

    #[test]
    fn compute_occupancies_tests() {
        assert_eq!(
            Board::compute_occupancies([INITIAL_WHITE_POSITIONS, INITIAL_BLACK_POSITIONS]),
            INITIAL_OCCUPANCIES
        );
    }
}
