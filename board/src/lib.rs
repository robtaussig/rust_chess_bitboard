extern crate bitboard;
use crate::bitboard::*;
extern crate piece;
use piece::Pieces;
extern crate square;
use square::Square;
extern crate constants;
use crate::constants::*;

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board {
    pub piece_bbs: [[BitBoard; 6]; 2],
    pub color_bbs: [BitBoard; 2],
    pub combined_bbs: [BitBoard; 8],
    pub side_to_move: usize,
    pub checkers: BitBoard,
    pub pinned: BitBoard,
    pub en_passant: BitBoard,
    pub castle_rights: BitBoard,
    pub half_moves_since_action: u8,
    pub full_moves: u16,
    pub attacked_squares: BitBoard,
}

pub struct BoardParams {
    pub white_pawns: BitBoard,
    pub white_knights: BitBoard,
    pub white_bishops: BitBoard,
    pub white_rooks: BitBoard,
    pub white_queens: BitBoard,
    pub white_kings: BitBoard,
    pub black_pawns: BitBoard,
    pub black_knights: BitBoard,
    pub black_bishops: BitBoard,
    pub black_rooks: BitBoard,
    pub black_queens: BitBoard,
    pub black_kings: BitBoard,
    pub side_to_move: Option<usize>,
    pub castle_rights: Option<BitBoard>,
    pub en_passant: Option<BitBoard>,
    pub half_moves_since_action: Option<u8>,
    pub full_moves: Option<u16>,
}

impl Default for BoardParams {
    fn default() -> Self {
        BoardParams {
            white_pawns: INITIAL_WHITE_PAWNS,
            white_knights: INITIAL_WHITE_KNIGHTS,
            white_bishops: INITIAL_WHITE_BISHOPS,
            white_rooks: INITIAL_WHITE_ROOKS,
            white_queens: INITIAL_WHITE_QUEENS,
            white_kings: INITIAL_WHITE_KINGS,
            black_pawns: INITIAL_BLACK_PAWNS,
            black_knights: INITIAL_BLACK_KNIGHTS,
            black_bishops: INITIAL_BLACK_BISHOPS,
            black_rooks: INITIAL_BLACK_ROOKS,
            black_queens: INITIAL_BLACK_QUEENS,
            black_kings: INITIAL_BLACK_KINGS,
            side_to_move: Some(WHITE),
            castle_rights: Some(INITIAL_CASTLE_RIGHTS),
            en_passant: Some(EMPTY),
            half_moves_since_action: Some(0),
            full_moves: Some(1),
        }
    }
}

impl Board {
    pub fn new(params: BoardParams) -> Board {
        let mut piece_bbs = [[EMPTY; 6]; 2];
        let mut combined_bbs = [EMPTY; 8];
        let mut color_bbs = [EMPTY; 2];
        let mut castle_rights = params.castle_rights.unwrap_or(EMPTY);

        piece_bbs[WHITE][PAWNS_BB] = params.white_pawns;
        piece_bbs[WHITE][KNIGHTS_BB] = params.white_knights;
        piece_bbs[WHITE][BISHOPS_BB] = params.white_bishops;
        piece_bbs[WHITE][ROOKS_BB] = params.white_rooks;
        piece_bbs[WHITE][QUEENS_BB] = params.white_queens;
        piece_bbs[WHITE][KINGS_BB] = params.white_kings;
        piece_bbs[BLACK][PAWNS_BB] = params.black_pawns;
        piece_bbs[BLACK][KNIGHTS_BB] = params.black_knights;
        piece_bbs[BLACK][BISHOPS_BB] = params.black_bishops;
        piece_bbs[BLACK][ROOKS_BB] = params.black_rooks;
        piece_bbs[BLACK][QUEENS_BB] = params.black_queens;
        piece_bbs[BLACK][KINGS_BB] = params.black_kings;

        let white_pieces = params.white_pawns
            | params.white_knights
            | params.white_bishops
            | params.white_rooks
            | params.white_queens
            | params.white_kings;

        let black_pieces = params.black_pawns
            | params.black_knights
            | params.black_bishops
            | params.black_rooks
            | params.black_queens
            | params.black_kings;

        let pieces = white_pieces | black_pieces;
        let empty_squares = !pieces;

        let pawns = params.white_pawns | params.black_pawns;
        let knights = params.white_knights | params.black_knights;
        let bishops = params.white_bishops | params.black_bishops;
        let rooks = params.white_rooks | params.black_rooks;
        let queens = params.white_queens | params.black_queens;
        let kings = params.white_kings | params.black_kings;

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

        if (params.white_rooks & A1_SQUARE).is_empty() {
            castle_rights &= !C1_SQUARE;
        }
        if (params.white_rooks & H1_SQUARE).is_empty() {
            castle_rights &= !G1_SQUARE;
        }
        if (params.black_rooks & A8_SQUARE).is_empty() {
            castle_rights &= !C8_SQUARE;
        }
        if (params.black_rooks & H8_SQUARE).is_empty() {
            castle_rights &= !G8_SQUARE;
        }

        Board {
            piece_bbs,
            color_bbs,
            combined_bbs,
            side_to_move: params.side_to_move.unwrap_or(WHITE),
            pinned: EMPTY,
            checkers: EMPTY,
            en_passant: params.en_passant.unwrap_or(EMPTY),
            castle_rights,
            half_moves_since_action: params.half_moves_since_action.unwrap_or(0),
            full_moves: params.full_moves.unwrap_or(1),
            attacked_squares: EMPTY,
        }
    }

