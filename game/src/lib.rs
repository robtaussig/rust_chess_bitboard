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
use moment::*;

pub struct Game {
    pub board: Board,
    pub history: Vec<Moment>,
    pub future: Vec<Moment>,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::default();
        Game {
            board,
            history: vec![Moment::new(board.to_fen(), (EMPTY, EMPTY))],
            future: Vec::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        Game {
            board: Board::from_fen(fen),
            history: vec![Moment::new(fen.into(), (EMPTY, EMPTY))],
            future: Vec::new(),
        }
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
        let board_fen = self.board.to_fen();
        self.history.push(Moment::new(board_fen, last_move));
        self.future = Vec::new();
    }

    pub fn calculate_derived_bitboards(board: &Board) -> (BitBoard, BitBoard, BitBoard) {
        let (checkers, pinned) = MoveGen::find_checkers_and_pinned_pieces(&board);
        let attacked_squares = MoveGen::find_attacked_squares(&board);
        (checkers, pinned, attacked_squares)
    }

    //TODO test
    //Returns list of all moved pieces, including castling rook
    pub fn make_move(&mut self, chessmove: &ChessMove) -> Vec<(BitBoard, BitBoard)> {
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
            _ => ()
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
            _ => ()
        }

        if let Some(promotion) = chessmove.promotion {
            self.board.piece_bbs[moving_piece.color_bb_index()][moving_piece.piece_by_color_bb_index()] ^= chessmove.to;
            self.board.combined_bbs[moving_piece.combined_color_bb_index()] ^= chessmove.to;

            self.board.piece_bbs[promotion.color_bb_index()][promotion.piece_by_color_bb_index()] |= chessmove.to;
            self.board.combined_bbs[promotion.combined_color_bb_index()] |= chessmove.to;
            moves.push((EMPTY, chessmove.to));
        }

        self.board.side_to_move ^= 1;

        let (checkers, pinned, attacked_squares) = Game::calculate_derived_bitboards(&self.board);
        self.board.checkers = checkers;
        self.board.pinned = pinned;
        self.board.attacked_squares = attacked_squares;
        self.record_moment((chessmove.from, chessmove.to));
        moves
    }

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
        
        self.board = Board::new_from_pieces(
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
            WHITE,
        );

        self.record_moment((EMPTY, EMPTY));
        self
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
            let mut g = Game::new();

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
            let mut g = Game::new();

            g.make_move(&ChessMove::from_notation("E2", "E4"));
            g.make_move(&ChessMove::from_notation("E7", "E5"));
            g.make_move(&ChessMove::from_notation("D1", "F3"));
            g.make_move(&ChessMove::from_notation("C7", "C5"));
            
            let valid_queen_moves = MoveGen::valid_queen_moves(&g.board, F3_SQUARE, g.board.color_bbs[WHITE]);
            let is_a5_valid = valid_queen_moves & A4_SQUARE;
            println!("{}", is_a5_valid);
        }
    }
}
