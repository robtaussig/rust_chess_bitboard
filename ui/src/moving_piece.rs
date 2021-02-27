use bitboard::BitBoard;
use piece::{Pieces};
use ggez::graphics::{
    draw, Color, DrawMode, DrawParam, Mesh,
};

pub struct MovingPiece {
    _piece: Pieces,
    pub bitboard: BitBoard,
    pos: (f32, f32),
    to: (f32, f32),
    radius: f32,
    vel: (f32, f32),
    pub done: bool,
    color: Color,
}

impl MovingPiece {
    pub fn new(piece: Pieces, from_square: BitBoard, to_square: BitBoard, square_size: f32, frames_to_dest: u64) -> Self {
        let (from_row, from_col) = (from_square.row() as f32, from_square.col() as f32);
        let (to_row, to_col) = (to_square.row() as f32, to_square.col() as f32);

        let (from_x, from_y) = (from_col * square_size + (square_size / 2f32), from_row * square_size + (square_size / 2f32));
        let (to_x, to_y) = (to_col * square_size + (square_size / 2f32), to_row * square_size + (square_size / 2f32));

        let color = match piece.is_white() {
            true => Color::new(0.8, 0.8, 0.8, 1.0),
            false => Color::new(0.2, 0.2, 0.2, 1.0),
        };

        MovingPiece {
            _piece: piece,
            bitboard: to_square,
            pos: (from_x, from_y),
            to: (to_x, to_y),
            radius: square_size * 0.4,
            vel: ((to_x - from_x) / (frames_to_dest as f32 * 1.1f32), (to_y - from_y) / (frames_to_dest as f32 * 1.1f32)),
            done: false,
            color,
        }
    }

    pub fn update(&mut self) -> &Self {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;

        self.vel.0 *= 1.1;
        self.vel.1 *= 1.1;

        if 
            (self.vel.0 > 0f32 && self.pos.0 >= self.to.0)
            || (self.vel.1 > 0f32 && self.pos.1 >= self.to.1)
            || (self.vel.0 < 0f32 && self.pos.0 <= self.to.0)
            || (self.vel.1 < 0f32 && self.pos.1 <= self.to.1)
        {
            self.done = true;
        }

        self
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            ggez::mint::Point2 {
                x: self.pos.0,
                y: self.pos.1,
            },
            self.radius,
            0.1,
            self.color,
        )
        .expect("error building piece");
    
        draw(ctx, &mesh, DrawParam::default())
    }
}
