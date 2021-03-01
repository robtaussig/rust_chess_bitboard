extern crate board;
use crate::board::Board;
extern crate bitboard;
use crate::bitboard::BitBoard;
extern crate chessmove;
use crate::chessmove::ChessMove;
extern crate constants;
use crate::constants::*;

pub struct MoveGen {}

impl MoveGen {
    //TODO test
    pub fn gen_legal_moves(board: &Board) -> Vec<ChessMove> {
        if board.checkers != EMPTY {
            if board.checkers.popcnt() > 1 {
                //TODO only generate moves that involve moving king
                Vec::new()
            } else {
                //TODO only generate moves that involve moving king, blocking check, or capturing checker
                Vec::new()
            }
        } else if board.pinned != EMPTY {
            //TODO filter pseudo legal moves for any moves that involve a pinned piece that does not move along pinned line
            MoveGen::gen_psuedo_legal_moves(board)
        } else {
            MoveGen::gen_psuedo_legal_moves(board)
        }
    }

    //TODO test
    //TODO determine castling
    pub fn gen_psuedo_legal_moves(board: &Board) -> Vec<ChessMove> {
        let mut move_vec: Vec<ChessMove> = Vec::new();
        let pawns = board.piece_bbs[board.side_to_move][PAWNS_BB];
        let knights = board.piece_bbs[board.side_to_move][KNIGHTS_BB];
        let bishops = board.piece_bbs[board.side_to_move][BISHOPS_BB];
        let rooks = board.piece_bbs[board.side_to_move][ROOKS_BB];
        let queens = board.piece_bbs[board.side_to_move][QUEENS_BB];
        let kings = board.piece_bbs[board.side_to_move][KINGS_BB];
        let own_side = board.color_bbs[board.side_to_move];

        for bit in pawns.bits() {
            let square = SQUARES[bit];
            if board.side_to_move == WHITE {
                let cm = MoveGen::valid_white_pawn_moves(board, square);
                if cm != EMPTY {
                    move_vec.push(ChessMove::new(square, cm));
                }
            } else {
                let cm = MoveGen::valid_black_pawn_moves(board, square);
                if cm != EMPTY {
                    move_vec.push(ChessMove::new(square, cm));
                }
            }
        }

        for bit in knights.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_knight_moves(board, square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in bishops.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_bishop_moves(board, square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in rooks.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_rook_moves(board, square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in queens.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_queen_moves(board, square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in kings.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_king_moves(board, square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        move_vec
    }

    pub fn valid_king_moves(_board: &Board, squares: BitBoard, own_side: BitBoard) -> BitBoard {
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

    pub fn valid_knight_moves(_board: &Board, squares: BitBoard, own_side: BitBoard) -> BitBoard {
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

    pub fn valid_white_pawn_moves(board: &Board, squares: BitBoard) -> BitBoard {
        let one_step = (squares.shl(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_3).shl(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;

        let left_attack = (squares & CLEAR_A_FILE).shl(7);
        let right_attack = (squares & CLEAR_H_FILE).shl(9);
        let attacks = left_attack | right_attack;
        let valid_attacks = attacks & board.color_bbs[BLACK];

        valid_steps | valid_attacks
    }

    pub fn valid_black_pawn_moves(board: &Board, squares: BitBoard) -> BitBoard {
        let one_step = (squares.shr(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_6).shr(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;

        let left_attack = (squares & CLEAR_A_FILE).shr(9);
        let right_attack = (squares & CLEAR_H_FILE).shr(7);
        let attacks = left_attack | right_attack;
        let valid_attacks = attacks & board.color_bbs[WHITE];

        valid_steps | valid_attacks
    }

    pub fn valid_rook_moves(board: &Board, squares: BitBoard, own_pieces: BitBoard) -> BitBoard {
        MoveGen::south_attacks(board, squares, own_pieces)
            | MoveGen::north_attacks(board, squares, own_pieces)
            | MoveGen::east_attacks(board, squares, own_pieces)
            | MoveGen::west_attacks(board, squares, own_pieces)
    }

    pub fn valid_bishop_moves(board: &Board, squares: BitBoard, own_pieces: BitBoard) -> BitBoard {
        MoveGen::south_east_attacks(board, squares, own_pieces)
            | MoveGen::north_east_attacks(board, squares, own_pieces)
            | MoveGen::north_west_attacks(board, squares, own_pieces)
            | MoveGen::south_west_attacks(board, squares, own_pieces)
    }

    pub fn valid_queen_moves(board: &Board, squares: BitBoard, own_pieces: BitBoard) -> BitBoard {
        MoveGen::valid_rook_moves(board, squares, own_pieces)
            | MoveGen::valid_bishop_moves(board, squares, own_pieces)
    }

    pub fn south_attacks(board: &Board, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        attacks |= board.combined_bbs[EMPTY_SQUARES_BB] & (attacks.shr(8));
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & (board.combined_bbs[EMPTY_SQUARES_BB].shr(8));
        attacks |= empty & (attacks.shr(16));
        empty &= empty.shr(16);
        attacks |= empty & (attacks.shr(32));
        (attacks.shr(8)) & !own_pieces
    }

    pub fn north_attacks(board: &Board, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        attacks |= board.combined_bbs[EMPTY_SQUARES_BB] & (attacks.shl(8));
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & (board.combined_bbs[EMPTY_SQUARES_BB].shl(8));
        attacks |= empty & (attacks.shl(16));
        empty &= empty.shl(16);
        attacks |= empty & (attacks.shl(32));
        (attacks.shl(8)) & !own_pieces
    }

    pub fn west_attacks(board: &Board, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & CLEAR_H_FILE;
        attacks |= empty & (attacks.shr(1));
        empty &= empty.shr(1);
        attacks |= empty & (attacks.shr(2));
        empty &= empty.shr(2);
        attacks |= empty & (attacks.shr(4));
        attacks.shr(1) & !own_pieces & CLEAR_H_FILE
    }

    pub fn east_attacks(board: &Board, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & CLEAR_A_FILE;
        attacks |= empty & (attacks.shl(1));
        empty &= empty.shl(1);
        attacks |= empty & (attacks.shl(2));
        empty &= empty.shl(2);
        attacks |= empty & (attacks.shl(4));
        attacks.shl(1) & !own_pieces & CLEAR_A_FILE
    }

    pub fn north_east_attacks(
        board: &Board,
        mut attacks: BitBoard,
        own_pieces: BitBoard,
    ) -> BitBoard {
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & CLEAR_A_FILE;
        attacks |= empty & (attacks.shl(9));
        empty &= empty.shl(9);
        attacks |= empty & (attacks.shl(18));
        empty &= empty.shl(18);
        attacks |= empty & (attacks.shl(36));
        attacks.shl(9) & !own_pieces & CLEAR_A_FILE
    }

    pub fn south_east_attacks(
        board: &Board,
        mut attacks: BitBoard,
        own_pieces: BitBoard,
    ) -> BitBoard {
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & CLEAR_A_FILE;
        attacks |= empty & (attacks.shr(7));
        empty &= empty.shr(7);
        attacks |= empty & (attacks.shr(14));
        empty &= empty.shr(14);
        attacks |= empty & (attacks.shr(28));
        attacks.shr(7) & !own_pieces & CLEAR_A_FILE
    }

    pub fn south_west_attacks(
        board: &Board,
        mut attacks: BitBoard,
        own_pieces: BitBoard,
    ) -> BitBoard {
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & CLEAR_H_FILE;
        attacks |= empty & (attacks.shr(9));
        empty &= empty.shr(9);
        attacks |= empty & (attacks.shr(18));
        empty &= empty.shr(18);
        attacks |= empty & (attacks.shr(36));
        attacks.shr(9) & !own_pieces & CLEAR_H_FILE
    }

    pub fn north_west_attacks(
        board: &Board,
        mut attacks: BitBoard,
        own_pieces: BitBoard,
    ) -> BitBoard {
        let mut empty = board.combined_bbs[EMPTY_SQUARES_BB] & CLEAR_H_FILE;
        attacks |= empty & (attacks.shl(7));
        empty &= empty.shl(7);
        attacks |= empty & (attacks.shl(14));
        empty &= empty.shl(14);
        attacks |= empty & (attacks.shl(28));
        attacks.shl(7) & !own_pieces & CLEAR_H_FILE
    }

    //TODO test
    //TODO implement pinned
    pub fn find_checkers_and_pinners(board: &Board) -> (BitBoard, BitBoard) {
        let mut checkers = EMPTY;

        let ksq = board.piece_bbs[board.side_to_move][KINGS_BB];
        let other_pieces_collection: [BitBoard; 6];
        if board.side_to_move == WHITE {
            other_pieces_collection = board.piece_bbs[BLACK];
        } else {
            other_pieces_collection = board.piece_bbs[WHITE];
        }

        let bishop_attackers =
            MoveGen::valid_bishop_moves(board, ksq, board.color_bbs[board.side_to_move]);
        let rook_attackers =
            MoveGen::valid_rook_moves(board, ksq, board.color_bbs[board.side_to_move]);
        let knight_attackers =
            MoveGen::valid_knight_moves(board, ksq, board.color_bbs[board.side_to_move]);
        let pawn_attackers: BitBoard;
        if board.side_to_move == WHITE {
            pawn_attackers = MoveGen::valid_white_pawn_moves(board, ksq);
        } else {
            pawn_attackers = MoveGen::valid_black_pawn_moves(board, ksq);
        }

        checkers ^= bishop_attackers
            & (other_pieces_collection[BISHOPS_BB] | other_pieces_collection[QUEENS_BB]);
        checkers ^= rook_attackers
            & (other_pieces_collection[ROOKS_BB] | other_pieces_collection[QUEENS_BB]);
        checkers ^= knight_attackers & other_pieces_collection[KNIGHTS_BB];
        checkers ^= pawn_attackers & other_pieces_collection[PAWNS_BB];

        (checkers, EMPTY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    mod gen_psuedo_legal_moves {
        use super::*;

        #[test]
        fn it_works() {
            let b = Board::default();
            let chessmoves = ChessMove::broken_up(MoveGen::gen_psuedo_legal_moves(&b));

            for chessmove in chessmoves {
                chessmove.from.print_bb();
                chessmove.to.print_bb();
            }
        }
    }

    mod valid_king_moves {
        use super::*;

        #[test]
        fn it_works_with_no_obstacles() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E3_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, WHITE,
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
                MoveGen::valid_king_moves(&b, b.piece_bbs[WHITE][KINGS_BB], b.color_bbs[WHITE]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_edges() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, A1_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, WHITE,
            );

            let valid_squares = A2_SQUARE | B2_SQUARE | B1_SQUARE;

            assert_eq!(
                MoveGen::valid_king_moves(&b, b.piece_bbs[WHITE][KINGS_BB], b.color_bbs[WHITE]),
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
                WHITE,
            );

            let valid_squares = F3_SQUARE | F2_SQUARE | D3_SQUARE;

            assert_eq!(
                MoveGen::valid_king_moves(&b, b.piece_bbs[WHITE][KINGS_BB], b.color_bbs[WHITE]),
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
                EMPTY, WHITE,
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
                MoveGen::valid_knight_moves(&b, b.piece_bbs[WHITE][KNIGHTS_BB], b.color_bbs[WHITE]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_edges() {
            let b = Board::new(
                EMPTY, B4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, WHITE,
            );

            let valid_squares =
                A6_SQUARE | C6_SQUARE | D5_SQUARE | D3_SQUARE | C2_SQUARE | A2_SQUARE;

            assert_eq!(
                MoveGen::valid_knight_moves(&b, b.piece_bbs[WHITE][KNIGHTS_BB], b.color_bbs[WHITE]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                C_FILE, B4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, WHITE,
            );

            let valid_squares = A6_SQUARE | D5_SQUARE | D3_SQUARE | A2_SQUARE;

            assert_eq!(
                MoveGen::valid_knight_moves(&b, b.piece_bbs[WHITE][KNIGHTS_BB], b.color_bbs[WHITE]),
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
                EMPTY, WHITE,
            );

            let valid_squares = (E_FILE | RANK_4) ^ E4_SQUARE;

            assert_eq!(
                MoveGen::valid_rook_moves(&b, b.piece_bbs[WHITE][ROOKS_BB], b.color_bbs[WHITE]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                E6_SQUARE, EMPTY, EMPTY, E4_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = (E_FILE | RANK_4) ^ E4_SQUARE ^ E6_SQUARE ^ E7_SQUARE ^ E8_SQUARE;

            assert_eq!(
                MoveGen::valid_rook_moves(&b, b.piece_bbs[WHITE][ROOKS_BB], b.color_bbs[WHITE]),
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
                EMPTY, WHITE,
            );

            let valid_squares =
                A2_SQUARE | C2_SQUARE | D3_SQUARE | E4_SQUARE | F5_SQUARE | G6_SQUARE | H7_SQUARE;

            assert_eq!(
                MoveGen::valid_bishop_moves(&b, b.piece_bbs[WHITE][BISHOPS_BB], b.color_bbs[WHITE]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                EMPTY, G6_SQUARE, B1_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = A2_SQUARE | C2_SQUARE | D3_SQUARE | E4_SQUARE | F5_SQUARE;

            assert_eq!(
                MoveGen::valid_bishop_moves(&b, b.piece_bbs[WHITE][BISHOPS_BB], b.color_bbs[WHITE]),
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
                G3_SQUARE, B6_SQUARE, EMPTY, EMPTY, G6_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let bishop_moves =
                MoveGen::valid_bishop_moves(&b, b.piece_bbs[WHITE][QUEENS_BB], b.color_bbs[WHITE]);
            let rook_moves =
                MoveGen::valid_rook_moves(&b, b.piece_bbs[WHITE][QUEENS_BB], b.color_bbs[WHITE]);
            let valid_moves = bishop_moves | rook_moves;

            assert_eq!(
                MoveGen::valid_queen_moves(&b, b.piece_bbs[WHITE][QUEENS_BB], b.color_bbs[WHITE]),
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
                WHITE,
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

            assert_eq!(
                MoveGen::valid_white_pawn_moves(&b, b.piece_bbs[WHITE][PAWNS_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_from_non_home_square() {
            let b = Board::new(
                RANK_4, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, WHITE,
            );

            let valid_squares = A5_SQUARE
                | B5_SQUARE
                | C5_SQUARE
                | D5_SQUARE
                | E5_SQUARE
                | F5_SQUARE
                | G5_SQUARE
                | H5_SQUARE;

            assert_eq!(
                MoveGen::valid_white_pawn_moves(&b, b.piece_bbs[WHITE][PAWNS_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_captures() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, F6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = E6_SQUARE | F6_SQUARE;

            assert_eq!(
                MoveGen::valid_white_pawn_moves(&b, b.piece_bbs[WHITE][PAWNS_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_obstacles() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = EMPTY;

            assert_eq!(
                MoveGen::valid_white_pawn_moves(&b, b.piece_bbs[WHITE][PAWNS_BB]),
                valid_squares
            );
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
                WHITE,
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

            assert_eq!(
                MoveGen::valid_black_pawn_moves(&b, b.piece_bbs[BLACK][PAWNS_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_from_non_home_square() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, RANK_4, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, WHITE,
            );

            let valid_squares = A3_SQUARE
                | B3_SQUARE
                | C3_SQUARE
                | D3_SQUARE
                | E3_SQUARE
                | F3_SQUARE
                | G3_SQUARE
                | H3_SQUARE;

            assert_eq!(
                MoveGen::valid_black_pawn_moves(&b, b.piece_bbs[BLACK][PAWNS_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_captures() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, F6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = E5_SQUARE | F5_SQUARE;

            assert_eq!(
                MoveGen::valid_black_pawn_moves(&b, b.piece_bbs[BLACK][PAWNS_BB]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_obstacles() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = EMPTY;

            assert_eq!(
                MoveGen::valid_black_pawn_moves(&b, b.piece_bbs[BLACK][PAWNS_BB]),
                valid_squares
            );
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
}
