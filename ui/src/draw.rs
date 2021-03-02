extern crate game;
extern crate piece;
use piece::Pieces;
extern crate bitboard;
use bitboard::BitBoard;
extern crate constants;
use constants::*;
use ggez;
use ggez::mint::{Point2};
use ggez::graphics::{
    draw, Color, DrawMode, DrawParam, Mesh, Rect,
};

const SQUARE_SIZE: f32 = SCREEN_HEIGHT / 8.;

const MOVE_FROM_COLOR: Color = Color::new(0.2, 0.4, 0.2, 1.0);
const LAST_MOVE_BORDER_COLOR: Color = Color::new(0.6, 0.3, 0.3, 1.0);

const WHITE_SQUARE: Color = Color::new(0.9, 0.9, 0.9, 1.0);
const BLACK_SQUARE: Color = Color::new(0.1, 0.1, 0.1, 1.0);

const WHITE_PIECE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
const BLACK_PIECE_COLOR: Color = Color::new(0.5, 0.8, 0.45, 1.0);

const PIECE_BORDER_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);
const BORDER_WIDTH: f32 = 2f32;

pub fn draw_piece(
    ctx: &mut ggez::Context,
    row: usize,
    col: usize,
    piece: &Pieces,
) -> ggez::GameResult {
    let (center_x, center_y) = row_and_col_to_coord(row, col);
    
    draw_piece_at(ctx, center_x, center_y, piece)
}

pub fn draw_border(
    ctx: &mut ggez::Context,
    points: &[Point2<f32>],
) -> ggez::GameResult {
    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::stroke(BORDER_WIDTH),
        &points,
        PIECE_BORDER_COLOR,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default())
}

pub fn draw_pawn(ctx: &mut ggez::Context, x: f32, y: f32, color: Color) -> ggez::GameResult {
    let points = [
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
    ];

    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &points,
        color,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default()).expect("Error drawing piece");
    draw_border(ctx, &points)
}

pub fn draw_rook(ctx: &mut ggez::Context, x: f32, y: f32, color: Color) -> ggez::GameResult {
    let points = [
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.18,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.18,
            y: y - SQUARE_SIZE * 0.18,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.06,
            y: y - SQUARE_SIZE * 0.18,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.06,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.06,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.06,
            y: y - SQUARE_SIZE * 0.18,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.18,
            y: y - SQUARE_SIZE * 0.18,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.18,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
    ];

    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &points,
        color,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default()).expect("Error drawing piece");
    draw_border(ctx, &points)
}

pub fn draw_knight(ctx: &mut ggez::Context, x: f32, y: f32, color: Color) -> ggez::GameResult {
    let points = [
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.18,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.18,
            y: y + SQUARE_SIZE * 0.18,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.18,
            y: y - SQUARE_SIZE * 0.06,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.06,
            y: y + SQUARE_SIZE * 0.06,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.06,
        },
    ];

    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &points,
        color,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default()).expect("Error drawing piece");
    draw_border(ctx, &points)
}

pub fn draw_bishop(ctx: &mut ggez::Context, x: f32, y: f32, color: Color) -> ggez::GameResult {
    let points = [
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.06,
        },
        Point2 {
            x,
            y: y - SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y - SQUARE_SIZE * 0.06,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.3,
            y: y + SQUARE_SIZE * 0.3,
        },
    ];

    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &points,
        color,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default()).expect("Error drawing piece");
    draw_border(ctx, &points)
}

pub fn draw_king(ctx: &mut ggez::Context, x: f32, y: f32, color: Color) -> ggez::GameResult {
    let points = [
        Point2 {
            x: x - SQUARE_SIZE * 0.14,
            y: y - SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.14,
            y: y - SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.14,
            y: y - SQUARE_SIZE * 0.14,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.42,
            y: y - SQUARE_SIZE * 0.14,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.42,
            y: y + SQUARE_SIZE * 0.14,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.14,
            y: y + SQUARE_SIZE * 0.14,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.14,
            y: y + SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.14,
            y: y + SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.14,
            y: y + SQUARE_SIZE * 0.14,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.42,
            y: y + SQUARE_SIZE * 0.14,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.42,
            y: y - SQUARE_SIZE * 0.14,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.14,
            y: y - SQUARE_SIZE * 0.14,
        },
    ];

    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &points,
        color,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default()).expect("Error drawing piece");
    draw_border(ctx, &points)
}

