mod constants;
use constants::*;
mod util;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Pieces {
    WPawn,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
    Empty,
}

type BoardArray = [[Pieces; 8]; 8];

pub struct Board {
    pub white_pieces: u64,
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_kings: u64,

    pub black_pieces: u64,
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_kings: u64,

    pub pieces: u64,
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,

    pub empty_squares: u64,
}

impl Default for Board {
    fn default() -> Self {
        Board::new(
            INITIAL_WHITE_PAWNS,
            INITIAL_WHITE_KNIGHTS,
            INITIAL_WHITE_BISHOPS,
            INITIAL_WHITE_ROOKS,
            INITIAL_WHITE_QUEENS,
            INITIAL_WHITE_KINGS,
            INITIAL_BLACK_PAWNS,
            INITIAL_BLACK_KNIGHTS,
            INITIAL_BLACK_BISHOPS,
            INITIAL_BLACK_ROOKS,
            INITIAL_BLACK_QUEENS,
            INITIAL_BLACK_KINGS,
        )
    }
}

impl Board {
    pub fn new(
        white_pawns: u64,
        white_knights: u64,
        white_bishops: u64,
        white_rooks: u64,
        white_queens: u64,
        white_kings: u64,
        black_pawns: u64,
        black_knights: u64,
        black_bishops: u64,
        black_rooks: u64,
        black_queens: u64,
        black_kings: u64,
    ) -> Board {
        let white_pieces =
            white_pawns |
            white_knights |
            white_bishops |
            white_rooks |
            white_queens |
            white_kings;

        let black_pieces =
            black_pawns |
            black_knights |
            black_bishops |
            black_rooks |
            black_queens |
            black_kings;

        let pieces = white_pieces | black_pieces;
        let empty_squares = !pieces;

        let pawns = white_pawns | black_pawns;
        let knights = white_knights | black_knights;
        let bishops = white_bishops | black_bishops;
        let rooks = white_rooks | black_rooks;
        let queens = white_queens | black_queens;
        let kings = white_kings | black_kings;

        Board {
            white_pieces,
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_kings,

            black_pieces,
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_kings,

            pieces,
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            kings,

            empty_squares,
        }
    }

    pub fn from_array(board: BoardArray) -> Self {
        let mut white_pawns: u64 = 0;
        let mut white_knights: u64 = 0;
        let mut white_bishops: u64 = 0;
        let mut white_rooks: u64 = 0;
        let mut white_queens: u64 = 0;
        let mut white_kings: u64 = 0;
        let mut black_pawns: u64 = 0;
        let mut black_knights: u64 = 0;
        let mut black_bishops: u64 = 0;
        let mut black_rooks: u64 = 0;
        let mut black_queens: u64 = 0;
        let mut black_kings: u64 = 0;

        board.iter()
            .enumerate()
            .for_each(|(row, pieces)| {
                pieces.iter()
                    .enumerate()
                    .for_each(|(col, piece)| {
                        let pos = (row * 8) + col;
                        let square = SQUARE_MASK[pos];

                        match piece {
                            Pieces::WPawn => white_pawns |= square,
                            Pieces::WKnight => white_knights |= square,
                            Pieces::WBishop => white_bishops |= square,
                            Pieces::WRook => white_rooks |= square,
                            Pieces::WQueen => white_queens |= square,
                            Pieces::WKing => white_kings |= square,
                            Pieces::BPawn => black_pawns |= square,
                            Pieces::BKnight => black_knights |= square,
                            Pieces::BBishop => black_bishops |= square,
                            Pieces::BRook => black_rooks |= square,
                            Pieces::BQueen => black_queens |= square,
                            Pieces::BKing => black_kings |= square,
                            Pieces::Empty => (),
                        }
                    });
            });

        Board::new(
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_kings,
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_kings,
        )
    }

    pub fn to_array(&self) -> BoardArray {
        let mut board_array: BoardArray = [[Pieces::Empty; 8]; 8];
        for pos in 0..64 {
            let rank = pos / 8;
            let file = pos % 8;
            let square = SQUARE_MASK[pos];
            
            let piece = self.get_piece_at(square);
            board_array[rank][file] = piece;
        }

        board_array
    }

