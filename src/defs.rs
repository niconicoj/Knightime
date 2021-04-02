use std::{convert::TryFrom, hint::unreachable_unchecked, ops::BitAnd};

pub type Square = u32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Into<usize> for Side {
    fn into(self) -> usize {
        self as usize
    }
}

impl Side {
    pub fn get_opposite_side(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
pub enum Sides {
    White = 0,
    Black = 1,
    Both = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
}

impl TryFrom<u32> for Piece {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Piece::King),
            1 => Ok(Piece::Queen),
            2 => Ok(Piece::Pawn),
            3 => Ok(Piece::Knight),
            4 => Ok(Piece::Bishop),
            5 => Ok(Piece::Rook),
            _ => Err("fail to decode piece from u32"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum Promotion {
    Queen = 1,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
}

impl TryFrom<u32> for Promotion {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Promotion::Queen),
            3 => Ok(Promotion::Knight),
            4 => Ok(Promotion::Bishop),
            5 => Ok(Promotion::Rook),
            _ => Err("fail to decode promotion from u32"),
        }
    }
}

impl TryFrom<char> for Promotion {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'q' => Ok(Promotion::Queen),
            'n' => Ok(Promotion::Knight),
            'b' => Ok(Promotion::Bishop),
            'r' => Ok(Promotion::Rook),
            _ => Err("fail to decode promotion from u32"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
#[repr(u32)]
pub enum CastleRights {
    None,
    KingSide,
    QueenSide,
    Both,
}

impl BitAnd<usize> for CastleRights {
    type Output = Self;

    fn bitand(self, rhs: usize) -> Self::Output {
        CastleRights::from_index(self.to_index() & rhs)
    }
}

impl CastleRights {
    pub fn add(&self, add: CastleRights) -> CastleRights {
        CastleRights::from_index(self.to_index() | add.to_index())
    }

    pub fn from_index(i: usize) -> CastleRights {
        match i {
            0 => CastleRights::None,
            1 => CastleRights::KingSide,
            2 => CastleRights::QueenSide,
            3 => CastleRights::Both,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn to_string(&self, side: Side) -> String {
        let result = match *self {
            CastleRights::None => "- -",
            CastleRights::KingSide => "k -",
            CastleRights::QueenSide => "- q",
            CastleRights::Both => "k q",
        };

        if side == Side::White {
            result.to_uppercase()
        } else {
            result.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn castle_bitwise_and_tests() {
        let castle_rights = CastleRights::Both;

        assert_eq!(castle_rights & 0, CastleRights::None);
        assert_eq!(castle_rights & 1, CastleRights::KingSide);
        assert_eq!(castle_rights & 2, CastleRights::QueenSide);
        assert_eq!(castle_rights & 3, CastleRights::Both);

        let castle_rights = CastleRights::KingSide;

        assert_eq!(castle_rights & 0, CastleRights::None);
        assert_eq!(castle_rights & 1, CastleRights::KingSide);
        assert_eq!(castle_rights & 2, CastleRights::None);
        assert_eq!(castle_rights & 3, CastleRights::KingSide);

        let castle_rights = CastleRights::QueenSide;

        assert_eq!(castle_rights & 0, CastleRights::None);
        assert_eq!(castle_rights & 1, CastleRights::None);
        assert_eq!(castle_rights & 2, CastleRights::QueenSide);
        assert_eq!(castle_rights & 3, CastleRights::QueenSide);
    }
}
