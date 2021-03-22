use crate::bitboard::Bitboard;

pub const INITIAL_WHITE_POSITIONS: [Bitboard; 6] = [
    Bitboard(0x0000000000000010),
    Bitboard(0x0000000000000008),
    Bitboard(0x000000000000FF00),
    Bitboard(0x0000000000000042),
    Bitboard(0x0000000000000024),
    Bitboard(0x0000000000000081),
];

pub const INITIAL_BLACK_POSITIONS: [Bitboard; 6] = [
    Bitboard(0x1000000000000000),
    Bitboard(0x0800000000000000),
    Bitboard(0x00FF000000000000),
    Bitboard(0x4200000000000000),
    Bitboard(0x2400000000000000),
    Bitboard(0x8100000000000000),
];

pub const INITIAL_OCCUPANCIES: [Bitboard; 3] = [
    Bitboard(0x000000000000FFFF),
    Bitboard(0xFFFF000000000000),
    Bitboard(0xFFFF00000000FFFF),
];

pub const EMPTY_POSITION: [Bitboard; 6] = [
    Bitboard(0x0000000000000000),
    Bitboard(0x0000000000000000),
    Bitboard(0x0000000000000000),
    Bitboard(0x0000000000000000),
    Bitboard(0x0000000000000000),
    Bitboard(0x0000000000000000),
];

pub const EMPTY_OCCUPANCIES: [Bitboard; 3] = [
    Bitboard(0x0000000000000000),
    Bitboard(0x0000000000000000),
    Bitboard(0x0000000000000000),
];