    pub fn get_piece_at(&self, square: u64) -> Pieces {
        if self.empty_squares & square > 0 {
            Pieces::Empty
        } else {
            if self.pawns & square > 0 {
                if self.black_pieces & square > 0 {
                    Pieces::BPawn
                } else {
                    Pieces::WPawn
                }
            } else if self.knights & square > 0 {
                if self.black_pieces & square > 0 {
                    Pieces::BKnight
                } else {
                    Pieces::WKnight
                }
            } else if self.bishops & square > 0 {
                if self.black_pieces & square > 0 {
                    Pieces::BBishop
                } else {
                    Pieces::WBishop
                }
            } else if self.rooks & square > 0 {
                if self.black_pieces & square > 0 {
                    Pieces::BRook
                } else {
                    Pieces::WRook
                }
            } else if self.queens & square > 0 {
                if self.black_pieces & square > 0 {
                    Pieces::BQueen
                } else {
                    Pieces::WQueen
                }
            } else {
                if self.black_pieces & square > 0 {
                    Pieces::BKing
                } else {
                    Pieces::WKing
                }
            }
        }
    }

    pub fn valid_king_moves(&self, square: u64, own_side: u64) -> u64 {
        let clip_h = square & CLEAR_H_FILE;        
        let clip_a = square & CLEAR_A_FILE;        
        let left_up = clip_a << 7;
        let up = square << 8;        
        let right_up = clip_h << 9;        
        let right = clip_h << 1;        
        let down_right = clip_h >> 7;       
        let down = square >> 8;        
        let left_down = clip_a >> 9;        
        let left = clip_a >> 1;        
        
        let moves =
            left_up |
            up |
            right_up |
            right |
            down_right |
            down |
            left_down |
            left;

        moves & !own_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::{str_to_u64};

    const INTIAL_BOARD: BoardArray = [
        [Pieces::BRook, Pieces::BKnight, Pieces::BBishop, Pieces::BQueen, Pieces::BKing, Pieces::BBishop, Pieces::BKnight, Pieces::BRook],
        [Pieces::BPawn, Pieces::BPawn, Pieces::BPawn, Pieces::BPawn, Pieces::BPawn, Pieces::BPawn, Pieces::BPawn, Pieces::BPawn],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::WPawn, Pieces::WPawn, Pieces::WPawn, Pieces::WPawn, Pieces::WPawn, Pieces::WPawn, Pieces::WPawn, Pieces::WPawn],
        [Pieces::WRook, Pieces::WKnight, Pieces::WBishop, Pieces::WQueen, Pieces::WKing, Pieces::WBishop, Pieces::WKnight, Pieces::WRook],
    ];

    const EMPTY_BOARD: BoardArray = [
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
    ];

    mod from_array {
        use super::*;

        #[test]
        fn it_works_with_initial_board() {
            let b = Board::from_array(INTIAL_BOARD);
            assert_eq!(b.white_pawns, INITIAL_WHITE_PAWNS);
            assert_eq!(b.white_knights, INITIAL_WHITE_KNIGHTS);
            assert_eq!(b.white_bishops, INITIAL_WHITE_BISHOPS);
            assert_eq!(b.white_rooks, INITIAL_WHITE_ROOKS);
            assert_eq!(b.white_queens, INITIAL_WHITE_QUEENS);
            assert_eq!(b.white_kings, INITIAL_WHITE_KINGS);
            assert_eq!(b.black_pawns, INITIAL_BLACK_PAWNS);
            assert_eq!(b.black_knights, INITIAL_BLACK_KNIGHTS);
            assert_eq!(b.black_bishops, INITIAL_BLACK_BISHOPS);
            assert_eq!(b.black_rooks, INITIAL_BLACK_ROOKS);
            assert_eq!(b.black_queens, INITIAL_BLACK_QUEENS);
            assert_eq!(b.black_kings, INITIAL_BLACK_KINGS);
        }

        #[test]
        fn it_works_with_empty_board() {
            let b = Board::from_array(EMPTY_BOARD);
            assert_eq!(b.white_pawns, 0);
            assert_eq!(b.white_knights, 0);
            assert_eq!(b.white_bishops, 0);
            assert_eq!(b.white_rooks, 0);
            assert_eq!(b.white_queens, 0);
            assert_eq!(b.white_kings, 0);
            assert_eq!(b.black_pawns, 0);
            assert_eq!(b.black_knights, 0);
            assert_eq!(b.black_bishops, 0);
            assert_eq!(b.black_rooks, 0);
            assert_eq!(b.black_queens, 0);
            assert_eq!(b.black_kings, 0);
        }