pub fn draw_queen(ctx: &mut ggez::Context, x: f32, y: f32, color: Color) -> ggez::GameResult {
    let points = [
        Point2 {
            x: x - SQUARE_SIZE * 0.42,
            y: y - SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.105,
            y,
        },
        Point2 {
            x: x,
            y: y - SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.105,
            y,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.42,
            y: y - SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x + SQUARE_SIZE * 0.315,
            y: y + SQUARE_SIZE * 0.42,
        },
        Point2 {
            x: x - SQUARE_SIZE * 0.315,
            y: y + SQUARE_SIZE * 0.42,
        },
    ];

    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &points,
        color,
    )
    .expect("error building piece");

    draw(ctx, &mesh, DrawParam::default()).expect("Error drawing piece");
    draw_border(ctx, &points)
}

pub fn draw_piece_at(
    ctx: &mut ggez::Context,
    x: f32,
    y: f32,
    piece: &Pieces,
) -> ggez::GameResult {
    let color: Color;
    if piece.is_white() {
        color = WHITE_PIECE_COLOR;
    } else {
        color = BLACK_PIECE_COLOR;
    }

    match piece {
        Pieces::WPawn => draw_pawn(ctx, x, y, color),
        Pieces::BPawn => draw_pawn(ctx, x, y, color),
        Pieces::WKnight => draw_knight(ctx, x, y, color),
        Pieces::BKnight => draw_knight(ctx, x, y, color),
        Pieces::WBishop => draw_bishop(ctx, x, y, color),
        Pieces::BBishop => draw_bishop(ctx, x, y, color),
        Pieces::WRook => draw_rook(ctx, x, y, color),
        Pieces::BRook => draw_rook(ctx, x, y, color),
        Pieces::WQueen => draw_queen(ctx, x, y, color),
        Pieces::BQueen => draw_queen(ctx, x, y, color),
        Pieces::WKing => draw_king(ctx, x, y, color),
        Pieces::BKing => draw_king(ctx, x, y, color),
        Pieces::Empty => panic!("Cannot draw empty square"),
    }
}

pub fn draw_destination(
    ctx: &mut ggez::Context,
    row: usize,
    col: usize,
) -> ggez::GameResult {
    let (center_x, center_y) = row_and_col_to_coord(row, col);

    let mesh = Mesh::new_circle(
        ctx,
        DrawMode::fill(),
        Point2 {
            x: center_x,
            y: center_y,
        },
        SQUARE_SIZE * 0.2,
        0.1,
        Color::new(0.0, 0.0, 0.0, 0.6),
    ).unwrap();
    draw(ctx, &mesh, DrawParam::default())
}

pub fn draw_last_move_border(
    ctx: &mut ggez::Context,
    row: usize,
    col: usize,
) -> ggez::GameResult {
    let rect = Rect::new(
        col as f32 * SQUARE_SIZE,
        row as f32 * SQUARE_SIZE,
        SQUARE_SIZE,
        SQUARE_SIZE,
    );
    let mesh =
        Mesh::new_rectangle(ctx, DrawMode::stroke(3f32), rect, LAST_MOVE_BORDER_COLOR).expect("error creating rect");
    draw(ctx, &mesh, DrawParam::default())
}

pub fn draw_square(
    ctx: &mut ggez::Context,
    row: usize,
    col: usize,
    piece: &Pieces,
    is_moving_from: bool,
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
    } else if is_white {
        color = WHITE_SQUARE;
    } else {
        color = BLACK_SQUARE;
    }

    let mesh =
        Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color).expect("error creating rect");

    if piece != &Pieces::Empty {
        draw(ctx, &mesh, DrawParam::default()).expect("error drawing square");
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

pub fn row_and_col_to_coord(row: usize, col: usize) -> (f32, f32) {
    (
        (col as f32 * SQUARE_SIZE) + (SQUARE_SIZE / 2f32),
        (row as f32 * SQUARE_SIZE) + (SQUARE_SIZE / 2f32)
    )
}
