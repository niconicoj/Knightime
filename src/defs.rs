use std::hint::unreachable_unchecked;

pub type Square = u32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    White = 0,
    Black = 1,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub enum CastleRights {
    None,
    KingSide,
    QueenSide,
    Both,
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
            CastleRights::None => "",
            CastleRights::KingSide => "k",
            CastleRights::QueenSide => "q",
            CastleRights::Both => "kq",
        };

        if side == Side::White {
            result.to_uppercase()
        } else {
            result.to_string()
        }
    }
}
