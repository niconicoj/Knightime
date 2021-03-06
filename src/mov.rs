use crate::{bitboard::Bitboard, constants::*};

pub fn move_n(bitboard: Bitboard) -> Bitboard {
    bitboard << 8u32
}

pub fn move_s(bitboard: Bitboard) -> Bitboard {
    bitboard >> 8u32
}

pub fn move_e(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_H) << 1u32
}

pub fn move_ne(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_H) << 9u32
}

pub fn move_se(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_H) >> 7u32
}

pub fn move_w(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_A) >> 1u32
}

pub fn move_nw(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_A) << 7u32
}

pub fn move_sw(bitboard: Bitboard) -> Bitboard {
    (bitboard & !FILE_A) >> 9u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_north_tests() {
        assert_eq!(move_n(Bitboard::from_square(A1)), 0x0000000000000100);
        assert_eq!(move_n(Bitboard::from_square(A8)), 0x0000000000000000);
    }

    #[test]
    fn move_east_tests() {
        assert_eq!(move_e(Bitboard::from_square(A8)), 0x0200000000000000);
        assert_eq!(move_e(Bitboard::from_square(H1)), 0x0000000000000000);
    }

    #[test]
    fn move_south_tests() {
        assert_eq!(move_s(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_s(Bitboard::from_square(A8)), 0x0001000000000000);
    }

    #[test]
    fn move_west_tests() {
        assert_eq!(move_w(Bitboard::from_square(H1)), 0x0000000000000040);
        assert_eq!(move_w(Bitboard::from_square(A1)), 0x0000000000000000);
    }

    #[test]
    fn move_north_east_tests() {
        assert_eq!(move_ne(Bitboard::from_square(A1)), 0x0000000000000200);
        assert_eq!(move_ne(Bitboard::from_square(H1)), 0x0000000000000000);
        assert_eq!(move_ne(Bitboard::from_square(A8)), 0x0000000000000000);
        assert_eq!(move_ne(Bitboard::from_square(H8)), 0x0000000000000000);
    }

    #[test]
    fn move_north_west_tests() {
        assert_eq!(move_nw(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_nw(Bitboard::from_square(H1)), 0x0000000000004000);
        assert_eq!(move_nw(Bitboard::from_square(A8)), 0x0000000000000000);
        assert_eq!(move_nw(Bitboard::from_square(H8)), 0x0000000000000000);
    }

    #[test]
    fn move_south_east_tests() {
        assert_eq!(move_se(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_se(Bitboard::from_square(H1)), 0x0000000000000000);
        assert_eq!(move_se(Bitboard::from_square(A8)), 0x0002000000000000);
        assert_eq!(move_se(Bitboard::from_square(H8)), 0x0000000000000000);
    }

    #[test]
    fn move_south_west_tests() {
        assert_eq!(move_sw(Bitboard::from_square(A1)), 0x0000000000000000);
        assert_eq!(move_sw(Bitboard::from_square(H1)), 0x0000000000000000);
        assert_eq!(move_sw(Bitboard::from_square(A8)), 0x0000000000000000);
        assert_eq!(move_sw(Bitboard::from_square(H8)), 0x0040000000000000);
    }
}
