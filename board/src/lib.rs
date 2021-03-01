extern crate bitboard;
use crate::bitboard::BitBoard;
extern crate piece;
use piece::Pieces;
extern crate square;
use square::Square;
extern crate constants;
use crate::constants::*;

use std::fmt;

pub struct Board {
    pub piece_bbs: [[BitBoard; 6]; 2],
    pub color_bbs: [BitBoard; 2],
    pub combined_bbs: [BitBoard; 8],
    pub side_to_move: usize,
    pub checkers: BitBoard,
    pub pinned: BitBoard,
    pub en_passant: BitBoard,
    pub castle_rights: BitBoard,
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
        side_to_move: usize,
    ) -> Board {
        let mut piece_bbs = [[EMPTY; 6]; 2];
        let mut combined_bbs = [EMPTY; 8];
        let mut color_bbs = [EMPTY; 2];

        piece_bbs[WHITE][PAWNS_BB] = white_pawns;
        piece_bbs[WHITE][KNIGHTS_BB] = white_knights;
        piece_bbs[WHITE][BISHOPS_BB] = white_bishops;
        piece_bbs[WHITE][ROOKS_BB] = white_rooks;
        piece_bbs[WHITE][QUEENS_BB] = white_queens;
        piece_bbs[WHITE][KINGS_BB] = white_kings;
        piece_bbs[BLACK][PAWNS_BB] = black_pawns;
        piece_bbs[BLACK][KNIGHTS_BB] = black_knights;
        piece_bbs[BLACK][BISHOPS_BB] = black_bishops;
        piece_bbs[BLACK][ROOKS_BB] = black_rooks;
        piece_bbs[BLACK][QUEENS_BB] = black_queens;
        piece_bbs[BLACK][KINGS_BB] = black_kings;

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

        combined_bbs[ALL_PAWNS_BB] = pawns;
        combined_bbs[ALL_KNIGHTS_BB] = knights;
        combined_bbs[ALL_BISHOPS_BB] = bishops;
        combined_bbs[ALL_ROOKS_BB] = rooks;
        combined_bbs[ALL_QUEENS_BB] = queens;
        combined_bbs[ALL_KINGS_BB] = kings;
        color_bbs[WHITE] = white_pieces;
        color_bbs[BLACK] = black_pieces;
        combined_bbs[ALL_PIECES_BB] = pieces;
        combined_bbs[EMPTY_SQUARES_BB] = empty_squares;

        Board {
            piece_bbs,
            color_bbs,
            combined_bbs,
            side_to_move,
            pinned: EMPTY,
            checkers: EMPTY,
            en_passant: EMPTY,
            castle_rights: INITIAL_CASTLE_RIGHTS,
        }
    }

    pub fn to_array(&self) -> [[Square; 8]; 8] {
        let mut board_array: [[Square; 8]; 8] = [[Square::default(); 8]; 8];
        for pos in 0..64 {
            let rank = 7 - (pos / 8);
            let file = pos % 8;
            let square = SQUARES[pos];

            let piece = self.get_piece_at(square);
            board_array[rank][file] = Square::new(square, piece);
        }

        board_array
    }

    pub fn from_fen(fen: &str) -> Board {
        use std::iter::FromIterator;
        let res = Vec::from_iter(fen.split(" ").map(String::from));
        let mut white_pawns: BitBoard = EMPTY;
        let mut white_knights: BitBoard = EMPTY;
        let mut white_bishops: BitBoard = EMPTY;
        let mut white_rooks: BitBoard = EMPTY;
        let mut white_queens: BitBoard = EMPTY;
        let mut white_kings: BitBoard = EMPTY;
        let mut black_pawns: BitBoard = EMPTY;
        let mut black_knights: BitBoard = EMPTY;
        let mut black_bishops: BitBoard = EMPTY;
        let mut black_rooks: BitBoard = EMPTY;
        let mut black_queens: BitBoard = EMPTY;
        let mut black_kings: BitBoard = EMPTY;
        let mut side_to_move: usize = WHITE;
        res.iter().enumerate().for_each(|(part_idx, part)| {
            match part_idx {
                0 => {
                    let rows = Vec::from_iter(part.split("/").map(String::from));
                    rows.iter().enumerate().for_each(|(row_idx, row)| {
                        let mut empty_cols: usize = 0;
                        row.chars().map(String::from).enumerate().for_each(|(col_idx, char)| {
                            let rank = 7 - row_idx;
                            let square_idx = (rank * 8) + col_idx + empty_cols;
                            let square = SQUARES[square_idx];
                            match char.as_str() {
                                "r" => {
                                    black_rooks |= square;
                                },
                                "b" => {
                                    black_bishops |= square;
                                },
                                "n" => {
                                    black_knights |= square;
                                },
                                "q" => {
                                    black_queens |= square;
                                },
                                "k" => {
                                    black_kings |= square;
                                },
                                "p" => {
                                    black_pawns |= square;
                                },
                                "R" => {
                                    white_rooks |= square;
                                },
                                "B" => {
                                    white_bishops |= square;
                                },
                                "N" => {
                                    white_knights |= square;
                                },
                                "Q" => {
                                    white_queens |= square;
                                },
                                "K" => {
                                    white_kings |= square;
                                },
                                "P" => {
                                    white_pawns |= square;
                                },
                                _ => {
                                    let empties = char.parse::<usize>().unwrap();
                                    empty_cols += empties;
                                },
                            }
                        });
                    });
                },
                1 => {
                    if part == "w" {
                        side_to_move = WHITE;
                    } else {
                        side_to_move = BLACK;
                    }
                },
                2 => {
                    //castling: KQkq
                    //TODO
                },
                3 => {
                    //enpassant: e3 / -
                    //TODO
                },
                4 => {
                    //turns since last capture/pawn advance
                    //TODO
                },
                5 => {
                    //# of fullmoves, starts at 1
                    //TODO
                },
                _ => {

                },
            }
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
            side_to_move,
        )
    }

    pub fn to_fen(&self) -> String {
        let mut pieces: Vec<String> = Vec::new();
        self.to_array().iter().for_each(|row| {
            let mut empty_count = 0;
            let mut row_str: Vec<String> = Vec::new();
            row.iter().for_each(|square| {
                let piece = format!("{}", square.piece);
                if piece == "." {
                    empty_count += 1;
                } else {
                    if empty_count > 0 {
                        row_str.push(empty_count.to_string());
                        empty_count = 0;
                    }
                    row_str.push(piece);
                }
            });
            if empty_count > 0 {
                row_str.push(empty_count.to_string());
            }
            pieces.push(row_str.join(""));
        });
        let board_str = pieces.join("/");
        let side_to_move_str: &str;
        if self.side_to_move == WHITE {
            side_to_move_str = "w";
        } else {
            side_to_move_str = "b";
        }
        let castling_rights_str= "KQkq"; //TODO
        let en_passant_str= "-"; //TODO
        let half_moves_since_capture_promotion= "0"; //TODO
        let full_moves= "1"; //TODO

        format!("{} {} {} {} {} {}", board_str, side_to_move_str, castling_rights_str, en_passant_str, half_moves_since_capture_promotion, full_moves)
    }

    pub fn get_piece_at(&self, square: BitBoard) -> Pieces {
        if self.combined_bbs[EMPTY_SQUARES_BB] & square != EMPTY {
            Pieces::Empty
        } else {
            if self.combined_bbs[ALL_PAWNS_BB] & square != EMPTY {
                if self.color_bbs[BLACK] & square != EMPTY {
                    Pieces::BPawn
                } else {
                    Pieces::WPawn
                }
            } else if self.combined_bbs[ALL_KNIGHTS_BB] & square != EMPTY {
                if self.color_bbs[BLACK] & square != EMPTY {
                    Pieces::BKnight
                } else {
                    Pieces::WKnight
                }
            } else if self.combined_bbs[ALL_BISHOPS_BB] & square != EMPTY {
                if self.color_bbs[BLACK] & square != EMPTY {
                    Pieces::BBishop
                } else {
                    Pieces::WBishop
                }
            } else if self.combined_bbs[ALL_ROOKS_BB] & square != EMPTY {
                if self.color_bbs[BLACK] & square != EMPTY {
                    Pieces::BRook
                } else {
                    Pieces::WRook
                }
            } else if self.combined_bbs[ALL_QUEENS_BB] & square != EMPTY {
                if self.color_bbs[BLACK] & square != EMPTY {
                    Pieces::BQueen
                } else {
                    Pieces::WQueen
                }
            } else {
                if self.color_bbs[BLACK] & square != EMPTY {
                    Pieces::BKing
                } else {
                    Pieces::WKing
                }
            }
        }
    }

    pub fn print_board(&self) {
        println!("{}", self);
    }
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
            WHITE,
        )
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
        ",
            self.get_piece_at(A8_SQUARE),
            self.get_piece_at(B8_SQUARE),
            self.get_piece_at(C8_SQUARE),
            self.get_piece_at(D8_SQUARE),
            self.get_piece_at(E8_SQUARE),
            self.get_piece_at(F8_SQUARE),
            self.get_piece_at(G8_SQUARE),
            self.get_piece_at(H8_SQUARE),
            self.get_piece_at(A7_SQUARE),
            self.get_piece_at(B7_SQUARE),
            self.get_piece_at(C7_SQUARE),
            self.get_piece_at(D7_SQUARE),
            self.get_piece_at(E7_SQUARE),
            self.get_piece_at(F7_SQUARE),
            self.get_piece_at(G7_SQUARE),
            self.get_piece_at(H7_SQUARE),
            self.get_piece_at(A6_SQUARE),
            self.get_piece_at(B6_SQUARE),
            self.get_piece_at(C6_SQUARE),
            self.get_piece_at(D6_SQUARE),
            self.get_piece_at(E6_SQUARE),
            self.get_piece_at(F6_SQUARE),
            self.get_piece_at(G6_SQUARE),
            self.get_piece_at(H6_SQUARE),
            self.get_piece_at(A5_SQUARE),
            self.get_piece_at(B5_SQUARE),
            self.get_piece_at(C5_SQUARE),
            self.get_piece_at(D5_SQUARE),
            self.get_piece_at(E5_SQUARE),
            self.get_piece_at(F5_SQUARE),
            self.get_piece_at(G5_SQUARE),
            self.get_piece_at(H5_SQUARE),
            self.get_piece_at(A4_SQUARE),
            self.get_piece_at(B4_SQUARE),
            self.get_piece_at(C4_SQUARE),
            self.get_piece_at(D4_SQUARE),
            self.get_piece_at(E4_SQUARE),
            self.get_piece_at(F4_SQUARE),
            self.get_piece_at(G4_SQUARE),
            self.get_piece_at(H4_SQUARE),
            self.get_piece_at(A3_SQUARE),
            self.get_piece_at(B3_SQUARE),
            self.get_piece_at(C3_SQUARE),
            self.get_piece_at(D3_SQUARE),
            self.get_piece_at(E3_SQUARE),
            self.get_piece_at(F3_SQUARE),
            self.get_piece_at(G3_SQUARE),
            self.get_piece_at(H3_SQUARE),
            self.get_piece_at(A2_SQUARE),
            self.get_piece_at(B2_SQUARE),
            self.get_piece_at(C2_SQUARE),
            self.get_piece_at(D2_SQUARE),
            self.get_piece_at(E2_SQUARE),
            self.get_piece_at(F2_SQUARE),
            self.get_piece_at(G2_SQUARE),
            self.get_piece_at(H2_SQUARE),
            self.get_piece_at(A1_SQUARE),
            self.get_piece_at(B1_SQUARE),
            self.get_piece_at(C1_SQUARE),
            self.get_piece_at(D1_SQUARE),
            self.get_piece_at(E1_SQUARE),
            self.get_piece_at(F1_SQUARE),
            self.get_piece_at(G1_SQUARE),
            self.get_piece_at(H1_SQUARE),
        )
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    mod get_piece_at {
        use super::*;

        #[test]
        fn it_works_with_initial_board() {
            let b = Board::default();
            assert_eq!(b.get_piece_at(E2_SQUARE), Pieces::WPawn,);

            assert_eq!(b.get_piece_at(E7_SQUARE), Pieces::BPawn,);

            assert_eq!(b.get_piece_at(E8_SQUARE), Pieces::BKing,);

            assert_eq!(b.get_piece_at(E1_SQUARE), Pieces::WKing,);
        }
    }

    mod from_fen {
        use super::*;

        #[test]
        fn it_works() {
            let fen_board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let default_board = Board::default();

            fen_board.piece_bbs[WHITE].iter().enumerate().for_each(|(idx, bb)| {
                assert_eq!(&default_board.piece_bbs[WHITE][idx], bb);
            });

            fen_board.piece_bbs[BLACK].iter().enumerate().for_each(|(idx, bb)| {
                assert_eq!(&default_board.piece_bbs[BLACK][idx], bb);
            });
        }
    }

    mod to_fen {
        use super::*;

        #[test]
        fn it_works() {
            let default_board = Board::default();

            assert_eq!(default_board.to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }
    }

    mod default {
        use super::*;

        #[test]
        fn it_works() {
            let b = Board::default();
            assert_eq!(b.piece_bbs[WHITE][PAWNS_BB], INITIAL_WHITE_PAWNS);
            assert_eq!(b.piece_bbs[WHITE][KNIGHTS_BB], INITIAL_WHITE_KNIGHTS);
            assert_eq!(b.piece_bbs[WHITE][BISHOPS_BB], INITIAL_WHITE_BISHOPS);
            assert_eq!(b.piece_bbs[WHITE][ROOKS_BB], INITIAL_WHITE_ROOKS);
            assert_eq!(b.piece_bbs[WHITE][QUEENS_BB], INITIAL_WHITE_QUEENS);
            assert_eq!(b.piece_bbs[WHITE][KINGS_BB], INITIAL_WHITE_KINGS);
            assert_eq!(b.piece_bbs[BLACK][PAWNS_BB], INITIAL_BLACK_PAWNS);
            assert_eq!(b.piece_bbs[BLACK][KNIGHTS_BB], INITIAL_BLACK_KNIGHTS);
            assert_eq!(b.piece_bbs[BLACK][BISHOPS_BB], INITIAL_BLACK_BISHOPS);
            assert_eq!(b.piece_bbs[BLACK][ROOKS_BB], INITIAL_BLACK_ROOKS);
            assert_eq!(b.piece_bbs[BLACK][QUEENS_BB], INITIAL_BLACK_QUEENS);
            assert_eq!(b.piece_bbs[BLACK][KINGS_BB], INITIAL_BLACK_KINGS);
        }
    }
}
