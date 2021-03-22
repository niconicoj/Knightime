pub type Square = u32;

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    White,
    Black,
}

pub enum Piece {
    Bishop,
    Rook,
}
