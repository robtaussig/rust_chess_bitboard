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

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::default(),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        Game {
            board: Board::from_fen(fen),
        }
    }

    //TODO test
    //TODO handle promotion
    //TODO test for castling
    //TODO test for en passant
    pub fn make_move(&mut self, chessmove: &ChessMove) -> &mut Self {
        let prev_en_passant = self.board.en_passant;
        self.board.en_passant = EMPTY;

        let moving_piece = self.board.get_piece_at(chessmove.from);
        let target_piece = self.board.get_piece_at(chessmove.to);
        let combined_move = chessmove.from | chessmove.to;

        self.board.combined_bbs[EMPTY_SQUARES_BB] |= chessmove.from;
        self.board.combined_bbs[EMPTY_SQUARES_BB] &= !chessmove.to;
        self.board.combined_bbs[ALL_PIECES_BB] ^= chessmove.from;
        self.board.combined_bbs[ALL_PIECES_BB] |= chessmove.to;

        match target_piece {
            Pieces::WPawn => {
                self.board.piece_bbs[WHITE][PAWNS_BB] ^= chessmove.to;
                self.board.color_bbs[WHITE] ^= chessmove.to;
                self.board.combined_bbs[ALL_PAWNS_BB] ^= chessmove.to;
            }
            Pieces::BPawn => {
                self.board.piece_bbs[BLACK][PAWNS_BB] ^= chessmove.to;
                self.board.color_bbs[BLACK] ^= chessmove.to;
                self.board.combined_bbs[ALL_PAWNS_BB] ^= chessmove.to;
            }
            Pieces::WKnight => {
                self.board.piece_bbs[WHITE][KNIGHTS_BB] ^= chessmove.to;
                self.board.color_bbs[WHITE] ^= chessmove.to;
                self.board.combined_bbs[ALL_KNIGHTS_BB] ^= chessmove.to;
            }
            Pieces::BKnight => {
                self.board.piece_bbs[BLACK][KNIGHTS_BB] ^= chessmove.to;
                self.board.color_bbs[BLACK] ^= chessmove.to;
                self.board.combined_bbs[ALL_KNIGHTS_BB] ^= chessmove.to;
            }
            Pieces::WBishop => {
                self.board.piece_bbs[WHITE][BISHOPS_BB] ^= chessmove.to;
                self.board.color_bbs[WHITE] ^= chessmove.to;
                self.board.combined_bbs[ALL_BISHOPS_BB] ^= chessmove.to;
            }
            Pieces::BBishop => {
                self.board.piece_bbs[BLACK][BISHOPS_BB] ^= chessmove.to;
                self.board.color_bbs[BLACK] ^= chessmove.to;
                self.board.combined_bbs[ALL_BISHOPS_BB] ^= chessmove.to;
            }
            Pieces::WRook => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][ROOKS_BB] ^= chessmove.to;
                self.board.color_bbs[WHITE] ^= chessmove.to;
                self.board.combined_bbs[ALL_ROOKS_BB] ^= chessmove.to;
            }
            Pieces::BRook => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][ROOKS_BB] ^= chessmove.to;
                self.board.color_bbs[BLACK] ^= chessmove.to;
                self.board.combined_bbs[ALL_ROOKS_BB] ^= chessmove.to;
            }
            Pieces::WQueen => {
                self.board.piece_bbs[WHITE][QUEENS_BB] ^= chessmove.to;
                self.board.color_bbs[WHITE] ^= chessmove.to;
                self.board.combined_bbs[ALL_QUEENS_BB] ^= chessmove.to;
            }
            Pieces::BQueen => {
                self.board.piece_bbs[BLACK][QUEENS_BB] ^= chessmove.to;
                self.board.color_bbs[BLACK] ^= chessmove.to;
                self.board.combined_bbs[ALL_QUEENS_BB] ^= chessmove.to;
            }
            Pieces::WKing => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][KINGS_BB] ^= chessmove.to;
                self.board.color_bbs[WHITE] ^= chessmove.to;
                self.board.combined_bbs[ALL_KINGS_BB] ^= chessmove.to;
            }
            Pieces::BKing => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][KINGS_BB] ^= chessmove.to;
                self.board.color_bbs[BLACK] ^= chessmove.to;
                self.board.combined_bbs[ALL_KINGS_BB] ^= chessmove.to;
            }
            _ => {
                //TODO Handle error
            }
        }

        match moving_piece {
            Pieces::WPawn => {
                self.board.piece_bbs[WHITE][PAWNS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_PAWNS_BB] ^= combined_move;

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
                self.board.piece_bbs[BLACK][PAWNS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_PAWNS_BB] ^= combined_move;

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
            Pieces::WKnight => {
                self.board.piece_bbs[WHITE][KNIGHTS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_KNIGHTS_BB] ^= combined_move;
            }
            Pieces::BKnight => {
                self.board.piece_bbs[BLACK][KNIGHTS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_KNIGHTS_BB] ^= combined_move;
            }
            Pieces::WBishop => {
                self.board.piece_bbs[WHITE][BISHOPS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_BISHOPS_BB] ^= combined_move;
            }
            Pieces::BBishop => {
                self.board.piece_bbs[BLACK][BISHOPS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_BISHOPS_BB] ^= combined_move;
            }
            Pieces::WRook => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][ROOKS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_ROOKS_BB] ^= combined_move;
            }
            Pieces::BRook => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][ROOKS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_ROOKS_BB] ^= combined_move;
            }
            Pieces::WQueen => {
                self.board.piece_bbs[WHITE][QUEENS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_QUEENS_BB] ^= combined_move;
            }
            Pieces::BQueen => {
                self.board.piece_bbs[BLACK][QUEENS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_QUEENS_BB] ^= combined_move;
            }
            Pieces::WKing => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][KINGS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_KINGS_BB] ^= combined_move;
            }
            Pieces::BKing => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][KINGS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_KINGS_BB] ^= combined_move;
            }
            _ => {
                //TODO Handle error
            }
        }

        self.board.side_to_move ^= 1;

        let (checkers, pinned) = MoveGen::find_checkers_and_pinners(&self.board);
        self.board.checkers = checkers;
        self.board.pinned = pinned;
        self
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

            g.make_move(&ChessMove::from_notation("E2", "E4"))
                .make_move(&ChessMove::from_notation("F7", "F5"))
                .make_move(&ChessMove::from_notation("D1", "H5"));

            assert_eq!(g.board.get_piece_at(F5_SQUARE), Pieces::BPawn);
            assert_eq!(g.board.checkers, H5_SQUARE);
        }
    }

    mod north_east_attacks {
        use super::*;

        #[test]
        fn it_works() {
            let mut g = Game::new();

            g.make_move(&ChessMove::from_notation("E2", "E4"))
                .make_move(&ChessMove::from_notation("E7", "E5"))
                .make_move(&ChessMove::from_notation("D1", "F3"))
                .make_move(&ChessMove::from_notation("C7", "C5"));
            
            let valid_queen_moves = MoveGen::valid_queen_moves(&g.board, F3_SQUARE, g.board.color_bbs[WHITE]);
            let is_a5_valid = valid_queen_moves & A4_SQUARE;
            println!("{}", is_a5_valid);
        }
    }
}
