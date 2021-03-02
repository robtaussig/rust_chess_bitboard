use std::fmt;
extern crate constants;
use crate::constants::*;

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pieces {
    WPawn,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
    Empty,
}

impl Pieces {
    pub fn is_white(&self) -> bool {
        match self {
            Pieces::WPawn => true,
            Pieces::WKnight => true,
            Pieces::WBishop => true,
            Pieces::WRook => true,
            Pieces::WQueen => true,
            Pieces::WKing => true,
            _ => false,
        }
    }

    pub fn is_black(&self) -> bool {
        match self {
            Pieces::BPawn => true,
            Pieces::BKnight => true,
            Pieces::BBishop => true,
            Pieces::BRook => true,
            Pieces::BQueen => true,
            Pieces::BKing => true,
            _ => false,
        }
    }

    pub fn piece_type(&self) -> PieceType {
        match self {
            Pieces::WPawn => PieceType::Pawn,
            Pieces::WKnight => PieceType::Knight,
            Pieces::WBishop => PieceType::Bishop,
            Pieces::WRook => PieceType::Rook,
            Pieces::WQueen => PieceType::Queen,
            Pieces::WKing => PieceType::King,
            Pieces::BPawn => PieceType::Pawn,
            Pieces::BKnight => PieceType::Knight,
            Pieces::BBishop => PieceType::Bishop,
            Pieces::BRook => PieceType::Rook,
            Pieces::BQueen => PieceType::Queen,
            Pieces::BKing => PieceType::King,
            _ => PieceType::Empty,
        }
    }

    pub fn color_bb_index(&self) -> usize {
        match self {
            Pieces::WPawn => WHITE,
            Pieces::WKnight => WHITE,
            Pieces::WBishop => WHITE,
            Pieces::WRook => WHITE,
            Pieces::WQueen => WHITE,
            Pieces::WKing => WHITE,
            Pieces::BPawn => BLACK,
            Pieces::BKnight => BLACK,
            Pieces::BBishop => BLACK,
            Pieces::BRook => BLACK,
            Pieces::BQueen => BLACK,
            Pieces::BKing => BLACK,
            Pieces::Empty => EMPTY_SQUARES_BB,
        }
    }

    pub fn piece_by_color_bb_index(&self) -> usize {
        match self.piece_type() {
            PieceType::Pawn => PAWNS_BB,
            PieceType::Knight => KNIGHTS_BB,
            PieceType::Bishop => BISHOPS_BB,
            PieceType::Rook => ROOKS_BB,
            PieceType::Queen => QUEENS_BB,
            PieceType::King => KINGS_BB,
            PieceType::Empty => EMPTY_SQUARES_BB,
        }
    }

    pub fn combined_color_bb_index(&self) -> usize {
        match self.piece_type() {
            PieceType::Pawn => ALL_PAWNS_BB,
            PieceType::Knight => ALL_KNIGHTS_BB,
            PieceType::Bishop => ALL_BISHOPS_BB,
            PieceType::Rook => ALL_ROOKS_BB,
            PieceType::Queen => ALL_QUEENS_BB,
            PieceType::King => ALL_KINGS_BB,
            PieceType::Empty => EMPTY_SQUARES_BB,
        }
    }
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Pieces::WPawn => "P",
                Pieces::WKnight => "N",
                Pieces::WBishop => "B",
                Pieces::WRook => "R",
                Pieces::WQueen => "Q",
                Pieces::WKing => "K",
                Pieces::BPawn => "p",
                Pieces::BKnight => "n",
                Pieces::BBishop => "b",
                Pieces::BRook => "r",
                Pieces::BQueen => "q",
                Pieces::BKing => "k",
                Pieces::Empty => ".",
            }
        )
    }
}
