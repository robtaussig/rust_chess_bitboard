use crate::bitboard::{BitBoard};
use crate::constants::{SQUARES};

pub struct ChessMove {
    pub from: BitBoard,
    pub to: BitBoard,
}

impl ChessMove {
    pub fn new(from: BitBoard, to: BitBoard) -> Self {
        ChessMove { from, to }
    }

    // While #from will be a single square, #to represents every square available to the piece on #from
    // #broken_up returns a new Vec where each ChessMove contains only a single destination square
    pub fn broken_up(chessmoves: Vec<ChessMove>) -> Vec<ChessMove> {
        let mut broken_up_chessmoves: Vec<ChessMove> = Vec::new();

        for chessmove in chessmoves {
            for bit in chessmove.to.bits() {
                broken_up_chessmoves.push(ChessMove::new(
                    chessmove.from,
                    SQUARES[bit]
                ))
            }
        }

        broken_up_chessmoves
    }
}
