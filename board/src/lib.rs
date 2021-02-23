mod constants;
use constants::*;
mod bitboard;
use crate::bitboard::{BitBoard, EMPTY};
mod piece;
use piece::Pieces;

type BoardArray = [[Pieces; 8]; 8];
type BitBoardTables = [BitBoard; NUM_BITBOARDS];

pub struct Board {
    pub bbs: BitBoardTables,
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
        white_pawns: BitBoard,
        white_knights: BitBoard,
        white_bishops: BitBoard,
        white_rooks: BitBoard,
        white_queens: BitBoard,
        white_kings: BitBoard,
        black_pawns: BitBoard,
        black_knights: BitBoard,
        black_bishops: BitBoard,
        black_rooks: BitBoard,
        black_queens: BitBoard,
        black_kings: BitBoard,
    ) -> Board {
        let mut bbs: BitBoardTables = [EMPTY; NUM_BITBOARDS];

        bbs[WHITE_PAWNS_BB] = white_pawns;
        bbs[WHITE_KNIGHTS_BB] = white_knights;
        bbs[WHITE_BISHOPS_BB] = white_bishops;
        bbs[WHITE_ROOKS_BB] = white_rooks;
        bbs[WHITE_QUEENS_BB] = white_queens;
        bbs[WHITE_KINGS_BB] = white_kings;
        bbs[BLACK_PAWNS_BB] = black_pawns;
        bbs[BLACK_KNIGHTS_BB] = black_knights;
        bbs[BLACK_BISHOPS_BB] = black_bishops;
        bbs[BLACK_ROOKS_BB] = black_rooks;
        bbs[BLACK_QUEENS_BB] = black_queens;
        bbs[BLACK_KINGS_BB] = black_kings;

        let white_pieces =
            white_pawns | white_knights | white_bishops | white_rooks | white_queens | white_kings;

        let black_pieces =
            black_pawns | black_knights | black_bishops | black_rooks | black_queens | black_kings;

        let pieces = white_pieces | black_pieces;
        let empty_squares = !pieces;

        let pawns = white_pawns | black_pawns;
        let knights = white_knights | black_knights;
        let bishops = white_bishops | black_bishops;
        let rooks = white_rooks | black_rooks;
        let queens = white_queens | black_queens;
        let kings = white_kings | black_kings;

        bbs[ALL_PAWNS_BB] = pawns;
        bbs[ALL_KNIGHTS_BB] = knights;
        bbs[ALL_BISHOPS_BB] = bishops;
        bbs[ALL_ROOKS_BB] = rooks;
        bbs[ALL_QUEENS_BB] = queens;
        bbs[ALL_KINGS_BB] = kings;
        bbs[WHITE_PIECES_BB] = white_pieces;
        bbs[BLACK_PIECES_BB] = black_pieces;
        bbs[ALL_PIECES_BB] = pieces;
        bbs[EMPTY_SQUARES_BB] = empty_squares;

        Board {
            bbs,
        }
    }

    pub fn from_array(board: BoardArray) -> Self {
        let mut white_pawns = EMPTY;
        let mut white_knights = EMPTY;
        let mut white_bishops = EMPTY;
        let mut white_rooks = EMPTY;
        let mut white_queens = EMPTY;
        let mut white_kings = EMPTY;
        let mut black_pawns = EMPTY;
        let mut black_knights = EMPTY;
        let mut black_bishops = EMPTY;
        let mut black_rooks = EMPTY;
        let mut black_queens = EMPTY;
        let mut black_kings = EMPTY;

        board.iter().enumerate().for_each(|(row, pieces)| {
            pieces.iter().enumerate().for_each(|(col, piece)| {
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

    pub fn get_piece_at(&self, square: BitBoard) -> Pieces {
        if self.bbs[EMPTY_SQUARES_BB] & square != EMPTY {
            Pieces::Empty
        } else {
            if self.bbs[ALL_PAWNS_BB] & square != EMPTY {
                if self.bbs[BLACK_PIECES_BB] & square != EMPTY {
                    Pieces::BPawn
                } else {
                    Pieces::WPawn
                }
            } else if self.bbs[ALL_KNIGHTS_BB] & square != EMPTY {
                if self.bbs[BLACK_PIECES_BB] & square != EMPTY {
                    Pieces::BKnight
                } else {
                    Pieces::WKnight
                }
            } else if self.bbs[ALL_BISHOPS_BB] & square != EMPTY {
                if self.bbs[BLACK_PIECES_BB] & square != EMPTY {
                    Pieces::BBishop
                } else {
                    Pieces::WBishop
                }
            } else if self.bbs[ALL_ROOKS_BB] & square != EMPTY {
                if self.bbs[BLACK_PIECES_BB] & square != EMPTY {
                    Pieces::BRook
                } else {
                    Pieces::WRook
                }
            } else if self.bbs[ALL_QUEENS_BB] & square != EMPTY {
                if self.bbs[BLACK_PIECES_BB] & square != EMPTY {
                    Pieces::BQueen
                } else {
                    Pieces::WQueen
                }
            } else {
                if self.bbs[BLACK_PIECES_BB] & square != EMPTY {
                    Pieces::BKing
                } else {
                    Pieces::WKing
                }
            }
        }
    }

    pub fn valid_king_moves(&self, squares: BitBoard, own_side: BitBoard) -> BitBoard {
        let clip_h = squares & CLEAR_H_FILE;
        let clip_a = squares & CLEAR_A_FILE;
        let left_up = clip_a.shl(7);
        let up = squares.shl(8);
        let right_up = clip_h.shl(9);
        let right = clip_h.shl(1);
        let down_right = clip_h.shr(7);
        let down = squares.shr(8);
        let left_down = clip_a.shr(9);
        let left = clip_a.shr(1);

        let moves = left_up | up | right_up | right | down_right | down | left_down | left;

        moves & !own_side
    }

    pub fn valid_knight_moves(&self, squares: BitBoard, own_side: BitBoard) -> BitBoard {
        let left_up_clip = CLEAR_A_FILE & CLEAR_B_FILE;
        let up_left_clip = CLEAR_A_FILE;

        let up_right_clip = CLEAR_H_FILE;
        let right_up_clip = CLEAR_H_FILE & CLEAR_G_FILE;

        let right_down_clip = CLEAR_H_FILE & CLEAR_G_FILE;
        let down_right_clip = CLEAR_H_FILE;

        let down_left_clip = CLEAR_A_FILE;
        let left_down_clip = CLEAR_A_FILE & CLEAR_B_FILE;

        let left_up = (squares & left_up_clip).shl(6);
        let up_left = (squares & up_left_clip).shl(15);
        let up_right = (squares & up_right_clip).shl(17);
        let right_up = (squares & right_up_clip).shl(10);
        let right_down = (squares & right_down_clip).shr(6);
        let down_right = (squares & down_right_clip).shr(15);
        let down_left = (squares & down_left_clip).shr(17);
        let left_down = (squares & left_down_clip).shr(10);

        let moves = left_up
            | up_left
            | up_right
            | right_up
            | right_down
            | down_right
            | down_left
            | left_down;

        moves & !own_side
    }

    pub fn valid_white_pawn_moves(&self, squares: BitBoard) -> BitBoard {
        let one_step = (squares.shl(8)) & self.bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_3).shl(8)) & self.bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;

        let left_attack = (squares & CLEAR_A_FILE).shl(7);
        let right_attack = (squares & CLEAR_H_FILE).shl(9);
        let attacks = left_attack | right_attack;
        let valid_attacks = attacks & self.bbs[BLACK_PIECES_BB];

        valid_steps | valid_attacks
    }

    pub fn valid_black_pawn_moves(&self, squares: BitBoard) -> BitBoard {
        let one_step = (squares.shr(8)) & self.bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_6).shr(8)) & self.bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;

        let left_attack = (squares & CLEAR_A_FILE).shr(9);
        let right_attack = (squares & CLEAR_H_FILE).shr(7);
        let attacks = left_attack | right_attack;
        let valid_attacks = attacks & self.bbs[WHITE_PIECES_BB];

        valid_steps | valid_attacks
    }

    pub fn valid_rook_moves(&self, squares: BitBoard, own_pieces: BitBoard) -> BitBoard {
        self.south_attacks(squares, own_pieces)
            | self.north_attacks(squares, own_pieces)
            | self.east_attacks(squares, own_pieces)
            | self.west_attacks(squares, own_pieces)
    }

    pub fn valid_bishop_moves(&self, squares: BitBoard, own_pieces: BitBoard) -> BitBoard {
        self.south_east_attacks(squares, own_pieces)
            | self.north_east_attacks(squares, own_pieces)
            | self.north_west_attacks(squares, own_pieces)
            | self.south_west_attacks(squares, own_pieces)
    }

    pub fn valid_queen_moves(&self, squares: BitBoard, own_pieces: BitBoard) -> BitBoard {
        self.valid_rook_moves(squares, own_pieces)
            | self.valid_bishop_moves(squares, own_pieces)
    }

    pub fn south_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        attacks |= self.bbs[EMPTY_SQUARES_BB] & (attacks.shr(8));
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & (self.bbs[EMPTY_SQUARES_BB].shr(8));
        attacks |= empty & (attacks.shr(16));
        empty &= empty.shr(16);
        attacks |= empty & (attacks.shr(32));
        (attacks.shr(8)) & !own_pieces
    }

    pub fn north_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        attacks |= self.bbs[EMPTY_SQUARES_BB] & (attacks.shl(8));
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & (self.bbs[EMPTY_SQUARES_BB].shl(8));
        attacks |= empty & (attacks.shl(16));
        empty &= empty.shl(16);
        attacks |= empty & (attacks.shl(32));
        (attacks.shl(8)) & !own_pieces
    }

    pub fn west_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & CLEAR_H_FILE;
        attacks |= empty & (attacks.shr(1));
        empty &= empty.shr(1);
        attacks |= empty & (attacks.shr(2));
        empty &= empty.shr(2);
        attacks |= empty & (attacks.shr(4));
        attacks & !own_pieces
    }

    pub fn east_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & CLEAR_A_FILE;
        attacks |= empty & (attacks.shl(1));
        empty &= empty.shl(1);
        attacks |= empty & (attacks.shl(2));
        empty &= empty.shl(2);
        attacks |= empty & (attacks.shl(4));
        attacks & !own_pieces
    }

    pub fn north_east_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & CLEAR_A_FILE;
        attacks |= empty & (attacks.shl(9));
        empty &= empty.shl(9);
        attacks |= empty & (attacks.shl(18));
        empty &= empty.shl(18);
        attacks |= empty & (attacks.shl(36));
        attacks & !own_pieces
    }

    pub fn south_east_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & CLEAR_A_FILE;
        attacks |= empty & (attacks.shr(7));
        empty &= empty.shr(7);
        attacks |= empty & (attacks.shr(14));
        empty &= empty.shr(14);
        attacks |= empty & (attacks.shr(28));
        attacks & !own_pieces
    }

    pub fn south_west_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & CLEAR_H_FILE;
        attacks |= empty & (attacks.shr(9));
        empty &= empty.shr(9);
        attacks |= empty & (attacks.shr(18));
        empty &= empty.shr(18);
        attacks |= empty & (attacks.shr(36));
        attacks & !own_pieces
    }

    pub fn north_west_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = self.bbs[EMPTY_SQUARES_BB] & CLEAR_H_FILE;
        attacks |= empty & (attacks.shl(7));
        empty &= empty.shl(7);
        attacks |= empty & (attacks.shl(14));
        empty &= empty.shl(14);
        attacks |= empty & (attacks.shl(28));
        attacks & !own_pieces
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    const INTIAL_BOARD: BoardArray = [
        [
            Pieces::BRook,
            Pieces::BKnight,
            Pieces::BBishop,
            Pieces::BQueen,
            Pieces::BKing,
            Pieces::BBishop,
            Pieces::BKnight,
            Pieces::BRook,
        ],
        [
            Pieces::BPawn,
            Pieces::BPawn,
            Pieces::BPawn,
            Pieces::BPawn,
            Pieces::BPawn,
            Pieces::BPawn,
            Pieces::BPawn,
            Pieces::BPawn,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::WPawn,
            Pieces::WPawn,
            Pieces::WPawn,
            Pieces::WPawn,
            Pieces::WPawn,
            Pieces::WPawn,
            Pieces::WPawn,
            Pieces::WPawn,
        ],
        [
            Pieces::WRook,
            Pieces::WKnight,
            Pieces::WBishop,
            Pieces::WQueen,
            Pieces::WKing,
            Pieces::WBishop,
            Pieces::WKnight,
            Pieces::WRook,
        ],
    ];

    const EMPTY_BOARD: BoardArray = [
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
    ];

    mod from_array {
        use super::*;

        #[test]
        fn it_works_with_initial_board() {
            let b = Board::from_array(INTIAL_BOARD);
            assert_eq!(b.bbs[WHITE_PAWNS_BB], INITIAL_WHITE_PAWNS);
            assert_eq!(b.bbs[WHITE_KNIGHTS_BB], INITIAL_WHITE_KNIGHTS);
            assert_eq!(b.bbs[WHITE_BISHOPS_BB], INITIAL_WHITE_BISHOPS);
            assert_eq!(b.bbs[WHITE_ROOKS_BB], INITIAL_WHITE_ROOKS);
            assert_eq!(b.bbs[WHITE_QUEENS_BB], INITIAL_WHITE_QUEENS);
            assert_eq!(b.bbs[WHITE_KINGS_BB], INITIAL_WHITE_KINGS);
            assert_eq!(b.bbs[BLACK_PAWNS_BB], INITIAL_BLACK_PAWNS);
            assert_eq!(b.bbs[BLACK_KNIGHTS_BB], INITIAL_BLACK_KNIGHTS);
            assert_eq!(b.bbs[BLACK_BISHOPS_BB], INITIAL_BLACK_BISHOPS);
            assert_eq!(b.bbs[BLACK_ROOKS_BB], INITIAL_BLACK_ROOKS);
            assert_eq!(b.bbs[BLACK_QUEENS_BB], INITIAL_BLACK_QUEENS);
            assert_eq!(b.bbs[BLACK_KINGS_BB], INITIAL_BLACK_KINGS);
        }

        #[test]
        fn it_works_with_empty_board() {
            let b = Board::from_array(EMPTY_BOARD);
            assert_eq!(b.bbs[WHITE_PAWNS_BB], EMPTY);
            assert_eq!(b.bbs[WHITE_KNIGHTS_BB], EMPTY);
            assert_eq!(b.bbs[WHITE_BISHOPS_BB], EMPTY);
            assert_eq!(b.bbs[WHITE_ROOKS_BB], EMPTY);
            assert_eq!(b.bbs[WHITE_QUEENS_BB], EMPTY);
            assert_eq!(b.bbs[WHITE_KINGS_BB], EMPTY);
            assert_eq!(b.bbs[BLACK_PAWNS_BB], EMPTY);
            assert_eq!(b.bbs[BLACK_KNIGHTS_BB], EMPTY);
            assert_eq!(b.bbs[BLACK_BISHOPS_BB], EMPTY);
            assert_eq!(b.bbs[BLACK_ROOKS_BB], EMPTY);
            assert_eq!(b.bbs[BLACK_QUEENS_BB], EMPTY);
            assert_eq!(b.bbs[BLACK_KINGS_BB], EMPTY);
        }

        #[test]
        fn derived_bitboards_work() {
            let b = Board::from_array(INTIAL_BOARD);

            assert_eq!(
                BitBoard::from_str("0000000000000000111111111111111111111111111111110000000000000000"),
                b.bbs[EMPTY_SQUARES_BB],
            );

            assert_eq!(
                BitBoard::from_str("1111111111111111000000000000000000000000000000000000000000000000"),
                b.bbs[BLACK_PIECES_BB],
            );

            assert_eq!(
                BitBoard::from_str("0000000000000000000000000000000000000000000000001111111111111111"),
                b.bbs[WHITE_PIECES_BB],
            );

            assert_eq!(
                BitBoard::from_str("1111111111111111000000000000000000000000000000001111111111111111"),
                b.bbs[ALL_PIECES_BB],
            );
        }
    }

    mod get_piece_at {
        use super::*;

        #[test]
        fn it_works_with_initial_board() {
            let b = Board::from_array(INTIAL_BOARD);
            assert_eq!(b.get_piece_at(E2_SQUARE), Pieces::WPawn,);

            assert_eq!(b.get_piece_at(E7_SQUARE), Pieces::BPawn,);

            assert_eq!(b.get_piece_at(E8_SQUARE), Pieces::BKing,);

            assert_eq!(b.get_piece_at(E1_SQUARE), Pieces::WKing,);
        }
    }
    mod valid_king_moves {
        use super::*;

        #[test]
        fn it_works_with_no_obstacles() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E3_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares = D4_SQUARE
                | E4_SQUARE
                | F4_SQUARE
                | F3_SQUARE
                | F2_SQUARE
                | E2_SQUARE
                | D2_SQUARE
                | D3_SQUARE;

            assert_eq!(
                b.valid_king_moves(b.bbs[WHITE_KINGS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_edges() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, A1_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares = A2_SQUARE | B2_SQUARE | B1_SQUARE;

            assert_eq!(
                b.valid_king_moves(b.bbs[WHITE_KINGS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                RANK_4,
                E2_SQUARE | D2_SQUARE,
                EMPTY,
                EMPTY,
                EMPTY,
                E3_SQUARE,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
            );

            let valid_squares = F3_SQUARE | F2_SQUARE | D3_SQUARE;

            assert_eq!(
                b.valid_king_moves(b.bbs[WHITE_KINGS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_castling() {
            //TODO
            assert!(false);
        }

        #[test]
        fn it_checks_for_check() {
            //TODO
            assert!(false);
        }
    }

    mod valid_knight_moves {
        use super::*;

        #[test]
        fn it_works_with_no_obstacles() {
            let b = Board::new(
                EMPTY, D4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares = B5_SQUARE
                | C6_SQUARE
                | E6_SQUARE
                | F5_SQUARE
                | F3_SQUARE
                | E2_SQUARE
                | C2_SQUARE
                | B3_SQUARE;

            assert_eq!(
                b.valid_knight_moves(b.bbs[WHITE_KNIGHTS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_edges() {
            let b = Board::new(
                EMPTY, B4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares =
                A6_SQUARE | C6_SQUARE | D5_SQUARE | D3_SQUARE | C2_SQUARE | A2_SQUARE;

            assert_eq!(
                b.valid_knight_moves(b.bbs[WHITE_KNIGHTS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                C_FILE, B4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares = A6_SQUARE | D5_SQUARE | D3_SQUARE | A2_SQUARE;

            assert_eq!(
                b.valid_knight_moves(b.bbs[WHITE_KNIGHTS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_checks_for_check() {
            //TODO
            assert!(false);
        }
    }

    mod valid_rook_moves {
        use super::*;

        #[test]
        fn it_works_with_no_obstacles() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, E4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares = (E_FILE | RANK_4) ^ E4_SQUARE;

            assert_eq!(
                b.valid_rook_moves(b.bbs[WHITE_ROOKS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                E6_SQUARE, EMPTY, EMPTY, E4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY,
            );

            let valid_squares = (E_FILE | RANK_4) ^ E4_SQUARE ^ E6_SQUARE ^ E7_SQUARE ^ E8_SQUARE;

            assert_eq!(
                b.valid_rook_moves(b.bbs[WHITE_ROOKS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_checks_for_castling() {
            //TODO
            assert!(false);
        }

        #[test]
        fn it_checks_for_check() {
            //TODO
            assert!(false);
        }
    }

    mod valid_bishop_moves {
        use super::*;

        #[test]
        fn it_works_with_no_obstacles() {
            let b = Board::new(
                EMPTY, EMPTY, B1_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares =
                A2_SQUARE |
                C2_SQUARE |
                D3_SQUARE |
                E4_SQUARE |
                F5_SQUARE |
                G6_SQUARE |
                H7_SQUARE;

            assert_eq!(
                b.valid_bishop_moves(b.bbs[WHITE_BISHOPS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                EMPTY, G6_SQUARE, B1_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let valid_squares =
                A2_SQUARE |
                C2_SQUARE |
                D3_SQUARE |
                E4_SQUARE |
                F5_SQUARE;

            assert_eq!(
                b.valid_bishop_moves(b.bbs[WHITE_BISHOPS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_checks_for_check() {
            //TODO
            assert!(false);
        }
    }

    mod valid_queen_moves {
        use super::*;

        #[test]
        fn it_combines_rooks_and_bishop_moves() {
            let b = Board::new(
                G3_SQUARE, B6_SQUARE, EMPTY, EMPTY, G6_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY,
            );

            let bishop_moves = b.valid_bishop_moves(b.bbs[WHITE_QUEENS_BB], b.bbs[WHITE_PIECES_BB]);
            let rook_moves = b.valid_rook_moves(b.bbs[WHITE_QUEENS_BB], b.bbs[WHITE_PIECES_BB]);
            let valid_moves = bishop_moves | rook_moves;

            assert_eq!(
                b.valid_queen_moves(b.bbs[WHITE_QUEENS_BB], b.bbs[WHITE_PIECES_BB]),
                valid_moves,
            );
        }

        #[test]
        fn it_checks_for_check() {
            //TODO
            assert!(false);
        }
    }

    mod valid_white_pawn_moves {
        use super::*;

        #[test]
        fn it_works_from_home_square() {
            let b = Board::new(
                RANK_2 ^ C2_SQUARE,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
            );

            let valid_squares = A3_SQUARE
                | A4_SQUARE
                | B3_SQUARE
                | B4_SQUARE
                | D3_SQUARE
                | D4_SQUARE
                | E3_SQUARE
                | E4_SQUARE
                | F3_SQUARE
                | F4_SQUARE
                | G3_SQUARE
                | G4_SQUARE
                | H3_SQUARE
                | H4_SQUARE;

            assert_eq!(b.valid_white_pawn_moves(b.bbs[WHITE_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_from_non_home_square() {
            let b = Board::new(
                RANK_4, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
            );

            let valid_squares = A5_SQUARE
                | B5_SQUARE
                | C5_SQUARE
                | D5_SQUARE
                | E5_SQUARE
                | F5_SQUARE
                | G5_SQUARE
                | H5_SQUARE;

            assert_eq!(b.valid_white_pawn_moves(b.bbs[WHITE_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_captures() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, F6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY,
            );

            let valid_squares = E6_SQUARE | F6_SQUARE;

            assert_eq!(b.valid_white_pawn_moves(b.bbs[WHITE_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_obstacles() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY,
            );

            let valid_squares = EMPTY;

            assert_eq!(b.valid_white_pawn_moves(b.bbs[WHITE_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_checks_for_check() {
            //TODO
            assert!(false);
        }

        #[test]
        fn it_checks_for_en_passant() {
            //TODO
            assert!(false);
        }
    }

    mod valid_black_pawn_moves {
        use super::*;

        #[test]
        fn it_works_from_home_square() {
            let b = Board::new(
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                RANK_7 ^ C7_SQUARE,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
                EMPTY,
            );

            let valid_squares = A6_SQUARE
                | A5_SQUARE
                | B6_SQUARE
                | B5_SQUARE
                | D6_SQUARE
                | D5_SQUARE
                | E6_SQUARE
                | E5_SQUARE
                | F6_SQUARE
                | F5_SQUARE
                | G6_SQUARE
                | G5_SQUARE
                | H6_SQUARE
                | H5_SQUARE;

            assert_eq!(b.valid_black_pawn_moves(b.bbs[BLACK_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_from_non_home_square() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, RANK_4, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
            );

            let valid_squares = A3_SQUARE
                | B3_SQUARE
                | C3_SQUARE
                | D3_SQUARE
                | E3_SQUARE
                | F3_SQUARE
                | G3_SQUARE
                | H3_SQUARE;

            assert_eq!(b.valid_black_pawn_moves(b.bbs[BLACK_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_captures() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, F6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY,
            );

            let valid_squares = E5_SQUARE | F5_SQUARE;

            assert_eq!(b.valid_black_pawn_moves(b.bbs[BLACK_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_obstacles() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY,
            );

            let valid_squares = EMPTY;

            assert_eq!(b.valid_black_pawn_moves(b.bbs[BLACK_PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_checks_for_check() {
            //TODO
            assert!(false);
        }

        #[test]
        fn it_checks_for_en_passant() {
            //TODO
            assert!(false);
        }
    }

    mod default {
        use super::*;

        #[test]
        fn it_works() {
            let b = Board::default();
            assert_eq!(b.bbs[WHITE_PAWNS_BB], INITIAL_WHITE_PAWNS);
            assert_eq!(b.bbs[WHITE_KNIGHTS_BB], INITIAL_WHITE_KNIGHTS);
            assert_eq!(b.bbs[WHITE_BISHOPS_BB], INITIAL_WHITE_BISHOPS);
            assert_eq!(b.bbs[WHITE_ROOKS_BB], INITIAL_WHITE_ROOKS);
            assert_eq!(b.bbs[WHITE_QUEENS_BB], INITIAL_WHITE_QUEENS);
            assert_eq!(b.bbs[WHITE_KINGS_BB], INITIAL_WHITE_KINGS);
            assert_eq!(b.bbs[BLACK_PAWNS_BB], INITIAL_BLACK_PAWNS);
            assert_eq!(b.bbs[BLACK_KNIGHTS_BB], INITIAL_BLACK_KNIGHTS);
            assert_eq!(b.bbs[BLACK_BISHOPS_BB], INITIAL_BLACK_BISHOPS);
            assert_eq!(b.bbs[BLACK_ROOKS_BB], INITIAL_BLACK_ROOKS);
            assert_eq!(b.bbs[BLACK_QUEENS_BB], INITIAL_BLACK_QUEENS);
            assert_eq!(b.bbs[BLACK_KINGS_BB], INITIAL_BLACK_KINGS);
        }
    }
}
