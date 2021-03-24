use crate::defs::Square;

pub const A1: Square = 0;
pub const A2: Square = 8;
pub const A3: Square = 16;
pub const A4: Square = 24;
pub const A5: Square = 32;
pub const A6: Square = 40;
pub const A7: Square = 48;
pub const A8: Square = 56;

pub const B1: Square = 1;
pub const B2: Square = 9;
pub const B3: Square = 17;
pub const B4: Square = 25;
pub const B5: Square = 33;
pub const B6: Square = 41;
pub const B7: Square = 49;
pub const B8: Square = 57;

pub const C1: Square = 2;
pub const C2: Square = 10;
pub const C3: Square = 18;
pub const C4: Square = 26;
pub const C5: Square = 34;
pub const C6: Square = 42;
pub const C7: Square = 50;
pub const C8: Square = 58;

pub const D1: Square = 3;
pub const D2: Square = 11;
pub const D3: Square = 19;
pub const D4: Square = 27;
pub const D5: Square = 35;
pub const D6: Square = 43;
pub const D7: Square = 51;
pub const D8: Square = 59;

pub const E1: Square = 4;
pub const E2: Square = 12;
pub const E3: Square = 20;
pub const E4: Square = 28;
pub const E5: Square = 36;
pub const E6: Square = 44;
pub const E7: Square = 52;
pub const E8: Square = 60;

pub const F1: Square = 5;
pub const F2: Square = 13;
pub const F3: Square = 21;
pub const F4: Square = 29;
pub const F5: Square = 37;
pub const F6: Square = 45;
pub const F7: Square = 53;
pub const F8: Square = 61;

pub const G1: Square = 6;
pub const G2: Square = 14;
pub const G3: Square = 22;
pub const G4: Square = 30;
pub const G5: Square = 38;
pub const G6: Square = 46;
pub const G7: Square = 54;
pub const G8: Square = 62;

pub const H1: Square = 7;
pub const H2: Square = 15;
pub const H3: Square = 23;
pub const H4: Square = 31;
pub const H5: Square = 39;
pub const H6: Square = 47;
pub const H7: Square = 55;
pub const H8: Square = 63;

pub const SQUARE_NAME: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

pub const UNICODE_PIECE: [[&str; 6]; 2] = [
    ["♔", "♕", "♙", "♘", "♗", "♖"],
    ["♚", "♛", "♟︎", "♞", "♝", "♜"],
];

pub const ASCII_PIECE: [[&str; 6]; 2] = [
    ["K", "Q", "P", "N", "B", "R"],
    ["k", "q", "p", "n", "b", "r"],
];

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_AB: u64 = 0x0303030303030303;
pub const FILE_H: u64 = 0x8080808080808080;
pub const FILE_HG: u64 = 0xc0c0c0c0c0c0c0c0;

pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_8: u64 = 0xFF00000000000000;
pub const RANK_12: u64 = 0x000000000000FFFF;
pub const RANK_78: u64 = 0xFFFF000000000000;

pub const DIAGONAL_AH: u64 = 0x8040201008040201;
pub const DIAGONAL_HA: u64 = 0x0102040810204080;
