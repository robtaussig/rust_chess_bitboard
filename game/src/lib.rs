extern crate board;
use crate::board::Board;
extern crate piece;
use crate::piece::Pieces;
extern crate movegen;
use crate::movegen::MoveGen;
extern crate chessmove;
use crate::chessmove::ChessMove;
extern crate constants;
use crate::constants::*;
extern crate bitboard;
use crate::bitboard::*;
mod moment;
use board::BoardParams;
use moment::*;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub prev_board: Option<Board>,
    pub history: Vec<Moment>,
    pub future: Vec<Moment>,
    record_history: bool,
}

impl Game {
    pub fn new(mut board: Board) -> Self {
        let (checkers, pinned, attacked_squares) = MoveGen::calculate_derived_bitboards(&board);
        board.checkers = checkers;
        board.pinned = pinned;
        board.attacked_squares = attacked_squares;

        Game {
            board,
            prev_board: None,
            history: vec![Moment::new(board.to_fen(), (EMPTY, EMPTY))],
            future: Vec::new(),
            record_history: true,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let board = Board::from_fen(fen);
        Game::new(board)
    }

    pub fn restart_game(&mut self) {
        self.board = Board::default();
        self.record_moment((EMPTY, EMPTY));
    }

    pub fn restart_from_fen(&mut self, fen: &str) {
        self.board = Board::from_fen(fen);
        self.record_moment((EMPTY, EMPTY));
    }

    pub fn go_back(&mut self) -> (BitBoard, BitBoard) {
        self.future.push(self.history.pop().unwrap());
        let moment = self.history.last().unwrap();
        self.board = Board::from_fen(moment.fen.as_str());
        moment.last_move
    }

    pub fn go_forward(&mut self) -> (BitBoard, BitBoard) {
        self.history.push(self.future.pop().unwrap());
        let moment = self.history.last().unwrap();
        self.board = Board::from_fen(moment.fen.as_str());
        moment.last_move
    }

    pub fn record_moment(&mut self, last_move: (BitBoard, BitBoard)) {
        if self.record_history {
            let board_fen = self.board.to_fen();
            self.history.push(Moment::new(board_fen, last_move));
            self.future = Vec::new();
        }
    }

    //TODO test
    //Returns list of all moved pieces, including castling rook
    pub fn make_move(&mut self, chessmove: &ChessMove) -> Vec<(BitBoard, BitBoard)> {
        self.prev_board = Some(self.board);
        let prev_en_passant = self.board.en_passant;
        self.board.en_passant = EMPTY;
        let mut moves: Vec<(BitBoard, BitBoard)> = Vec::new();

        let moving_piece = self.board.get_piece_at(chessmove.from);
        let target_piece = self.board.get_piece_at(chessmove.to);

        self.board.move_piece(chessmove.from, chessmove.to);

        moves.push((chessmove.from, chessmove.to));

        match target_piece {
            Pieces::WRook => {
                if chessmove.to == A1_SQUARE {
                    self.board.castle_rights &= !C1_SQUARE;
                } else if chessmove.to == H1_SQUARE {
                    self.board.castle_rights &= !G1_SQUARE;
                }
            }
            Pieces::BRook => {
                if chessmove.to == A8_SQUARE {
                    self.board.castle_rights &= !C8_SQUARE;
                } else if chessmove.to == H8_SQUARE {
                    self.board.castle_rights &= !G8_SQUARE;
                }
            }
            Pieces::WKing => {
                panic!("King invalidly captured");
            }
            Pieces::BKing => {
                panic!("King invalidly captured");
            }
            _ => (),
        }

        match moving_piece {
            Pieces::WPawn => {
                if chessmove.to == prev_en_passant {
                    let piece_to_remove = prev_en_passant.shr(8);
                    self.board.piece_bbs[BLACK][PAWNS_BB] ^= piece_to_remove;
                    self.board.color_bbs[BLACK] ^= piece_to_remove;
                    self.board.combined_bbs[ALL_PAWNS_BB] ^= piece_to_remove;

                    self.board.combined_bbs[EMPTY_SQUARES_BB] |= piece_to_remove;
                    self.board.combined_bbs[ALL_PIECES_BB] ^= piece_to_remove;
                }

                if chessmove.from.shl(16) == chessmove.to {
                    self.board.en_passant = chessmove.from.shl(8);
                }
            }
            Pieces::BPawn => {
                if chessmove.to == prev_en_passant {
                    let piece_to_remove = prev_en_passant.shl(8);
                    self.board.piece_bbs[WHITE][PAWNS_BB] ^= piece_to_remove;
                    self.board.color_bbs[WHITE] ^= piece_to_remove;
                    self.board.combined_bbs[ALL_PAWNS_BB] ^= piece_to_remove;

                    self.board.combined_bbs[EMPTY_SQUARES_BB] |= piece_to_remove;
                    self.board.combined_bbs[ALL_PIECES_BB] ^= piece_to_remove;
                }

                if chessmove.from.shr(16) == chessmove.to {
                    self.board.en_passant = chessmove.from.shr(8);
                }
            }
            Pieces::WRook => {
                if chessmove.from == A1_SQUARE {
                    self.board.castle_rights &= !C1_SQUARE;
                } else if chessmove.from == H1_SQUARE {
                    self.board.castle_rights &= !G1_SQUARE;
                }
            }
            Pieces::BRook => {
                if chessmove.from == A8_SQUARE {
                    self.board.castle_rights &= !C8_SQUARE;
                } else if chessmove.from == H8_SQUARE {
                    self.board.castle_rights &= !G8_SQUARE;
                }
            }
            Pieces::WKing => {
                if chessmove.from == E1_SQUARE && chessmove.to == G1_SQUARE {
                    self.board.move_piece(H1_SQUARE, F1_SQUARE);
                    moves.push((H1_SQUARE, F1_SQUARE));
                } else if chessmove.from == E1_SQUARE && chessmove.to == C1_SQUARE {
                    self.board.move_piece(A1_SQUARE, D1_SQUARE);
                    moves.push((A1_SQUARE, D1_SQUARE));
                }

                self.board.castle_rights &= !(C1_SQUARE | G1_SQUARE);
            }
            Pieces::BKing => {
                if chessmove.from == E8_SQUARE && chessmove.to == G8_SQUARE {
                    self.board.move_piece(H8_SQUARE, F8_SQUARE);
                    moves.push((H8_SQUARE, F8_SQUARE));
                } else if chessmove.from == E8_SQUARE && chessmove.to == C8_SQUARE {
                    self.board.move_piece(A8_SQUARE, D8_SQUARE);
                    moves.push((A8_SQUARE, D8_SQUARE));
                }

                self.board.castle_rights &= !(C8_SQUARE | G8_SQUARE);
            }
            _ => (),
        }

        if let Some(promotion) = chessmove.promotion {
            self.board.piece_bbs[moving_piece.color_bb_index()]
                [moving_piece.piece_by_color_bb_index()] ^= chessmove.to;
            self.board.combined_bbs[moving_piece.combined_color_bb_index()] ^= chessmove.to;

            self.board.piece_bbs[promotion.color_bb_index()]
                [promotion.piece_by_color_bb_index()] |= chessmove.to;
            self.board.combined_bbs[promotion.combined_color_bb_index()] |= chessmove.to;
            moves.push((EMPTY, chessmove.to));
        }
        self.board.switch_side_to_move();
        let (checkers, pinned, attacked_squares) = MoveGen::calculate_derived_bitboards(&self.board);
        self.board.checkers = checkers;
        self.board.pinned = pinned;
        self.board.attacked_squares = attacked_squares;

        self.record_moment((chessmove.from, chessmove.to));
        moves
    }

    pub fn undo(&mut self) {
        if let Some(prev_board) = self.prev_board {
            self.history.pop();
            self.board = prev_board;
        }
    }

    // // TODO test everything else first
    // //TODO test
    // pub fn get_strategic_eval(_board: &Board) -> i32 {
    //     //Consider phase of game
    //     //Consider board openness
    //     0
    // }

    // //TODO test
    // pub fn get_positional_eval(_board: &Board) -> i32 {
    //     //Consider #attacked squares
    //     //Consider castle rights
    //     //Consider strong positions by pieces
    //     //Consdier #attacked pieces
    //     //Consider #
    //     0
    // }

    // //TODO test
    // pub fn get_material_eval(board: &Board) -> i32 {
    //     let (white, black) = board.get_material_eval();

    //     if board.side_to_move == WHITE {
    //         white as i32 - black as i32
    //     } else {
    //         black as i32 - white as i32
    //     }
    // }

    // //Always from white perspective
    // //TODO test
    // pub fn get_full_eval(board: &Board) -> i32 {
    //     let material = Game::get_material_eval(board);
    //     let positional = Game::get_positional_eval(board);
    //     let strategic = Game::get_strategic_eval(board);
    //     material + positional + strategic
    // }

    // //TODO test
    // pub fn get_best_move(&mut self, depth: u8) -> Option<(BitBoard, BitBoard)> {
    //     let mut g = self.clone();

    //     g.record_history = false;

    //     let (_eval_score, chessmove) = g.get_best_move_recursive(depth, true, i32::MIN, i32::MAX);

    //     println!("{}", _eval_score);

    //     chessmove
    // }

    // pub fn get_best_move_recursive(
    //     &mut self,
    //     depth: u8,
    //     is_maximizer: bool,
    //     mut alpha: i32,
    //     mut beta: i32,
    // ) -> (i32, Option<(BitBoard, BitBoard)>) {
    //     if depth == 0 {
    //         let eval_score = Game::get_full_eval(&self.board);
    //         if is_maximizer {
    //             return (eval_score, None);
    //         } else {
    //             return (-eval_score, None);
    //         }
    //     }

    //     let mut best_move: Option<(BitBoard, BitBoard)> = None;
    //     let mut best_move_value = match is_maximizer {
    //         true => i32::MIN,
    //         false => i32::MAX,
    //     };
    //     let mut value: i32;

    //     let valid_moves = ChessMove::broken_up(MoveGen::gen_legal_moves(&self.board));

    //     let mut valid_moves_with_eval: Vec<(&ChessMove, i32, Board)> = valid_moves
    //         .iter()
    //         .map(|chessmove| {
    //             // let mut cloned = self.clone();
    //             self.make_move(chessmove);
    //             let board = self.board;
    //             let eval = Game::get_material_eval(&board);
    //             self.undo();
    //             (chessmove, eval, board)
    //         })
    //         .collect();

    //     valid_moves_with_eval.sort_by(|l, r| l.1.partial_cmp(&r.1).unwrap());

    //     for valid_move in valid_moves_with_eval {
    //         self.board = valid_move.2;
    //         value = self
    //             .get_best_move_recursive(depth - 1, !is_maximizer, alpha, beta)
    //             .0;

    //         if is_maximizer {
    //             if value > best_move_value {
    //                 best_move_value = value;
    //                 best_move = Some((valid_move.0.from, valid_move.0.to));
    //             }
    //             alpha = std::cmp::max(alpha, value);
    //         } else {
    //             if value < best_move_value {
    //                 best_move_value = value;
    //                 best_move = Some((valid_move.0.from, valid_move.0.to));
    //             }
    //             beta = std::cmp::min(beta, value);
    //         }

    //         if beta <= alpha {
    //             break;
    //         }
    //     }

    //     (best_move_value, best_move)
    // }

    pub fn randomize_board(&mut self) -> &Self {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        let mut white_pawns = EMPTY;
        let mut taken_squares = EMPTY;

        while white_pawns.popcnt() < 8 {
            let square = SQUARES[rng.gen_range(8..56)];
            white_pawns |= square;
        }

        taken_squares |= white_pawns;

        let mut black_pawns = EMPTY;
        while black_pawns.popcnt() < 8 {
            let square = SQUARES[rng.gen_range(8..56)];
            black_pawns |= square & !taken_squares;
        }

        taken_squares |= black_pawns;

        let mut white_knights = EMPTY;
        while white_knights.popcnt() < 2 {
            let square = SQUARES[rng.gen_range(0..64)];
            white_knights |= square & !taken_squares;
        }

        taken_squares |= white_knights;

        let mut black_knights = EMPTY;
        while black_knights.popcnt() < 2 {
            let square = SQUARES[rng.gen_range(0..64)];
            black_knights |= square & !taken_squares;
        }

        taken_squares |= black_knights;

        let mut white_bishops = EMPTY;
        while white_bishops.popcnt() < 2 {
            let square = SQUARES[rng.gen_range(0..64)];
            white_bishops |= square & !taken_squares;
        }

        taken_squares |= white_bishops;

        let mut black_bishops = EMPTY;
        while black_bishops.popcnt() < 2 {
            let square = SQUARES[rng.gen_range(0..64)];
            black_bishops |= square & !taken_squares;
        }

        taken_squares |= black_bishops;

        let mut white_rooks = EMPTY;
        while white_rooks.popcnt() < 2 {
            let square = SQUARES[rng.gen_range(0..64)];
            white_rooks |= square & !taken_squares;
        }

        taken_squares |= white_rooks;

        let mut black_rooks = EMPTY;
        while black_rooks.popcnt() < 2 {
            let square = SQUARES[rng.gen_range(0..64)];
            black_rooks |= square & !taken_squares;
        }

        taken_squares |= black_rooks;

        let mut white_queens = EMPTY;
        while white_queens.popcnt() < 1 {
            let square = SQUARES[rng.gen_range(0..64)];
            white_queens |= square & !taken_squares;
        }

        taken_squares |= white_queens;

        let mut black_queens = EMPTY;
        while black_queens.popcnt() < 1 {
            let square = SQUARES[rng.gen_range(0..64)];
            black_queens |= square & !taken_squares;
        }

        taken_squares |= black_queens;

        let mut white_kings = EMPTY;
        while white_kings.popcnt() < 1 {
            let square = SQUARES[rng.gen_range(0..64)];
            white_kings |= square & !taken_squares;
        }

        taken_squares |= white_kings;

        let mut black_kings = EMPTY;
        while black_kings.popcnt() < 1 {
            let square = SQUARES[rng.gen_range(0..64)];
            black_kings |= square & !taken_squares;
        }

        self.board = Board::new(BoardParams {
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
            side_to_move: Some(WHITE),
            castle_rights: None,
            en_passant: None,
            half_moves_since_action: None,
            full_moves: None,
        });

        self.record_moment((EMPTY, EMPTY));
        self
    }
}

impl Default for Game {
    fn default() -> Self {
        let b = Board::default();
        Game::new(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    mod make_move {
        use super::*;
        #[test]
        fn it_works() {
            let mut g = Game::default();

            g.make_move(&ChessMove::from_notation("E2", "E4"));
            g.make_move(&ChessMove::from_notation("F7", "F5"));
            g.make_move(&ChessMove::from_notation("D1", "H5"));

            assert_eq!(g.board.get_piece_at(F5_SQUARE), Pieces::BPawn);
            assert_eq!(g.board.checkers, H5_SQUARE);
        }
    }

    mod north_east_attacks {
        use super::*;

        #[test]
        fn it_works() {
            let mut g = Game::default();

            g.make_move(&ChessMove::from_notation("E2", "E4"));
            g.make_move(&ChessMove::from_notation("E7", "E5"));
            g.make_move(&ChessMove::from_notation("D1", "F3"));
            g.make_move(&ChessMove::from_notation("C7", "C5"));

            let valid_queen_moves =
                MoveGen::valid_queen_moves(&g.board, F3_SQUARE, g.board.color_bbs[WHITE]);
            let is_a5_valid = valid_queen_moves & A4_SQUARE;
            println!("{}", is_a5_valid);
        }
    }

    mod get_best_move {
        use super::*;

        #[test]
        fn sandbox() {
            let g =
                Game::from_fen("rn1qkbnr/1pp2ppp/p7/3pp3/4P1b1/3P4/PPP2PPP/RNB1KBNR w kq - 0 1");
            let moves = MoveGen::gen_legal_moves(&g.board);

            g.board.attacked_squares.print_bb("Attackers");
            g.board.print_board();
            for chessmove in moves {
                if chessmove.from == E1_SQUARE {
                    chessmove.from.print_bb("From");
                    chessmove.to.print_bb("To");
                }
            }
        }

        // #[test]
        // fn it_works() {
        //     let mut g =
        //         Game::from_fen("rn1qkbnr/1pp2ppp/p7/3pp3/4P1b1/3P4/PPP2PPP/RNB1KBNR w kq - 0 1");
        //     let best_move = g.get_best_move(6);
        //     if let Some(chessmove) = best_move {
        //         chessmove.0.print_bb("From");
        //         chessmove.1.print_bb("To");
        //     }
        // }

        // #[test]
        // fn it_works2() {
        //     let mut g =
        //         Game::from_fen("r1bqkbnr/1ppp1pp1/p1n4p/4p3/4P3/1P3N2/PBPP1PPP/RN1QKB1R w KQkq - 0 1");
        //     let best_move = g.get_best_move(3);
        //     if let Some(chessmove) = best_move {
        //         chessmove.0.print_bb("From");
        //         chessmove.1.print_bb("To");
        //     }
        // }
    }
}
