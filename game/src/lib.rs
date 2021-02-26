extern crate board;
use crate::board::{Board};
extern crate piece;
use crate::piece::{Pieces};
extern crate movegen;
use crate::movegen::{MoveGen};
extern crate chessmove;
use crate::chessmove::{ChessMove};
extern crate constants;
use crate::constants::*;

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board:Board::default(),
        }
    }

    //TODO test
    //TODO handle promotion
    //TODO test for castling
    //TODO test for en passant
    pub fn make_move(&mut self, chessmove: &ChessMove) -> &mut Self {
        self.board.en_passant = EMPTY;

        let moving_piece = self.board.get_piece_at(chessmove.from);
        let target_piece = self.board.get_piece_at(chessmove.to);
        let combined_move = chessmove.from | chessmove.to;

        self.board.combined_bbs[EMPTY_SQUARES_BB] |= chessmove.from;
        self.board.combined_bbs[EMPTY_SQUARES_BB] &= !chessmove.to;
        self.board.combined_bbs[ALL_PIECES_BB] ^= chessmove.from;
        self.board.combined_bbs[ALL_PIECES_BB] |= chessmove.to;

        match moving_piece {
            Pieces::WPawn => {
                //TODO Update enpassant
                self.board.piece_bbs[WHITE][PAWNS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_PAWNS_BB] &= !chessmove.from;
                self.board.combined_bbs[ALL_PAWNS_BB] |= chessmove.to;
            },
            Pieces::BPawn => {
                //TODO Update enpassant
                self.board.piece_bbs[BLACK][PAWNS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_PAWNS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_PAWNS_BB] |= chessmove.to;
            },
            Pieces::WKnight => {
                self.board.piece_bbs[WHITE][KNIGHTS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_KNIGHTS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_KNIGHTS_BB] |= chessmove.to;
            },
            Pieces::BKnight => {
                self.board.piece_bbs[BLACK][KNIGHTS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_KNIGHTS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_KNIGHTS_BB] |= chessmove.to;
            },
            Pieces::WBishop => {
                self.board.piece_bbs[WHITE][BISHOPS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_BISHOPS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_BISHOPS_BB] |= chessmove.to;
            },
            Pieces::BBishop => {
                self.board.piece_bbs[BLACK][BISHOPS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_BISHOPS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_BISHOPS_BB] |= chessmove.to;
            },
            Pieces::WRook => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][ROOKS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_ROOKS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_ROOKS_BB] |= chessmove.to;
            },
            Pieces::BRook => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][ROOKS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_ROOKS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_ROOKS_BB] |= chessmove.to;
            },
            Pieces::WQueen => {
                self.board.piece_bbs[WHITE][QUEENS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_QUEENS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_QUEENS_BB] |= chessmove.to;
            },
            Pieces::BQueen => {
                self.board.piece_bbs[BLACK][QUEENS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_QUEENS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_QUEENS_BB] |= chessmove.to;
            },
            Pieces::WKing => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][KINGS_BB] ^= combined_move;
                self.board.color_bbs[WHITE] ^= combined_move;
                self.board.combined_bbs[ALL_KINGS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_KINGS_BB] |= chessmove.to;
            },
            Pieces::BKing => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][KINGS_BB] ^= combined_move;
                self.board.color_bbs[BLACK] ^= combined_move;
                self.board.combined_bbs[ALL_KINGS_BB] ^= chessmove.from;
                self.board.combined_bbs[ALL_KINGS_BB] |= chessmove.to;
            },
            _ => {
                //TODO Handle error
            }
        }

        match target_piece {
            Pieces::WPawn => {
                //TODO Update enpassant
                self.board.piece_bbs[WHITE][PAWNS_BB] ^= chessmove.to;
                self.board.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BPawn => {
                //TODO Update enpassant
                self.board.piece_bbs[BLACK][PAWNS_BB] ^= chessmove.to;                
                self.board.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WKnight => {
                self.board.piece_bbs[WHITE][KNIGHTS_BB] ^= chessmove.to;                
                self.board.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BKnight => {
                self.board.piece_bbs[BLACK][KNIGHTS_BB] ^= chessmove.to;                
                self.board.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WBishop => {
                self.board.piece_bbs[WHITE][BISHOPS_BB] ^= chessmove.to;                
                self.board.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BBishop => {
                self.board.piece_bbs[BLACK][BISHOPS_BB] ^= chessmove.to;                
                self.board.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WRook => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][ROOKS_BB] ^= chessmove.to;                
                self.board.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BRook => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][ROOKS_BB] ^= chessmove.to;                
                self.board.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WQueen => {
                self.board.piece_bbs[WHITE][QUEENS_BB] ^= chessmove.to;                
                self.board.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BQueen => {
                self.board.piece_bbs[BLACK][QUEENS_BB] ^= chessmove.to;                
                self.board.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WKing => {
                //TODO Update castling rights
                self.board.piece_bbs[WHITE][KINGS_BB] ^= chessmove.to;                
                self.board.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BKing => {
                //TODO Update castling rights
                self.board.piece_bbs[BLACK][KINGS_BB] ^= chessmove.to;                
                self.board.color_bbs[BLACK] ^= chessmove.to;
            },
            _ => {
                //TODO Handle error
            }
        }

        let (checkers, pinned) = MoveGen::find_checkers_and_pinners(&self.board);

        self.board.checkers = checkers;
        self.board.pinned = pinned;
        if self.board.side_to_move == WHITE {
            self.board.side_to_move = BLACK;
        } else {
            self.board.side_to_move = BLACK;
        }

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

            g
                .make_move(&ChessMove::from_notation("E2", "E4"))
                .make_move(&ChessMove::from_notation("F7", "F5"))
                .make_move(&ChessMove::from_notation("D1", "H5"));

            assert_eq!(g.board.get_piece_at(F5_SQUARE), Pieces::BPawn);
            assert_eq!(g.board.checkers, H5_SQUARE);
        }
    }
}