    pub fn to_array(&self) -> [[Square; 8]; 8] {
        let mut board_array: [[Square; 8]; 8] = [[Square::default(); 8]; 8];
        for (pos, square) in SQUARES.iter().enumerate() {
            let rank = 7 - (pos / 8);
            let file = pos % 8;

            let piece = self.get_piece_at(*square);
            board_array[rank][file] = Square::new(*square, piece);
        }

        board_array
    }

    pub fn move_piece(&mut self, from: BitBoard, to: BitBoard) -> &mut Self {
        let moving_piece = self.get_piece_at(from);
        let target_piece = self.get_piece_at(to);
        let combined_move = from | to;

        let moving_piece_color_bb_index = moving_piece.color_bb_index();
        let moving_piece_by_color_bb_index = moving_piece.piece_by_color_bb_index();
        let moving_piece_combined_bb_index = moving_piece.combined_color_bb_index();

        let target_color_bb_index = target_piece.color_bb_index();
        let target_piece_color_bb_index = target_piece.piece_by_color_bb_index();
        let target_piece_combined_bb_index = target_piece.combined_color_bb_index();

        if target_color_bb_index != EMPTY_SQUARES_BB {
            self.piece_bbs[target_color_bb_index][target_piece_color_bb_index] ^= to;
            self.color_bbs[target_color_bb_index] ^= to;
            self.combined_bbs[target_piece_combined_bb_index] ^= to;
        }

        self.piece_bbs[moving_piece_color_bb_index][moving_piece_by_color_bb_index] ^=
            combined_move;
        self.color_bbs[moving_piece_color_bb_index] ^= combined_move;
        self.combined_bbs[moving_piece_combined_bb_index] ^= combined_move;

        self.combined_bbs[EMPTY_SQUARES_BB] |= from;
        self.combined_bbs[EMPTY_SQUARES_BB] &= !to;
        self.combined_bbs[ALL_PIECES_BB] ^= from;
        self.combined_bbs[ALL_PIECES_BB] |= to;

        self
    }

