use bitboard::BitBoard;

#[derive(Clone)]
pub struct Moment {
    pub last_move: (BitBoard, BitBoard),
    pub fen: String,
}

impl Moment {
    pub fn new(fen: String, last_move: (BitBoard, BitBoard)) -> Moment {
        Moment { fen, last_move }
    }
}
