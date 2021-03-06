extern crate board;
use crate::board::Board;
extern crate bitboard;
use crate::bitboard::*;
extern crate chessmove;
use crate::chessmove::ChessMove;
extern crate constants;
use crate::constants::*;
extern crate magic;
use crate::magic::between_generated::*;

pub fn between_bb(left: BitBoard, right: BitBoard) -> BitBoard {
    BETWEEN[left.index()][right.index()]
}

pub struct MoveGen {}

impl MoveGen {
    //TODO test
    pub fn gen_legal_moves(board: &Board) -> Vec<ChessMove> {
        let ksq = board.piece_bbs[board.side_to_move][KINGS_BB];
        let valid_king_moves =
            MoveGen::valid_king_moves(board, ksq, board.color_bbs[board.side_to_move]);
        let safe_squares = valid_king_moves & !board.attacked_squares;

        // Instead of testing every move for whether it leaves the player in check,
        // first filter out king moves that place the king on an attacked square.
        // Then later filter out moves involving pinned pieces (pre-calculated) that move out of the pin,
        // And then filtler out moves that do not address an immediate check (pre-calculated).
        let safe_king_moves = MoveGen::gen_psuedo_legal_moves(board)
            .into_iter()
            .map(|chessmove| {
                if chessmove.from != ksq {
                    return chessmove;
                }
                ChessMove::new(chessmove.from, safe_squares)
            })
            .filter(|chessmove| chessmove.to.is_not_empty())
            .collect::<Vec<ChessMove>>();

        // If king is currently in check, only consider moves that:
        // a) Move the king
        // b) Capture checking piece
        // c) Block checking piece
        if board.checkers.is_not_empty() {
            // If king is in check by more than one piece, the only valid response is to move the king
            if board.checkers.popcnt() > 1 {
                if safe_squares.is_empty() {
                    return Vec::new();
                }
                safe_king_moves
                    .into_iter()
                    .filter(|chessmove| {
                        if chessmove.from != ksq {
                            return false;
                        }
                        true
                    })
                    .collect::<Vec<ChessMove>>()
            } else {
                // Only 1 piece is attacking the king, so between_bb() returns path from attacker to king
                // A non-king move can only be valid by capturing attacker or moving to path
                let attack_ray = between_bb(board.checkers, ksq);

                safe_king_moves
                    .into_iter()
                    .map(|chessmove| {
                        if chessmove.from == ksq {
                            return chessmove;
                        }

                        let allowed_squares =
                            MoveGen::filter_moves_out_of_pin(board, &chessmove, ksq);

                        ChessMove::new(
                            chessmove.from,
                            allowed_squares & (board.checkers | attack_ray),
                        )
                    })
                    .filter(|chessmove| chessmove.to.is_not_empty())
                    .collect::<Vec<ChessMove>>()
            }
        } else {
            // King is not in check
            safe_king_moves
                .into_iter()
                .map(|chessmove| {
                    if chessmove.from == ksq {
                        return chessmove;
                    }

                    if (chessmove.from & board.pinned).is_empty() {
                        return chessmove;
                    }

                    let allowed_squares = MoveGen::filter_moves_out_of_pin(board, &chessmove, ksq);

                    ChessMove::new(chessmove.from, allowed_squares)
                })
                .filter(|chessmove| chessmove.to.is_not_empty())
                .collect::<Vec<ChessMove>>()
        }
    }

    fn filter_moves_out_of_pin(
        board: &Board,
        chessmove: &ChessMove,
        king_square: BitBoard,
    ) -> BitBoard {
        match (chessmove.from & board.pinned).is_empty() {
            // Piece is not pinned
            true => chessmove.to,
            // Piece is pinned, so only squares on the same attack ray are valid destinations
            // This can determined if the destination is between the king and the origin,
            // or if the origin is between the destination and the king; the latter
            // can be safely assumed because a sliding piece cannot skip over the attacker
            false => chessmove.to.bits().fold(EMPTY, |acc, bit| {
                let square = SQUARES[bit];
                if (between_bb(king_square, square) & chessmove.from).is_not_empty()
                    || (between_bb(king_square, chessmove.from) & square).is_not_empty()
                {
                    acc | square
                } else {
                    acc
                }
            }),
        }
    }