        #[test]
        fn derived_bitboards_work() {
            let b = Board::from_array(INTIAL_BOARD);

            assert_eq!(
                str_to_u64("0000000000000000111111111111111111111111111111110000000000000000"),
                b.empty_squares,
            );

            assert_eq!(
                str_to_u64("1111111111111111000000000000000000000000000000000000000000000000"),
                b.black_pieces,
            );

            assert_eq!(
                str_to_u64("0000000000000000000000000000000000000000000000001111111111111111"),
                b.white_pieces,
            );

            assert_eq!(
                str_to_u64("1111111111111111000000000000000000000000000000001111111111111111"),
                b.pieces,
            );
        }
    }

    mod get_piece_at {
        use super::*;

        #[test]
        fn it_works_with_initial_board() {
            let b = Board::from_array(INTIAL_BOARD);
            assert_eq!(
                b.get_piece_at(E2_SQUARE),
                Pieces::WPawn,
            );

            assert_eq!(
                b.get_piece_at(E7_SQUARE),
                Pieces::BPawn,
            );

            assert_eq!(
                b.get_piece_at(E8_SQUARE),
                Pieces::BKing,
            );

            assert_eq!(
                b.get_piece_at(E1_SQUARE),
                Pieces::WKing,
            );
        }
    }
    mod valid_king_moves {
        use super::*;

        #[test]
        fn it_works_with_no_obstacles() {
            let empty: u64 = 0;
            let b = Board::new(
                empty,
                empty,
                empty,
                empty,
                empty,
                E3_SQUARE,
                empty,
                empty,
                empty,
                empty,
                empty,
                empty,
            );

            let valid_squares =
                D4_SQUARE |
                E4_SQUARE |
                F4_SQUARE |
                F3_SQUARE |
                F2_SQUARE |
                E2_SQUARE |
                D2_SQUARE |
                D3_SQUARE;

            assert_eq!(b.valid_king_moves(b.white_kings, b.white_pieces), valid_squares);
        }

        #[test]
        fn it_works_with_edges() {
            let empty: u64 = 0;
            let b = Board::new(
                empty,
                empty,
                empty,
                empty,
                empty,
                A1_SQUARE,
                empty,
                empty,
                empty,
                empty,
                empty,
                empty,
            );

            let valid_squares =
                A2_SQUARE |
                B2_SQUARE |
                B1_SQUARE;

            assert_eq!(b.valid_king_moves(b.white_kings, b.white_pieces), valid_squares);
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let empty: u64 = 0;
            let b = Board::new(
                RANK_4,
                E2_SQUARE | D2_SQUARE,
                empty,
                empty,
                empty,
                E3_SQUARE,
                empty,
                empty,
                empty,
                empty,
                empty,
                empty,
            );

            let valid_squares =
                F3_SQUARE |
                F2_SQUARE |
                D3_SQUARE;

            assert_eq!(b.valid_king_moves(b.white_kings, b.white_pieces), valid_squares);
        }
    }

    mod default {
        use super::*;

        #[test]
        fn it_works() {
            let b = Board::default();
            assert_eq!(b.white_pawns, INITIAL_WHITE_PAWNS);
            assert_eq!(b.white_knights, INITIAL_WHITE_KNIGHTS);
            assert_eq!(b.white_bishops, INITIAL_WHITE_BISHOPS);
            assert_eq!(b.white_rooks, INITIAL_WHITE_ROOKS);
            assert_eq!(b.white_queens, INITIAL_WHITE_QUEENS);
            assert_eq!(b.white_kings, INITIAL_WHITE_KINGS);
            assert_eq!(b.black_pawns, INITIAL_BLACK_PAWNS);
            assert_eq!(b.black_knights, INITIAL_BLACK_KNIGHTS);
            assert_eq!(b.black_bishops, INITIAL_BLACK_BISHOPS);
            assert_eq!(b.black_rooks, INITIAL_BLACK_ROOKS);
            assert_eq!(b.black_queens, INITIAL_BLACK_QUEENS);
            assert_eq!(b.black_kings, INITIAL_BLACK_KINGS);
        }
    }
}
