extern crate board;
use board::Board;
extern crate game;
use game::Game;
extern crate piece;
use piece::Pieces;
extern crate bitboard;
use bitboard::BitBoard;
extern crate chessmove;
use chessmove::ChessMove;
extern crate movegen;
use movegen::MoveGen;
extern crate constants;
use constants::*;
use ggez;
use ggez::event::{run, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{
    clear, draw, present, Color, DrawMode, DrawParam, Font, Mesh, Rect, Scale, Text,
};
use ggez::input::keyboard;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 600.;

const SQUARE_SIZE: f32 = SCREEN_HEIGHT / 8.;
const VALID_DESTINATION_COLOR: Color = Color::new(1.0, 0.0, 1.0, 1.0);
const MOVE_FROM_COLOR: Color = Color::new(0.0, 0.0, 1.0, 1.0);
const LAST_MOVE_FROM_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);
const LAST_MOVE_TO_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);
const WHITE_SQUARE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
const BLACK_SQUARE: Color = Color::new(0.0, 0.0, 0.0, 1.0);
const WHITE_PIECE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
const BLACK_PIECE_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);

struct MainState {
    game: Game,
    move_from: BitBoard,
    last_move: (BitBoard, BitBoard),
    needs_draw: bool,
    valid_moves: Vec<ChessMove>,
}

impl MainState {
    fn new() -> Self {
        let game = Game::new();
        let valid_moves = MoveGen::gen_legal_moves(&game.board);

        MainState {
            game,
            move_from: EMPTY,
            last_move: (EMPTY, EMPTY),
            needs_draw: true,
            valid_moves,
        }
    }

    fn is_valid_destination(&self, to: BitBoard) -> bool {
        self.valid_moves.iter().any(|chessmove| {
            ((chessmove.from & self.move_from) != EMPTY) && ((chessmove.to & to) != EMPTY)
        })
    }

    fn commit_move(&mut self, from: BitBoard, to: BitBoard) {
        self.game.make_move(&ChessMove::new(from, to));
        self.valid_moves = MoveGen::gen_legal_moves(&self.game.board);
        self.last_move = (from, to);
    }
}

pub fn draw_piece(
    ctx: &mut ggez::Context,
    row: usize,
    col: usize,
    piece: &Pieces,
) -> ggez::GameResult {
    let center_y = (row as f32 * SQUARE_SIZE) + (SQUARE_SIZE / 2f32);
    let center_x = (col as f32 * SQUARE_SIZE) + (SQUARE_SIZE / 2f32);
    let color: Color;
    if piece.white() {
        color = WHITE_PIECE_COLOR;
    } else {
        color = BLACK_PIECE_COLOR;
    }
    let mesh = Mesh::new_circle(
        ctx,
        DrawMode::fill(),
        ggez::mint::Point2 {
            x: center_x,
            y: center_y,
        },
        SQUARE_SIZE * 0.4,
        0.1,
        color,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default())
}

pub fn draw_square(
    ctx: &mut ggez::Context,
    row: usize,
    col: usize,
    piece: &Pieces,
    is_last_move_from: bool,
    is_last_move_to: bool,
    is_moving_from: bool,
    is_valid_destination: bool,
) -> ggez::GameResult {
    let is_white = row % 2 == col % 2;
    let rect = Rect::new(
        col as f32 * SQUARE_SIZE,
        row as f32 * SQUARE_SIZE,
        SQUARE_SIZE,
        SQUARE_SIZE,
    );
    let color: Color;
    if is_moving_from {
        color = MOVE_FROM_COLOR;
    } else if is_valid_destination {
        color = VALID_DESTINATION_COLOR;
    } else if is_last_move_from {
        color = LAST_MOVE_FROM_COLOR;
    } else if is_last_move_to {
        color = LAST_MOVE_TO_COLOR;
    } else if is_white {
        color = WHITE_SQUARE;
    } else {
        color = BLACK_SQUARE;
    }

    let mesh =
        Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color).expect("error creating rect");

    if piece != &Pieces::Empty {
        draw(ctx, &mesh, DrawParam::default());
        draw_piece(ctx, row, col, piece)
    } else {
        draw(ctx, &mesh, DrawParam::default())
    }
}

pub fn coord_to_row_and_square(x: f32, y: f32) -> (usize, usize) {
    ((y / SQUARE_SIZE) as usize, (x / SQUARE_SIZE) as usize)
}

pub fn row_and_col_to_square(row: usize, col: usize) -> BitBoard {
    SQUARES[(7 - row) * 8 + col]
}

pub fn coord_to_bitboard(x: f32, y: f32) -> BitBoard {
    let (row, col) = coord_to_row_and_square(x, y);
    row_and_col_to_square(row, col)
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let destination = coord_to_bitboard(x, y);
        self.needs_draw = true;
        if self.move_from != EMPTY && self.is_valid_destination(destination) {
            self.commit_move(self.move_from, destination);
            self.move_from = EMPTY;
        } else {
            self.move_from = destination;
        }
    }

    //TODO format cell if valid target
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.needs_draw == false {
            return Ok(());
        }
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        self.game
            .board
            .to_array()
            .iter()
            .enumerate()
            .for_each(|(row_idx, row)| {
                row.iter().enumerate().for_each(|(col_idx, square)| {
                    draw_square(
                        ctx,
                        7 - row_idx,
                        col_idx,
                        &square.piece,
                        self.last_move.0 == square.bitboard,
                        self.last_move.1 == square.bitboard,
                        self.move_from == square.bitboard,
                        self.is_valid_destination(square.bitboard),
                    )
                    .expect("Failed to draw square");
                })
            });

        present(ctx).expect("error presenting");

        self.needs_draw = false;
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Chess", "Rob Taussig")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();
    let main_state = &mut MainState::new();
    run(ctx, event_loop, main_state)
}
