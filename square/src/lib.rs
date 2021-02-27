extern crate bitboard;
use bitboard::BitBoard;
extern crate piece;
use piece::Pieces;
extern crate constants;
use constants::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub bitboard: BitBoard,
    pub piece: Pieces,
}

impl Default for Square {
    fn default() -> Self {
        Square {
            bitboard: EMPTY,
            piece: Pieces::Empty,
        }
    }
}

impl Square {
    pub fn new(bitboard: BitBoard, piece: Pieces) -> Self {
        Square { bitboard, piece }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