    pub fn from_fen(fen: &str) -> Board {
        let res: Vec<String> = fen.split(' ').map(String::from).collect();
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
        let mut en_passant: BitBoard = EMPTY;
        let mut castle_rights: BitBoard = EMPTY;
        let mut half_moves_since_action: u8 = 0;
        let mut full_moves: u16 = 0;
        res.iter()
            .enumerate()
            .for_each(|(part_idx, part)| match part_idx {
                0 => {
                    let rows: Vec<String> = part.split('/').map(String::from).collect();
                    rows.iter().enumerate().for_each(|(row_idx, row)| {
                        let mut empty_cols: usize = 0;
                        row.chars().enumerate().for_each(|(col_idx, char)| {
                            let rank = 7 - row_idx;
                            let square_idx = (rank * 8) + col_idx + empty_cols;
                            let square = SQUARES[square_idx];

                            match char {
                                'r' => {
                                    black_rooks |= square;
                                }
                                'b' => {
                                    black_bishops |= square;
                                }
                                'n' => {
                                    black_knights |= square;
                                }
                                'q' => {
                                    black_queens |= square;
                                }
                                'k' => {
                                    black_kings |= square;
                                }
                                'p' => {
                                    black_pawns |= square;
                                }
                                'R' => {
                                    white_rooks |= square;
                                }
                                'B' => {
                                    white_bishops |= square;
                                }
                                'N' => {
                                    white_knights |= square;
                                }
                                'Q' => {
                                    white_queens |= square;
                                }
                                'K' => {
                                    white_kings |= square;
                                }
                                'P' => {
                                    white_pawns |= square;
                                }
                                _ => {
                                    if let Some(empties) = char.to_digit(10) {
                                        empty_cols += empties as usize - 1;
                                    } else {
                                        println!("{}", char);
                                    }
                                }
                            }
                        });
                    });
                }
                1 => {
                    if part == "w" {
                        side_to_move = WHITE;
                    } else {
                        side_to_move = BLACK;
                    }
                }
                2 => {
                    castle_rights = part.chars().fold(castle_rights, |acc, piece| {
                        acc | match piece.to_string().as_str() {
                            "K" => G1_SQUARE,
                            "Q" => C1_SQUARE,
                            "k" => G8_SQUARE,
                            "q" => C8_SQUARE,
                            _ => EMPTY,
                        }
                    });
                }
                3 => {
                    en_passant = match part.as_str() {
                        "-" => EMPTY,
                        _ => Board::square_from_notation(part),
                    };
                }
                4 => {
                    half_moves_since_action = part.parse::<u8>().unwrap();
                }
                5 => {
                    full_moves = part.parse::<u16>().unwrap();
                }
                _ => {}
            });

        Board::new(BoardParams {
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
            side_to_move: Some(side_to_move),
            castle_rights: Some(castle_rights),
            en_passant: Some(en_passant),
            half_moves_since_action: Some(half_moves_since_action),
            full_moves: Some(full_moves),
        })
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
        let mut castle_rights: Vec<&str> = Vec::new();

        if (self.castle_rights & G1_SQUARE).is_not_empty() {
            castle_rights.push("K");
        }

        if (self.castle_rights & C1_SQUARE).is_not_empty() {
            castle_rights.push("Q");
        }

        if (self.castle_rights & G8_SQUARE).is_not_empty() {
            castle_rights.push("k");
        }

        if (self.castle_rights & C8_SQUARE).is_not_empty() {
            castle_rights.push("q");
        }

        let castle_rights_str = castle_rights.join("");
        let en_passant_str: String;
        if self.en_passant.is_empty() {
            en_passant_str = String::from("-");
        } else {
            en_passant_str = Board::square_to_notation(self.en_passant);
        }

        let half_moves_since_capture_promotion = self.half_moves_since_action.to_string();
        let full_moves = self.full_moves.to_string();

        format!(
            "{} {} {} {} {} {}",
            board_str,
            side_to_move_str,
            castle_rights_str,
            en_passant_str,
            half_moves_since_capture_promotion,
            full_moves
        )
    }

    pub fn get_piece_at(&self, square: BitBoard) -> Pieces {
        if (self.combined_bbs[EMPTY_SQUARES_BB] & square).is_not_empty() {
            Pieces::Empty
        } else if (self.combined_bbs[ALL_PAWNS_BB] & square).is_not_empty() {
            if (self.color_bbs[BLACK] & square).is_not_empty() {
                Pieces::BPawn
            } else {
                Pieces::WPawn
            }
        } else if (self.combined_bbs[ALL_KNIGHTS_BB] & square).is_not_empty() {
            if (self.color_bbs[BLACK] & square).is_not_empty() {
                Pieces::BKnight
            } else {
                Pieces::WKnight
            }
        } else if (self.combined_bbs[ALL_BISHOPS_BB] & square).is_not_empty() {
            if (self.color_bbs[BLACK] & square).is_not_empty() {
                Pieces::BBishop
            } else {
                Pieces::WBishop
            }
        } else if (self.combined_bbs[ALL_ROOKS_BB] & square).is_not_empty() {
            if (self.color_bbs[BLACK] & square).is_not_empty() {
                Pieces::BRook
            } else {
                Pieces::WRook
            }
        } else if (self.combined_bbs[ALL_QUEENS_BB] & square).is_not_empty() {
            if (self.color_bbs[BLACK] & square).is_not_empty() {
                Pieces::BQueen
            } else {
                Pieces::WQueen
            }
        } else if (self.color_bbs[BLACK] & square).is_not_empty() {
            Pieces::BKing
        } else {
            Pieces::WKing
        }
    }

    pub fn square_to_notation(square: BitBoard) -> String {
        let (row, col) = (7 - square.row(), square.col());
        let rank = (row + 1).to_string();
        let file = match col {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => "",
        };
        format!("{}{}", file, rank)
    }

    pub fn square_from_notation(notation: &str) -> BitBoard {
        let file: usize = match notation.chars().next().unwrap().to_string().as_str() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => 0,
        };

