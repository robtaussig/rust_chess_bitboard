use crate::bitboard::{BitBoard};

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const PAWNS_BB: usize = 0;
pub const KNIGHTS_BB: usize = 1;
pub const BISHOPS_BB: usize = 2;
pub const ROOKS_BB: usize = 3;
pub const QUEENS_BB: usize = 4;
pub const KINGS_BB: usize = 5;

pub const ALL_PAWNS_BB: usize = 0;
pub const ALL_KNIGHTS_BB: usize = 1;
pub const ALL_BISHOPS_BB: usize = 2;
pub const ALL_ROOKS_BB: usize = 3;
pub const ALL_QUEENS_BB: usize = 4;
pub const ALL_KINGS_BB: usize = 5;
pub const ALL_PIECES_BB: usize = 6;
pub const EMPTY_SQUARES_BB: usize = 7;

pub const INITIAL_WHITE_PAWNS: BitBoard = BitBoard(65280);
pub const INITIAL_WHITE_KNIGHTS: BitBoard = BitBoard(66);
pub const INITIAL_WHITE_BISHOPS: BitBoard = BitBoard(36);
pub const INITIAL_WHITE_ROOKS: BitBoard = BitBoard(129);
pub const INITIAL_WHITE_QUEENS: BitBoard = BitBoard(8);
pub const INITIAL_WHITE_KINGS: BitBoard = BitBoard(16);
pub const INITIAL_BLACK_PAWNS: BitBoard = BitBoard(71776119061217280);
pub const INITIAL_BLACK_KNIGHTS: BitBoard = BitBoard(4755801206503243776);
pub const INITIAL_BLACK_BISHOPS: BitBoard = BitBoard(2594073385365405696);
pub const INITIAL_BLACK_ROOKS: BitBoard = BitBoard(9295429630892703744);
pub const INITIAL_BLACK_QUEENS: BitBoard = BitBoard(576460752303423488);
pub const INITIAL_BLACK_KINGS: BitBoard = BitBoard(1152921504606846976);
pub const INITIAL_CASTLE_RIGHTS: BitBoard = BitBoard(9223372036854775808);
