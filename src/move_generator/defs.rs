pub const SOURCE_SQUARE_MASK: u32 = 0x3f;
pub const TARGET_SQUARE_MASK: u32 = 0xfc0;
pub const PIECE_MASK: u32 = 0xf000;
pub const PROMOTION_MASK: u32 = 0xf0000;
pub const CAPTURE_MASK: u32 = 0x100000;
pub const DOUBLE_PUSH_MASK: u32 = 0x200000;
pub const EN_PASSANT_MASK: u32 = 0x400000;
pub const CASTLING_MASK: u32 = 0x800000;

pub const TARGET_SQUARE_SHIFT: u32 = 6;
pub const PIECE_SHIFT: u32 = 12;
pub const PROMOTION_SHIFT: u32 = 16;
pub const CAPTURE_SHIFT: u32 = 20;
pub const DOUBLE_PUSH_SHIFT: u32 = 21;
pub const EN_PASSANT_SHIFT: u32 = 22;
pub const CASTLING_SHIFT: u32 = 23;
