pub enum Player {
    White,
    Black,
}

pub const A1: u8 = 0;
pub const A2: u8 = 8;
pub const A3: u8 = 16;
pub const A4: u8 = 24;
pub const A5: u8 = 32;
pub const A6: u8 = 40;
pub const A7: u8 = 48;
pub const A8: u8 = 56;

pub const B1: u8 = 1;
pub const B2: u8 = 9;
pub const B3: u8 = 17;
pub const B4: u8 = 25;
pub const B5: u8 = 33;
pub const B6: u8 = 41;
pub const B7: u8 = 49;
pub const B8: u8 = 57;

pub const C1: u8 = 2;
pub const C2: u8 = 10;
pub const C3: u8 = 18;
pub const C4: u8 = 26;
pub const C5: u8 = 34;
pub const C6: u8 = 42;
pub const C7: u8 = 50;
pub const C8: u8 = 58;

pub const D1: u8 = 3;
pub const D2: u8 = 11;
pub const D3: u8 = 19;
pub const D4: u8 = 27;
pub const D5: u8 = 35;
pub const D6: u8 = 43;
pub const D7: u8 = 51;
pub const D8: u8 = 59;

pub const E1: u8 = 4;
pub const E2: u8 = 12;
pub const E3: u8 = 20;
pub const E4: u8 = 28;
pub const E5: u8 = 36;
pub const E6: u8 = 44;
pub const E7: u8 = 52;
pub const E8: u8 = 60;

pub const F1: u8 = 5;
pub const F2: u8 = 13;
pub const F3: u8 = 21;
pub const F4: u8 = 29;
pub const F5: u8 = 37;
pub const F6: u8 = 45;
pub const F7: u8 = 53;
pub const F8: u8 = 61;

pub const G1: u8 = 6;
pub const G2: u8 = 14;
pub const G3: u8 = 22;
pub const G4: u8 = 30;
pub const G5: u8 = 38;
pub const G6: u8 = 46;
pub const G7: u8 = 54;
pub const G8: u8 = 62;

pub const H1: u8 = 7;
pub const H2: u8 = 15;
pub const H3: u8 = 23;
pub const H4: u8 = 31;
pub const H5: u8 = 39;
pub const H6: u8 = 47;
pub const H7: u8 = 55;
pub const H8: u8 = 63;

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
