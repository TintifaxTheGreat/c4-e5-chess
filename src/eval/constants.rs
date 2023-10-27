// Bitmaps
/// Center of the board
pub const CB_CENTER: u64 = 0x00003C3C3C3C0000;

/// Inner center of the board
pub const CB_CENTER_0: u64 = 0x00003c24243c0000;

/// Outer center of the board
pub const CB_CENTER_1: u64 = 0x0000001818000000;

/// Outer edge of the board
pub const CB_BOARD_0: u64 = 0xff818181818181ff;

/// Inner edge of the board
pub const CB_BOARD_1: u64 = 0x007e424242427e00;

/// Best squares for center pawns
pub const CB_CENTER_0_GOOD: u64 = 0x182424180000;

/// Best squares for king's safety
pub const CB_SAFE_KING: u64 = 0xc3000000000000c3;

/// Best squares for bishop
pub const CB_GOOD_BISHOP: u64 = 0x42006666004200;

/// Best squares for queen
pub const CB_GOOD_QUEEN: u64 = 0x3c1800000000183c;

/// 1st rank
pub const CB_RANK_1: u64 = 0xff;

/// 2nd rank
pub const CB_RANK_2: u64 = 0xff00;

/// 3rd rank
pub const CB_RANK_3: u64 = 0xff0000;

/// 6th rank
pub const CB_RANK_6: u64 = 0xff0000000000;

/// 7th rank
pub const CB_RANK_7: u64 = 0xff000000000000;

/// 8th rank
pub const CB_RANK_8: u64 = 0xff00000000000000;
