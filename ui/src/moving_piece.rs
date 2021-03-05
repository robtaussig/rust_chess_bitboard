use bitboard::BitBoard;
use piece::Pieces;

pub struct MovingPiece {
    pub piece: Pieces,
    pub bitboard: BitBoard,
    pub pos: (f32, f32),
    to: (f32, f32),
    vel: (f32, f32),
    pub done: bool,
}

impl MovingPiece {
    pub fn new(
        piece: Pieces,
        from_square: BitBoard,
        to_square: BitBoard,
        square_size: f32,
        frames_to_dest: u64,
    ) -> Self {
        let (from_row, from_col) = (from_square.row() as f32, from_square.col() as f32);
        let (to_row, to_col) = (to_square.row() as f32, to_square.col() as f32);

        let (from_x, from_y) = (
            from_col * square_size + (square_size / 2f32),
            from_row * square_size + (square_size / 2f32),
        );
        let (to_x, to_y) = (
            to_col * square_size + (square_size / 2f32),
            to_row * square_size + (square_size / 2f32),
        );

        MovingPiece {
            piece,
            bitboard: to_square,
            pos: (from_x, from_y),
            to: (to_x, to_y),
            vel: (
                (to_x - from_x) / (frames_to_dest as f32 * 1.1f32),
                (to_y - from_y) / (frames_to_dest as f32 * 1.1f32),
            ),
            done: false,
        }
    }

    pub fn update(&mut self) -> &Self {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;

        self.vel.0 *= 1.1;
        self.vel.1 *= 1.1;

        if (self.vel.0 > 0f32 && self.pos.0 >= self.to.0)
            || (self.vel.1 > 0f32 && self.pos.1 >= self.to.1)
            || (self.vel.0 < 0f32 && self.pos.0 <= self.to.0)
            || (self.vel.1 < 0f32 && self.pos.1 <= self.to.1)
        {
            self.done = true;
        }

        self
    }
}
