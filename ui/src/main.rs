#[macro_use] extern crate lazy_static;
extern crate game;
use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use board::Board;
use clipboard::{ClipboardProvider, ClipboardContext};
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
use ggez::mint::{Point2};
use ggez::event::{run, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{
    clear, draw, present, Color, DrawMode, DrawParam, Mesh, Rect,
};
mod moving_piece;
use moving_piece::*;

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
const SEARCH_DIRS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct MainState {
    game: Game,
    move_from: BitBoard,
    last_move: (BitBoard, BitBoard),
    needs_draw: bool,
    valid_moves: Vec<ChessMove>,
    moving_pieces: Rc<RefCell<HashMap<BitBoard, MovingPiece>>>,
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
            moving_pieces: Rc::new(RefCell::new(HashMap::new())),
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
        let mut moving_pieces = self.moving_pieces.borrow_mut();
        moving_pieces.insert(to, MovingPiece::new(
            self.game.board.get_piece_at(to),
            from,
            to,
            SQUARE_SIZE,
            20,
        ));
    }

    fn restart_from_fen(&mut self, fen: &str) {
        let game = Game::from_fen(fen);
        
        self.move_pieces_between_game_boards(&self.game.board, &game.board);
        self.game = game;
        self.restart_game();
    }

    fn restart_game(&mut self) {
        let valid_moves = MoveGen::gen_legal_moves(&self.game.board);
        
        self.last_move = (EMPTY, EMPTY);
        self.move_from = EMPTY;
        self.valid_moves = valid_moves;
        self.needs_draw = true;
    }

    fn randomize_board(&mut self) {
        let old_board = self.game.board;
        self.game.randomize_board();
        let new_board = self.game.board;
        self.move_pieces_between_game_boards(&old_board, &new_board);

        self.restart_game();
    }

    fn move_pieces_between_game_boards(&self, old_game_board: &Board, new_game_board: &Board) {
        let old_pieces = old_game_board.to_array();
        let new_pieces = new_game_board.to_array();

        let mut movers: HashMap<BitBoard, BitBoard> = HashMap::new();
        let mut found_pieces = [[false; 8]; 8];

        new_pieces.iter().for_each(|row| {
            row.iter().for_each(|square| {
                let search_pointer_x = square.bitboard.col();
                let search_pointer_y = square.bitboard.row();

                if old_pieces[search_pointer_y][search_pointer_x].piece == square.piece {
                    found_pieces[search_pointer_y][search_pointer_x] = true;
                }
            });
        });

        new_pieces.iter().for_each(|row| {
            row.iter().for_each(|square| {
                if square.piece != Pieces::Empty {
                    let mut searched = [[false; 8]; 8];
                    let search_pointer_x = square.bitboard.col();
                    let search_pointer_y = square.bitboard.row();
                    if found_pieces[search_pointer_y][search_pointer_x] == false {
                        let mut found_piece: BitBoard = EMPTY;
                        searched[search_pointer_y][search_pointer_x] = true;
                        let mut to_search = vec![(search_pointer_x, search_pointer_y)];
                        while to_search.len() > 0 {
                            let (x, y) = to_search.remove(0);
                            if searched[y][x] == false {
                                searched[y][x] = true;
                                if found_pieces[y][x] == false && old_pieces[y][x].piece == square.piece {
                                    found_piece = old_pieces[y][x].bitboard;
                                    break;
                                }
                            }
                            for (x_dir, y_dir) in SEARCH_DIRS.iter() {
                                let next_x = x as i8 + x_dir;
                                let next_y = y as i8 + y_dir;
                                if next_x >= 0 && next_y >= 0 && next_x <= 7 && next_y <= 7 {
                                    if searched[next_y as usize][next_x as usize] == false {
                                        to_search.push((next_x as usize, next_y as usize));
                                    }
                                }
                            }
                        }
                        if found_piece != EMPTY {
                            found_pieces[found_piece.row()][found_piece.col()] = true;
                            movers.insert(square.bitboard, found_piece);
                        }
                    }
                }
                
            });
        });

        let mut moving_pieces = self.moving_pieces.borrow_mut();
        
        movers.iter().for_each(|(to, from)| {
            moving_pieces.insert(*to, MovingPiece::new(
                new_game_board.get_piece_at(*to),
                *from,
                *to,
                SQUARE_SIZE,
                20,
            ));
        });
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
    
    draw_piece_at(ctx, center_x, center_y, piece)
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

    draw(ctx, &mesh, DrawParam::default())
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

    draw(ctx, &mesh, DrawParam::default())
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

    draw(ctx, &mesh, DrawParam::default())
}

//TODO
pub fn draw_bishop(ctx: &mut ggez::Context, x: f32, y: f32, color: Color) -> ggez::GameResult {
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

    draw(ctx, &mesh, DrawParam::default())
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
        Pieces::WQueen => draw_pawn(ctx, x, y, color),
        Pieces::BQueen => draw_pawn(ctx, x, y, color),
        Pieces::WKing => draw_pawn(ctx, x, y, color),
        Pieces::BKing => draw_pawn(ctx, x, y, color),
        Pieces::Empty => draw_pawn(ctx, x, y, color),
    }
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

pub fn is_fen(fen: &String) -> bool {
    use regex::Regex;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s*([rnbqkpRNBQKP1-8]+/){7}([rnbqkpRNBQKP1-8]+)\s[bw-]\s(([a-hkqA-HKQ]{1,4})|(-))\s(([a-h][36])|(-))\s\d+\s\d+\s*").unwrap();
    }
    RE.is_match(fen.as_str())
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut moving_pieces = self.moving_pieces.borrow_mut();
        let mut to_remove: Vec<BitBoard> = Vec::new();
        moving_pieces.iter_mut().for_each(|(bitboard, piece)| {
            piece.update();
            if piece.done == true {
                to_remove.push(*bitboard);
            }
        });

        to_remove.iter().for_each(|bitboard| {
            moving_pieces.remove(bitboard);
        });

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

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, keycode: KeyCode, keymods: KeyMods) {
        match keycode {
            KeyCode::V => {
                if keymods.contains(KeyMods::CTRL) {
                    let mut cp: ClipboardContext = ClipboardProvider::new().unwrap();
                    let contents = cp.get_contents().unwrap();
                    if is_fen(&contents) {
                        self.restart_from_fen(contents.as_str());
                    }
                }
            },
            KeyCode::R => {
                if keymods.contains(KeyMods::CTRL) {
                    self.randomize_board();
                }
            },
            _ => (),
        }
    }

    //TODO format cell if valid target
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.needs_draw == false {
            return Ok(());
        }
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        let mut moving_pieces = self.moving_pieces.borrow_mut();

        self.game
            .board
            .to_array()
            .iter()
            .enumerate()
            .for_each(|(row_idx, row)| {
                row.iter().enumerate().for_each(|(col_idx, square)| {
                    let piece: Pieces;
                    if moving_pieces.contains_key(&square.bitboard) == true {
                        piece = Pieces::Empty;
                    } else {
                        piece = square.piece;
                    }
                    draw_square(
                        ctx,
                        row_idx,
                        col_idx,
                        &piece,
                        self.last_move.0 == square.bitboard,
                        self.last_move.1 == square.bitboard,
                        self.move_from == square.bitboard,
                        self.is_valid_destination(square.bitboard),
                    )
                    .expect("Failed to draw square");
                })
            });

        moving_pieces.iter_mut().for_each(|(_, piece)| {
            draw_piece_at(ctx, piece.pos.0, piece.pos.1, &piece.piece).expect("Error drawing moving piece");
        });

        present(ctx).expect("error presenting");

        if moving_pieces.len() == 0 {
            self.needs_draw = false;
        }
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
