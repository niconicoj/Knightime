use crate::{bitboard::Bitboard, constants::*};

pub fn move_n(bitboard: Bitboard) -> Bitboard {
    bitboard << 8
}

pub fn move_s(bitboard: Bitboard) -> Bitboard {
    bitboard >> 8
}

pub fn move_e(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_H) << 1
}

pub fn move_ne(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_H) << 9
}

pub fn move_se(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_H) >> 7
}

pub fn move_w(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_A) >> 1
}

pub fn move_nw(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_A) << 7
}

pub fn move_sw(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_A) >> 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_step_moves_tests() {
        // North
        assert_eq!(move_n(Bitboard::from_square(A1)), 0x0000000000000100);
        assert_eq!(move_n(Bitboard::from_square(A8)), 0x0000000000000000);
        // South
        assert_eq!(move_s(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_s(Bitboard::from_square(A8)), 0x0001000000000000);
        // East
        assert_eq!(move_e(Bitboard::from_square(A8)), 0x0200000000000000);
        assert_eq!(move_e(Bitboard::from_square(H1)), 0x0000000000000000);
        // West
        assert_eq!(move_w(Bitboard::from_square(H1)), 0x0000000000000040);
        assert_eq!(move_w(Bitboard::from_square(A1)), 0x0000000000000000);
        // North-East
        assert_eq!(move_ne(Bitboard::from_square(A1)), 0x0000000000000200);
        assert_eq!(move_ne(Bitboard::from_square(H1)), 0x0000000000000000);
        assert_eq!(move_ne(Bitboard::from_square(A8)), 0x0000000000000000);
        assert_eq!(move_ne(Bitboard::from_square(H8)), 0x0000000000000000);
        // North-West
        assert_eq!(move_nw(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_nw(Bitboard::from_square(H1)), 0x0000000000004000);
        assert_eq!(move_nw(Bitboard::from_square(A8)), 0x0000000000000000);
        assert_eq!(move_nw(Bitboard::from_square(H8)), 0x0000000000000000);
        // South-East
        assert_eq!(move_se(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_se(Bitboard::from_square(H1)), 0x0000000000000000);
        assert_eq!(move_se(Bitboard::from_square(A8)), 0x0002000000000000);
        assert_eq!(move_se(Bitboard::from_square(H8)), 0x0000000000000000);
        // South-West
        assert_eq!(move_sw(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_sw(Bitboard::from_square(H1)), 0x0000000000000000);
        assert_eq!(move_sw(Bitboard::from_square(A8)), 0x0000000000000000);
        assert_eq!(move_sw(Bitboard::from_square(H8)), 0x0040000000000000);
    }
}
