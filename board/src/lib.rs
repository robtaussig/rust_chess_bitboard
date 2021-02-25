mod constants;
use constants::*;
mod bitboard;
use crate::bitboard::{BitBoard, EMPTY};
mod piece;
use piece::Pieces;
mod chessmove;
use crate::chessmove::{ChessMove};
use std::{fmt};

type BoardArray = [[Pieces; 8]; 8];

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
        write!(f,"
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
        let mut piece_bbs= [[EMPTY; 6]; 2];
        let mut combined_bbs= [EMPTY; 8];
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

        let mut b = Board {
            piece_bbs,
            color_bbs,
            combined_bbs,
            side_to_move,
            pinned: EMPTY,
            checkers: EMPTY,
            en_passant: EMPTY,
            castle_rights: INITIAL_CASTLE_RIGHTS,
        };

        let (checkers, pinned) = Board::find_checkers_and_pinners(&b);
        b.checkers = checkers;
        b.pinned = pinned;

        b
    }

    pub fn to_array(&self) -> BoardArray {
        let mut board_array: BoardArray = [[Pieces::Empty; 8]; 8];
        for pos in 0..64 {
            let rank = pos / 8;
            let file = pos % 8;
            let square = SQUARES[pos];

            let piece = self.get_piece_at(square);
            board_array[rank][file] = piece;
        }

        board_array
    }

    //TODO test
    pub fn gen_legal_moves(&self) -> Vec<ChessMove> {
        if self.checkers != EMPTY {
            if self.checkers.popcnt() > 1 {
                //TODO only generate moves that involve moving king
                Vec::new()
            } else {
                //TODO only generate moves that involve moving king, blocking check, or capturing checker
                Vec::new()
            }
        } else if self.pinned != EMPTY {
            //TODO filter pseudo legal moves for any moves that involve a pinned piece that does not move along pinned line
            self.gen_psuedo_legal_moves()
        } else {
            self.gen_psuedo_legal_moves()
        }
    }

    //TODO test
    //TODO determine castling
    pub fn gen_psuedo_legal_moves(&self) -> Vec<ChessMove> {
        let mut move_vec: Vec<ChessMove> = Vec::new();
        let pawns = self.piece_bbs[self.side_to_move][PAWNS_BB];
        let knights = self.piece_bbs[self.side_to_move][KNIGHTS_BB];
        let bishops = self.piece_bbs[self.side_to_move][BISHOPS_BB];
        let rooks = self.piece_bbs[self.side_to_move][ROOKS_BB];
        let queens = self.piece_bbs[self.side_to_move][QUEENS_BB];
        let kings = self.piece_bbs[self.side_to_move][KINGS_BB];
        let own_side = self.color_bbs[self.side_to_move];

        for bit in pawns.bits() {
            let square = SQUARES[bit];
            if self.side_to_move == WHITE {
                let cm = self.valid_white_pawn_moves(square);
                if cm != EMPTY {
                    move_vec.push(ChessMove::new(
                        square,
                        cm,
                    ));
                }
            } else {
                let cm = self.valid_black_pawn_moves(square);
                if cm != EMPTY {
                    move_vec.push(ChessMove::new(
                        square,
                        cm,
                    ));
                }
            }
        };

        for bit in knights.bits() {
            let square = SQUARES[bit];
            let cm = self.valid_knight_moves(square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(
                    square,
                    cm,
                ));
            }
        };

        for bit in bishops.bits() {
            let square = SQUARES[bit];
            let cm = self.valid_bishop_moves(square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(
                    square,
                    cm,
                ));
            }
        };

        for bit in rooks.bits() {
            let square = SQUARES[bit];
            let cm = self.valid_rook_moves(square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(
                    square,
                    cm,
                ));
            }
        };

        for bit in queens.bits() {
            let square = SQUARES[bit];
            let cm = self.valid_queen_moves(square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(
                    square,
                    cm,
                ));
            }
        };

        for bit in kings.bits() {
            let square = SQUARES[bit];
            let cm = self.valid_king_moves(square, own_side);
            if cm != EMPTY {
                move_vec.push(ChessMove::new(
                    square,
                    cm,
                ));
            }
        };

        move_vec
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

        let bishop_attackers = board.valid_bishop_moves(ksq, board.color_bbs[board.side_to_move]);
        let rook_attackers = board.valid_rook_moves(ksq, board.color_bbs[board.side_to_move]);
        let knight_attackers = board.valid_knight_moves(ksq, board.color_bbs[board.side_to_move]);
        let pawn_attackers: BitBoard;
        if board.side_to_move == WHITE {
            pawn_attackers = board.valid_white_pawn_moves(ksq);
        } else {
            pawn_attackers = board.valid_black_pawn_moves(ksq);
        }

        checkers ^= bishop_attackers & (other_pieces_collection[BISHOPS_BB] | other_pieces_collection[QUEENS_BB]);
        checkers ^= rook_attackers & (other_pieces_collection[ROOKS_BB] | other_pieces_collection[QUEENS_BB]);
        checkers ^= knight_attackers & other_pieces_collection[KNIGHTS_BB];
        checkers ^= pawn_attackers & other_pieces_collection[PAWNS_BB];

        (checkers, EMPTY)
    }

    //TODO test
    //TODO handle promotion
    //TODO test for castling
    //TODO test for en passant
    pub fn make_move(&mut self, chessmove: &ChessMove) -> &mut Self {
        self.en_passant = EMPTY;

        let moving_piece = self.get_piece_at(chessmove.from);
        let target_piece = self.get_piece_at(chessmove.to);
        let combined_move = chessmove.from | chessmove.to;

        self.combined_bbs[EMPTY_SQUARES_BB] |= chessmove.from;
        self.combined_bbs[EMPTY_SQUARES_BB] &= !chessmove.to;
        self.combined_bbs[ALL_PIECES_BB] ^= chessmove.from;
        self.combined_bbs[ALL_PIECES_BB] |= chessmove.to;

        match moving_piece {
            Pieces::WPawn => {
                //TODO Update enpassant
                self.piece_bbs[WHITE][PAWNS_BB] ^= combined_move;
                self.color_bbs[WHITE] ^= combined_move;
                self.combined_bbs[ALL_PAWNS_BB] &= !chessmove.from;
                self.combined_bbs[ALL_PAWNS_BB] |= chessmove.to;
            },
            Pieces::BPawn => {
                //TODO Update enpassant
                self.piece_bbs[BLACK][PAWNS_BB] ^= combined_move;
                self.color_bbs[BLACK] ^= combined_move;
                self.combined_bbs[ALL_PAWNS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_PAWNS_BB] |= chessmove.to;
            },
            Pieces::WKnight => {
                self.piece_bbs[WHITE][KNIGHTS_BB] ^= combined_move;
                self.color_bbs[WHITE] ^= combined_move;
                self.combined_bbs[ALL_KNIGHTS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_KNIGHTS_BB] |= chessmove.to;
            },
            Pieces::BKnight => {
                self.piece_bbs[BLACK][KNIGHTS_BB] ^= combined_move;
                self.color_bbs[BLACK] ^= combined_move;
                self.combined_bbs[ALL_KNIGHTS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_KNIGHTS_BB] |= chessmove.to;
            },
            Pieces::WBishop => {
                self.piece_bbs[WHITE][BISHOPS_BB] ^= combined_move;
                self.color_bbs[WHITE] ^= combined_move;
                self.combined_bbs[ALL_BISHOPS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_BISHOPS_BB] |= chessmove.to;
            },
            Pieces::BBishop => {
                self.piece_bbs[BLACK][BISHOPS_BB] ^= combined_move;
                self.color_bbs[BLACK] ^= combined_move;
                self.combined_bbs[ALL_BISHOPS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_BISHOPS_BB] |= chessmove.to;
            },
            Pieces::WRook => {
                //TODO Update castling rights
                self.piece_bbs[WHITE][ROOKS_BB] ^= combined_move;
                self.color_bbs[WHITE] ^= combined_move;
                self.combined_bbs[ALL_ROOKS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_ROOKS_BB] |= chessmove.to;
            },
            Pieces::BRook => {
                //TODO Update castling rights
                self.piece_bbs[BLACK][ROOKS_BB] ^= combined_move;
                self.color_bbs[BLACK] ^= combined_move;
                self.combined_bbs[ALL_ROOKS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_ROOKS_BB] |= chessmove.to;
            },
            Pieces::WQueen => {
                self.piece_bbs[WHITE][QUEENS_BB] ^= combined_move;
                self.color_bbs[WHITE] ^= combined_move;
                self.combined_bbs[ALL_QUEENS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_QUEENS_BB] |= chessmove.to;
            },
            Pieces::BQueen => {
                self.piece_bbs[BLACK][QUEENS_BB] ^= combined_move;
                self.color_bbs[BLACK] ^= combined_move;
                self.combined_bbs[ALL_QUEENS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_QUEENS_BB] |= chessmove.to;
            },
            Pieces::WKing => {
                //TODO Update castling rights
                self.piece_bbs[WHITE][KINGS_BB] ^= combined_move;
                self.color_bbs[WHITE] ^= combined_move;
                self.combined_bbs[ALL_KINGS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_KINGS_BB] |= chessmove.to;
            },
            Pieces::BKing => {
                //TODO Update castling rights
                self.piece_bbs[BLACK][KINGS_BB] ^= combined_move;
                self.color_bbs[BLACK] ^= combined_move;
                self.combined_bbs[ALL_KINGS_BB] ^= chessmove.from;
                self.combined_bbs[ALL_KINGS_BB] |= chessmove.to;
            },
            _ => {
                //TODO Handle error
            }
        }

        match target_piece {
            Pieces::WPawn => {
                //TODO Update enpassant
                self.piece_bbs[WHITE][PAWNS_BB] ^= chessmove.to;
                self.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BPawn => {
                //TODO Update enpassant
                self.piece_bbs[BLACK][PAWNS_BB] ^= chessmove.to;                
                self.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WKnight => {
                self.piece_bbs[WHITE][KNIGHTS_BB] ^= chessmove.to;                
                self.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BKnight => {
                self.piece_bbs[BLACK][KNIGHTS_BB] ^= chessmove.to;                
                self.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WBishop => {
                self.piece_bbs[WHITE][BISHOPS_BB] ^= chessmove.to;                
                self.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BBishop => {
                self.piece_bbs[BLACK][BISHOPS_BB] ^= chessmove.to;                
                self.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WRook => {
                //TODO Update castling rights
                self.piece_bbs[WHITE][ROOKS_BB] ^= chessmove.to;                
                self.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BRook => {
                //TODO Update castling rights
                self.piece_bbs[BLACK][ROOKS_BB] ^= chessmove.to;                
                self.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WQueen => {
                self.piece_bbs[WHITE][QUEENS_BB] ^= chessmove.to;                
                self.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BQueen => {
                self.piece_bbs[BLACK][QUEENS_BB] ^= chessmove.to;                
                self.color_bbs[BLACK] ^= chessmove.to;
            },
            Pieces::WKing => {
                //TODO Update castling rights
                self.piece_bbs[WHITE][KINGS_BB] ^= chessmove.to;                
                self.color_bbs[WHITE] ^= chessmove.to;
            },
            Pieces::BKing => {
                //TODO Update castling rights
                self.piece_bbs[BLACK][KINGS_BB] ^= chessmove.to;                
                self.color_bbs[BLACK] ^= chessmove.to;
            },
            _ => {
                //TODO Handle error
            }
        }

        let (checkers, pinned) = Board::find_checkers_and_pinners(self);

        self.checkers = checkers;
        self.pinned = pinned;
        if self.side_to_move == WHITE {
            self.side_to_move = BLACK;
        } else {
            self.side_to_move = BLACK;
        }

        self
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
        let one_step = (squares.shl(8)) & self.combined_bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_3).shl(8)) & self.combined_bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;

        let left_attack = (squares & CLEAR_A_FILE).shl(7);
        let right_attack = (squares & CLEAR_H_FILE).shl(9);
        let attacks = left_attack | right_attack;
        let valid_attacks = attacks & self.color_bbs[BLACK];

        valid_steps | valid_attacks
    }

    pub fn valid_black_pawn_moves(&self, squares: BitBoard) -> BitBoard {
        let one_step = (squares.shr(8)) & self.combined_bbs[EMPTY_SQUARES_BB];
        let two_steps = ((one_step & RANK_6).shr(8)) & self.combined_bbs[EMPTY_SQUARES_BB];
        let valid_steps = one_step | two_steps;

        let left_attack = (squares & CLEAR_A_FILE).shr(9);
        let right_attack = (squares & CLEAR_H_FILE).shr(7);
        let attacks = left_attack | right_attack;
        let valid_attacks = attacks & self.color_bbs[WHITE];

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
        attacks |= self.combined_bbs[EMPTY_SQUARES_BB] & (attacks.shr(8));
        let mut empty = !own_pieces & (!own_pieces.shr(8));
        attacks |= empty & (attacks.shr(16));
        empty &= empty.shr(16);
        attacks |= empty & (attacks.shr(32));
        (attacks.shr(8)) & !own_pieces
    }

    pub fn north_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        attacks |= !own_pieces & (attacks.shl(8));
        let mut empty = !own_pieces & (!own_pieces.shl(8));
        attacks |= empty & (attacks.shl(16));
        empty &= empty.shl(16);
        attacks |= empty & (attacks.shl(32));
        (attacks.shl(8)) & !own_pieces
    }

    pub fn west_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = !own_pieces & CLEAR_H_FILE;
        attacks |= empty & (attacks.shr(1));
        empty &= empty.shr(1);
        attacks |= empty & (attacks.shr(2));
        empty &= empty.shr(2);
        attacks |= empty & (attacks.shr(4));
        attacks & !own_pieces
    }

    pub fn east_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = !own_pieces & CLEAR_A_FILE;
        attacks |= empty & (attacks.shl(1));
        empty &= empty.shl(1);
        attacks |= empty & (attacks.shl(2));
        empty &= empty.shl(2);
        attacks |= empty & (attacks.shl(4));
        attacks & !own_pieces
    }

    pub fn north_east_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = !own_pieces & CLEAR_A_FILE;
        attacks |= empty & (attacks.shl(9));
        empty &= empty.shl(9);
        attacks |= empty & (attacks.shl(18));
        empty &= empty.shl(18);
        attacks |= empty & (attacks.shl(36));
        attacks & !own_pieces
    }

    pub fn south_east_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = !own_pieces & CLEAR_A_FILE;
        attacks |= empty & (attacks.shr(7));
        empty &= empty.shr(7);
        attacks |= empty & (attacks.shr(14));
        empty &= empty.shr(14);
        attacks |= empty & (attacks.shr(28));
        attacks & !own_pieces
    }

    pub fn south_west_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = !own_pieces & CLEAR_H_FILE;
        attacks |= empty & (attacks.shr(9));
        empty &= empty.shr(9);
        attacks |= empty & (attacks.shr(18));
        empty &= empty.shr(18);
        attacks |= empty & (attacks.shr(36));
        attacks & !own_pieces
    }

    pub fn north_west_attacks(&self, mut attacks: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let mut empty = !own_pieces & CLEAR_H_FILE;
        attacks |= empty & (attacks.shl(7));
        empty &= empty.shl(7);
        attacks |= empty & (attacks.shl(14));
        empty &= empty.shl(14);
        attacks |= empty & (attacks.shl(28));
        attacks & !own_pieces
    }

    pub fn print_board(&self) {
        println!("{}", self);
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    mod gen_psuedo_legal_moves {
        use super::*;

        #[test]
        fn it_works() {
            let b = Board::default();
            let chessmoves = ChessMove::broken_up(b.gen_psuedo_legal_moves());

            for chessmove in chessmoves {
                chessmove.from.print_bb();
                chessmove.to.print_bb();
            }
        }
    }

    mod make_move {
        use super::*;
        #[test]
        fn it_works() {
            let mut b = Board::default();

            b
                .make_move(&ChessMove::from_notation("E2", "E4"))
                .make_move(&ChessMove::from_notation("F7", "F5"))
                .make_move(&ChessMove::from_notation("D1", "H5"));

            assert_eq!(b.get_piece_at(F5_SQUARE), Pieces::BPawn);
            assert_eq!(b.checkers, H5_SQUARE);
        }
    }

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
                b.valid_king_moves(b.piece_bbs[WHITE][KINGS_BB], b.color_bbs[WHITE]),
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
                b.valid_king_moves(b.piece_bbs[WHITE][KINGS_BB], b.color_bbs[WHITE]),
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
                b.valid_king_moves(b.piece_bbs[WHITE][KINGS_BB], b.color_bbs[WHITE]),
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
                b.valid_knight_moves(b.piece_bbs[WHITE][KNIGHTS_BB], b.color_bbs[WHITE]),
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
                b.valid_knight_moves(b.piece_bbs[WHITE][KNIGHTS_BB], b.color_bbs[WHITE]),
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
                b.valid_knight_moves(b.piece_bbs[WHITE][KNIGHTS_BB], b.color_bbs[WHITE]),
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
                b.valid_rook_moves(b.piece_bbs[WHITE][ROOKS_BB], b.color_bbs[WHITE]),
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
                b.valid_rook_moves(b.piece_bbs[WHITE][ROOKS_BB], b.color_bbs[WHITE]),
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
                A2_SQUARE |
                C2_SQUARE |
                D3_SQUARE |
                E4_SQUARE |
                F5_SQUARE |
                G6_SQUARE |
                H7_SQUARE;

            assert_eq!(
                b.valid_bishop_moves(b.piece_bbs[WHITE][BISHOPS_BB], b.color_bbs[WHITE]),
                valid_squares
            );
        }

        #[test]
        fn it_works_with_other_shared_pieces() {
            let b = Board::new(
                EMPTY, G6_SQUARE, B1_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, WHITE,
            );

            let valid_squares =
                A2_SQUARE |
                C2_SQUARE |
                D3_SQUARE |
                E4_SQUARE |
                F5_SQUARE;

            assert_eq!(
                b.valid_bishop_moves(b.piece_bbs[WHITE][BISHOPS_BB], b.color_bbs[WHITE]),
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
                EMPTY, WHITE,
            );

            let bishop_moves = b.valid_bishop_moves(b.piece_bbs[WHITE][QUEENS_BB], b.color_bbs[WHITE]);
            let rook_moves = b.valid_rook_moves(b.piece_bbs[WHITE][QUEENS_BB], b.color_bbs[WHITE]);
            let valid_moves = bishop_moves | rook_moves;

            assert_eq!(
                b.valid_queen_moves(b.piece_bbs[WHITE][QUEENS_BB], b.color_bbs[WHITE]),
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

            assert_eq!(b.valid_white_pawn_moves(b.piece_bbs[WHITE][PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_from_non_home_square() {
            let b = Board::new(
                RANK_4, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WHITE,
            );

            let valid_squares = A5_SQUARE
                | B5_SQUARE
                | C5_SQUARE
                | D5_SQUARE
                | E5_SQUARE
                | F5_SQUARE
                | G5_SQUARE
                | H5_SQUARE;

            assert_eq!(b.valid_white_pawn_moves(b.piece_bbs[WHITE][PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_captures() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, F6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = E6_SQUARE | F6_SQUARE;

            assert_eq!(b.valid_white_pawn_moves(b.piece_bbs[WHITE][PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_obstacles() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = EMPTY;

            assert_eq!(b.valid_white_pawn_moves(b.piece_bbs[WHITE][PAWNS_BB]), valid_squares);
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

            assert_eq!(b.valid_black_pawn_moves(b.piece_bbs[BLACK][PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_from_non_home_square() {
            let b = Board::new(
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, RANK_4, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WHITE,
            );

            let valid_squares = A3_SQUARE
                | B3_SQUARE
                | C3_SQUARE
                | D3_SQUARE
                | E3_SQUARE
                | F3_SQUARE
                | G3_SQUARE
                | H3_SQUARE;

            assert_eq!(b.valid_black_pawn_moves(b.piece_bbs[BLACK][PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_captures() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, F6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = E5_SQUARE | F5_SQUARE;

            assert_eq!(b.valid_black_pawn_moves(b.piece_bbs[BLACK][PAWNS_BB]), valid_squares);
        }

        #[test]
        fn it_works_with_obstacles() {
            let b = Board::new(
                E5_SQUARE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, E6_SQUARE, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, WHITE,
            );

            let valid_squares = EMPTY;

            assert_eq!(b.valid_black_pawn_moves(b.piece_bbs[BLACK][PAWNS_BB]), valid_squares);
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