        let rank = notation
            .chars()
            .nth(1)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap()
            - 1;

        let square_pos = rank * 8 + file;
        SQUARES[square_pos]
    }

    pub fn print_board(&self) {
        println!("{}", self);
    }

    pub fn current_pieces(&self) -> [BitBoard; 6] {
        self.piece_bbs[self.side_to_move]
    }

    pub fn other_pieces(&self) -> [BitBoard; 6] {
        match self.side_to_move {
            WHITE => self.piece_bbs[BLACK],
            BLACK => self.piece_bbs[WHITE],
            _ => panic!("Invalid side to move"),
        }
    }

    pub fn current_pieces_by_color(&self) -> BitBoard {
        self.color_bbs[self.side_to_move]
    }

    pub fn other_pieces_by_color(&self) -> BitBoard {
        match self.side_to_move {
            WHITE => self.color_bbs[BLACK],
            BLACK => self.color_bbs[WHITE],
            _ => panic!("Invalid side to move"),
        }
    }

    pub fn switch_side_to_move(&mut self) {
        self.side_to_move ^= 1;
    }

    pub fn get_material_eval_by_color(&self, color: usize) -> u32 {
        let (pawns, bishops, knights, rooks, queens) = (
            self.piece_bbs[color][PAWNS_BB].popcnt(),
            self.piece_bbs[color][BISHOPS_BB].popcnt(),
            self.piece_bbs[color][KNIGHTS_BB].popcnt(),
            self.piece_bbs[color][ROOKS_BB].popcnt(),
            self.piece_bbs[color][QUEENS_BB].popcnt(),
        );

        pawns * PAWN_VALUE
            + bishops * BISHOP_VALUE
            + knights * KNIGHT_VALUE
            + rooks * ROOK_VALUE
            + queens * QUEEN_VALUE
    }

    //TODO test
    pub fn get_material_eval(&self) -> (u32, u32) {
        (
            self.get_material_eval_by_color(WHITE),
            self.get_material_eval_by_color(BLACK),
        )
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new(BoardParams::default())
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
        fn it_works_with_initial_board() {
            let fen_board =
                Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let default_board = Board::default();

            fen_board.piece_bbs[WHITE]
                .iter()
                .enumerate()
                .for_each(|(idx, bb)| {
                    assert_eq!(&default_board.piece_bbs[WHITE][idx], bb);
                });

            fen_board.piece_bbs[BLACK]
                .iter()
                .enumerate()
                .for_each(|(idx, bb)| {
                    assert_eq!(&default_board.piece_bbs[BLACK][idx], bb);
                });
        }

        #[test]
        fn it_works_after_move_1_e4() {
            let fen_board =
                Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
            let white_pawns = fen_board.piece_bbs[WHITE][PAWNS_BB];
            assert_eq!(white_pawns, RANK_2 ^ E2_SQUARE | E4_SQUARE);
        }
    }

    mod to_fen {
        use super::*;

        #[test]
        fn it_works() {
            let default_board = Board::default();

            assert_eq!(
                default_board.to_fen(),
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            );
        }
    }

    mod square_to_notation {
        use super::*;

        #[test]
        fn it_works() {
            assert_eq!("h8", Board::square_to_notation(H8_SQUARE));
            assert_eq!("h2", Board::square_to_notation(H2_SQUARE));
            assert_eq!("a1", Board::square_to_notation(A1_SQUARE));
            assert_eq!("a8", Board::square_to_notation(A8_SQUARE));
            assert_eq!("c5", Board::square_to_notation(C5_SQUARE));
            assert_eq!("f3", Board::square_to_notation(F3_SQUARE));
        }
    }

    mod square_from_notation {
        use super::*;

        #[test]
        fn it_works() {
            assert_eq!(Board::square_from_notation("h8"), H8_SQUARE);
            assert_eq!(Board::square_from_notation("h2"), H2_SQUARE);
            assert_eq!(Board::square_from_notation("a1"), A1_SQUARE);
            assert_eq!(Board::square_from_notation("a8"), A8_SQUARE);
            assert_eq!(Board::square_from_notation("c5"), C5_SQUARE);
            assert_eq!(Board::square_from_notation("f3"), F3_SQUARE);
        }
    }

    mod eval {
        use super::*;

        #[test]
        fn it_evaluates_material_correctly() {
            let b =
                Board::from_fen("b7/1PPP1pq1/1npn1pNP/R1P1p3/Pr5p/1pp3kB/P1R2N1p/3KB3 w  - 0 1");
            let adv = b.get_material_eval();
            assert_eq!(adv, (3000, 3150));
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
