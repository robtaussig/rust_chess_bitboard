extern crate bitboard;
use crate::bitboard::BitBoard;
extern crate constants;
use crate::constants::*;
extern crate piece;
use crate::piece::Pieces;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessMove {
    pub from: BitBoard,
    pub to: BitBoard,
    pub promotion: Option<Pieces>,
}

//TODO handle promotion
impl ChessMove {
    pub fn new(from: BitBoard, to: BitBoard) -> Self {
        ChessMove {
            from,
            to,
            promotion: None,
        }
    }

    pub fn promote(from: BitBoard, to: BitBoard, promotion: Pieces) -> Self {
        ChessMove {
            from,
            to,
            promotion: Some(promotion),
        }
    }

    //TODO test
    pub fn from_notation(from: &str, to: &str) -> Self {
        //TODO implement
        ChessMove {
            from: *NOTATION_MAP.get(from).unwrap(),
            to: *NOTATION_MAP.get(to).unwrap(),
            promotion: None,
        }
    }

    // While #from will be a single square, #to represents every square available to the piece on #from
    // #broken_up returns a new Vec where each ChessMove contains only a single destination square
    pub fn broken_up(chessmoves: Vec<ChessMove>) -> Vec<ChessMove> {
        let mut broken_up_chessmoves: Vec<ChessMove> = Vec::new();

        for chessmove in chessmoves {
            for bit in chessmove.to.bits() {
                broken_up_chessmoves.push(ChessMove::new(chessmove.from, SQUARES[bit]))
            }
        }

        broken_up_chessmoves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod from_notation {
        use super::*;

        #[test]
        fn it_works() {
            let m = ChessMove::from_notation("E2", "E4");

            assert_eq!(m.from, E2_SQUARE);
        }
    }

    //TODO
    mod broken_up {}
}
