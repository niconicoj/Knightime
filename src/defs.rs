pub type Square = u32;

#[derive(Copy, Clone, PartialEq)]
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

#[repr(u32)]
pub enum Castling {
    WhiteKingSide = 1,
    BlackKingSide = 2,
    WhiteQueenSide = 4,
    BlackQueenSide = 8,
}