    //TODO test
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
                if cm.is_not_empty() {
                    move_vec.push(ChessMove::new(square, cm));
                }
            } else {
                let cm = MoveGen::valid_black_pawn_moves(board, square);
                if cm.is_not_empty() {
                    move_vec.push(ChessMove::new(square, cm));
                }
            }
        }

        for bit in knights.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_knight_moves(board, square, own_side);
            if cm.is_not_empty() {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in bishops.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_bishop_moves(board, square, own_side);
            if cm.is_not_empty() {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in rooks.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_rook_moves(board, square, own_side);
            if cm.is_not_empty() {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in queens.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_queen_moves(board, square, own_side);
            if cm.is_not_empty() {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        for bit in kings.bits() {
            let square = SQUARES[bit];
            let cm = MoveGen::valid_king_moves(board, square, own_side);
            if cm.is_not_empty() {
                move_vec.push(ChessMove::new(square, cm));
            }
        }

        move_vec
    }

    pub fn valid_king_moves(board: &Board, squares: BitBoard, own_side: BitBoard) -> BitBoard {
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

        let mut kingside_castle_move = EMPTY;
        let mut queenside_castle_move = EMPTY;

        if board.checkers.is_empty() {
            if board.side_to_move == WHITE
                && squares == E1_SQUARE
                && ((board.castle_rights & (G1_SQUARE | C1_SQUARE)).is_not_empty())
            {
                if board.combined_bbs[EMPTY_SQUARES_BB]
                    & (WHITE_KINGSIDE_CASTLE_EMPTY_SQUARES & !board.attacked_squares)
                    == WHITE_KINGSIDE_CASTLE_EMPTY_SQUARES
                {
                    kingside_castle_move = G1_SQUARE;
                }
                if board.combined_bbs[EMPTY_SQUARES_BB]
                    & (WHITE_QUEENSIDE_CASTLE_EMPTY_SQUARES & !board.attacked_squares)
                    == WHITE_QUEENSIDE_CASTLE_EMPTY_SQUARES
                {
                    queenside_castle_move = C1_SQUARE;
                }
            } else if board.side_to_move == BLACK
                && squares == E8_SQUARE
                && ((board.castle_rights & (G8_SQUARE | C8_SQUARE)).is_not_empty())
            {
                if board.combined_bbs[EMPTY_SQUARES_BB]
                    & (BLACK_KINGSIDE_CASTLE_EMPTY_SQUARES & !board.attacked_squares)
                    == BLACK_KINGSIDE_CASTLE_EMPTY_SQUARES
                {
                    kingside_castle_move = G8_SQUARE;
                }
                if board.combined_bbs[EMPTY_SQUARES_BB]
                    & (BLACK_QUEENSIDE_CASTLE_EMPTY_SQUARES & !board.attacked_squares)
                    == BLACK_QUEENSIDE_CASTLE_EMPTY_SQUARES
                {
                    queenside_castle_move = C8_SQUARE;
                }
            }
        }

        let moves = left_up
            | up
            | right_up
            | right
            | down_right
            | down
            | left_down
            | left
            | kingside_castle_move
            | queenside_castle_move;

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

    pub fn valid_white_pawn_attacks(board: &Board, squares: BitBoard) -> BitBoard {
        let left_attack = (squares & CLEAR_A_FILE).shl(7);
        let right_attack = (squares & CLEAR_H_FILE).shl(9);
        let attacks = left_attack | right_attack;
        attacks & (board.color_bbs[BLACK] | board.en_passant)
    }

    pub fn valid_white_pawn_moves(board: &Board, squares: BitBoard) -> BitBoard {
        let one_step = (squares.shl(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_3).shl(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;
        let valid_attacks = MoveGen::valid_white_pawn_attacks(board, squares);

        valid_steps | valid_attacks
    }

    pub fn valid_black_pawn_attacks(board: &Board, squares: BitBoard) -> BitBoard {
        let left_attack = (squares & CLEAR_A_FILE).shr(9);
        let right_attack = (squares & CLEAR_H_FILE).shr(7);
        let attacks = left_attack | right_attack;
        attacks & board.color_bbs[WHITE]
    }

    pub fn valid_black_pawn_moves(board: &Board, squares: BitBoard) -> BitBoard {
        let one_step = (squares.shr(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_6).shr(8)) & board.combined_bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;
        let valid_attacks = MoveGen::valid_black_pawn_attacks(board, squares);

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
        let mut empty =
            board.combined_bbs[EMPTY_SQUARES_BB] & (board.combined_bbs[EMPTY_SQUARES_BB].shr(8));
        attacks |= empty & (attacks.shr(16));
        empty &= empty.shr(16);
        attacks |= empty & (attacks.shr(32));
        (attacks.shr(8)) & !own_pieces
    }

    pub fn north_attacks(board: &Board, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        attacks |= board.combined_bbs[EMPTY_SQUARES_BB] & (attacks.shl(8));
        let mut empty =
            board.combined_bbs[EMPTY_SQUARES_BB] & (board.combined_bbs[EMPTY_SQUARES_BB].shl(8));
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

    pub fn find_attack_rays(board: &Board, test_square: BitBoard, own_board: BitBoard) -> BitBoard {
        let (other_pieces_collection, other_pieces) = match board.side_to_move {
            WHITE => (board.piece_bbs[BLACK], board.color_bbs[BLACK]),
            BLACK => (board.piece_bbs[WHITE], board.color_bbs[WHITE]),
            _ => ([EMPTY; 6], EMPTY),
        };

        let mut bishop_attacks = MoveGen::valid_bishop_moves(board, test_square, own_board);
        let mut rook_attacks = MoveGen::valid_rook_moves(board, test_square, own_board);

        let bishop_like_attackers = bishop_attacks
            & (other_pieces_collection[BISHOPS_BB] | other_pieces_collection[QUEENS_BB]);

        if bishop_like_attackers.is_empty() {
            bishop_attacks = EMPTY;
        } else {
            bishop_attacks &=
                MoveGen::valid_bishop_moves(board, bishop_like_attackers, other_pieces);
        }

        let rook_like_attackers =
            rook_attacks & (other_pieces_collection[ROOKS_BB] | other_pieces_collection[QUEENS_BB]);

        if rook_like_attackers.is_empty() {
            rook_attacks = EMPTY;
        } else {
            rook_attacks &= MoveGen::valid_rook_moves(board, rook_like_attackers, other_pieces);
        }

        bishop_attacks | rook_attacks
    }

    pub fn calculate_derived_bitboards(board: &Board) -> (BitBoard, BitBoard, BitBoard) {
        let (checkers, pinned) = MoveGen::find_checkers_and_pinned_pieces(&board);
        let attacked_squares = MoveGen::find_attacked_squares(&board);
        (checkers, pinned, attacked_squares)
    }

    pub fn find_pinned_pieces(board: &Board) -> BitBoard {
        let ksq = board.piece_bbs[board.side_to_move][KINGS_BB];

        let other_pieces_collection = match board.side_to_move {
            WHITE => board.piece_bbs[BLACK],
            BLACK => board.piece_bbs[WHITE],
            _ => panic!("Invalid side to move"),
        };

        let bishop_like_attackers =
            other_pieces_collection[BISHOPS_BB] | other_pieces_collection[QUEENS_BB];
        let rook_like_attackers =
            other_pieces_collection[ROOKS_BB] | other_pieces_collection[QUEENS_BB];

        let bishop_like_attackers_without_blockers =
            bishop_like_attackers.bits().fold(EMPTY, |acc, bit| {
                let square = SQUARES[bit];
                if between_bb(square, ksq).is_not_empty()
                    && ksq.row() != square.row()
                    && ksq.col() != square.col()
                {
                    acc | square
                } else {
                    acc
                }
            });

        let rook_like_attackers_without_blockers =
            rook_like_attackers.bits().fold(EMPTY, |acc, bit| {
                let square = SQUARES[bit];
                if between_bb(square, ksq).is_not_empty()
                    && (ksq.row() == square.row() || ksq.col() == square.col())
                {
                    acc | square
                } else {
                    acc
                }
            });

        let attackers_without_blockers =
            bishop_like_attackers_without_blockers | rook_like_attackers_without_blockers;

        let mut pinned = EMPTY;
        attackers_without_blockers.bits().for_each(|attacker_bit| {
            let attacker_square = SQUARES[attacker_bit];
            let king_to_attacker = between_bb(attacker_square, ksq);
            let mut num_blockers = 0;
            let mut blocker = EMPTY;
            if king_to_attacker.is_not_empty() {
                board.color_bbs[board.side_to_move]
                    .bits()
                    .for_each(|blocker_bit| {
                        let test_blocker = SQUARES[blocker_bit];
                        if (king_to_attacker & test_blocker).is_not_empty() {
                            blocker = test_blocker;
                            num_blockers += 1;
                        }
                    });
                if num_blockers == 1 {
                    pinned |= blocker;
                }
            }
        });
        pinned
    }

    pub fn find_attackers(board: &Board, test_square: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut attackers = EMPTY;

        let other_pieces_collection: [BitBoard; 6];
        if board.side_to_move == WHITE {
            other_pieces_collection = board.piece_bbs[BLACK];
        } else {
            other_pieces_collection = board.piece_bbs[WHITE];
        }

        let bishop_attackers = MoveGen::valid_bishop_moves(board, test_square, own_pieces);
        let rook_attackers = MoveGen::valid_rook_moves(board, test_square, own_pieces);
        let knight_attackers = MoveGen::valid_knight_moves(board, test_square, own_pieces);
        let pawn_attackers: BitBoard;
        if board.side_to_move == WHITE {
            pawn_attackers = MoveGen::valid_white_pawn_attacks(board, test_square);
        } else {
            pawn_attackers = MoveGen::valid_black_pawn_attacks(board, test_square);
        }

        attackers ^= bishop_attackers
            & (other_pieces_collection[BISHOPS_BB] | other_pieces_collection[QUEENS_BB]);
        attackers ^= rook_attackers
            & (other_pieces_collection[ROOKS_BB] | other_pieces_collection[QUEENS_BB]);
        attackers ^= knight_attackers & other_pieces_collection[KNIGHTS_BB];
        attackers ^= pawn_attackers & other_pieces_collection[PAWNS_BB];

        attackers
    }

    //TODO test
    pub fn find_checkers_and_pinned_pieces(board: &Board) -> (BitBoard, BitBoard) {
        let ksq = board.piece_bbs[board.side_to_move][KINGS_BB];
        let checkers = MoveGen::find_attackers(board, ksq, board.color_bbs[board.side_to_move]);
        let pinned = MoveGen::find_pinned_pieces(board);

        (checkers, pinned)
    }

    //TODO test
    pub fn find_attacked_squares(board: &Board) -> BitBoard {
        let other_pieces = board.other_pieces();
        let mut attacked_squares = EMPTY;

        attacked_squares |= MoveGen::valid_queen_moves(board, other_pieces[QUEENS_BB], EMPTY);
        attacked_squares |= MoveGen::valid_bishop_moves(board, other_pieces[BISHOPS_BB], EMPTY);
        attacked_squares |= MoveGen::valid_rook_moves(board, other_pieces[ROOKS_BB], EMPTY);
        attacked_squares |= MoveGen::valid_knight_moves(board, other_pieces[KNIGHTS_BB], EMPTY);
        attacked_squares |= MoveGen::valid_king_moves(board, other_pieces[KINGS_BB], EMPTY);
        let (left_pawn_attacks, right_pawn_attacks) = match board.side_to_move {
            WHITE => (
                (other_pieces[PAWNS_BB] & CLEAR_A_FILE).shr(9),
                (other_pieces[PAWNS_BB] & CLEAR_H_FILE).shr(7),
            ),
            BLACK => (
                (other_pieces[PAWNS_BB] & CLEAR_A_FILE).shl(7),
                (other_pieces[PAWNS_BB] & CLEAR_H_FILE).shl(9),
            ),
            _ => panic!("Invalid side to move"),
        };
        attacked_squares |= left_pawn_attacks | right_pawn_attacks;
        attacked_squares
    }
}

pub fn init_board_from_fen(fen: &str) -> Board {
    let mut b = Board::from_fen(fen);
    let (checkers, pinned, attacked_squares) = MoveGen::calculate_derived_bitboards(&b);
    b.attacked_squares = attacked_squares;
    b.checkers = checkers;
    b.pinned = pinned;
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    mod castling {
        use super::*;

        #[test]
        fn cannot_castle_through_check() {
            let b = init_board_from_fen(
                "r2qkbnr/ppp1pppp/8/1b1pn3/P7/5N2/1PPP1PPP/RNBQK2R w KQkq - 0 1",
            );
            let moves = MoveGen::gen_legal_moves(&b);
            assert_eq!(
                moves
                    .iter()
                    .find(|cm| { cm.from == E1_SQUARE && ((cm.to & G1_SQUARE).is_not_empty()) }),
                None
            );
        }

        #[test]
        fn cannot_castle_out_of_check() {
            let b = init_board_from_fen(
                "rnbqk1nr/2pp1ppp/pp6/1B2p3/1b1PP3/5N2/PPP2PPP/RNBQK2R w KQkq - 0 1",
            );
            let moves = MoveGen::gen_legal_moves(&b);
            assert_eq!(
                moves
                    .iter()
                    .find(|cm| { cm.from == E1_SQUARE && ((cm.to & G1_SQUARE).is_not_empty()) }),
                None
            );
        }
    }

    mod moving_into_check {
        use super::*;

        #[test]
        fn it_works() {
            let b = init_board_from_fen(
                "r1bqkbnr/1pp2pp1/p1n4p/1B1pp1B1/4P3/3P1N1P/PPP2PP1/RN1QK2R b KQkq - 0 1",
            );
            let moves = MoveGen::gen_legal_moves(&b);
            assert_eq!(
                moves
                    .iter()
                    .find(|cm| { cm.from == E8_SQUARE && ((cm.to & E7_SQUARE).is_not_empty()) }),
                None
            );
        }
    }

    mod attacked_squares {
        #[test]
        fn it_works() {
            //TODO
        }
    }

    mod pinned_pieces {
        use super::*;

        #[test]
        fn it_works() {
            let b = init_board_from_fen(
                "rnbqkbnr/pppp1ppp/8/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 0 1",
            );
            let pinned = MoveGen::find_pinned_pieces(&b);

            assert_eq!(pinned, F7_SQUARE);
        }
    }
}
