#[macro_use]
extern crate lazy_static;
extern crate game;
use board::Board;
use clipboard::{ClipboardContext, ClipboardProvider};
use game::Game;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
extern crate piece;
use piece::Pieces;
extern crate bitboard;
use bitboard::*;
extern crate chessmove;
use chessmove::ChessMove;
extern crate movegen;
use movegen::MoveGen;
extern crate constants;
use constants::*;
use ggez::event::{run, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{clear, present, Color};
use ggez::input::mouse;
mod moving_piece;
use moving_piece::*;
mod draw;
use draw::*;
mod promotion;
use promotion::*;

struct MainState {
    game: Game,
    move_from: BitBoard,
    last_move: (BitBoard, BitBoard),
    needs_draw: bool,
    valid_moves: Vec<ChessMove>,
    moving_pieces: Rc<RefCell<HashMap<BitBoard, MovingPiece>>>,
    dragged_piece: Option<(BitBoard, Pieces, (f32, f32))>,
    promotion_panel: Option<PromotionUI>,
}

impl MainState {
    fn new() -> Self {
        let game = Game::default();
        let valid_moves = MoveGen::gen_legal_moves(&game.board);

        MainState {
            game,
            move_from: EMPTY,
            last_move: (EMPTY, EMPTY),
            needs_draw: true,
            valid_moves,
            moving_pieces: Rc::new(RefCell::new(HashMap::new())),
            dragged_piece: None,
            promotion_panel: None,
        }
    }

    fn is_valid_destination(&self, to: BitBoard) -> bool {
        self.valid_moves.iter().any(|chessmove| {
            ((chessmove.from & self.move_from).is_not_empty())
                && ((chessmove.to & to).is_not_empty())
        })
    }

    fn go_back(&mut self) {
        if self.game.history.len() > 1 {
            let prev_board = self.game.board;
            let last_move = self.game.go_back();

            self.move_pieces_between_game_boards(&prev_board, &self.game.board);
            let valid_moves = MoveGen::gen_legal_moves(&self.game.board);
            self.move_from = EMPTY;
            self.valid_moves = valid_moves;
            self.needs_draw = true;
            self.last_move = last_move;
        }
    }

    fn go_forward(&mut self) {
        if self.game.future.len() > 0 {
            let prev_board = self.game.board;
            let last_move = self.game.go_forward();

            self.move_pieces_between_game_boards(&prev_board, &self.game.board);
            let valid_moves = MoveGen::gen_legal_moves(&self.game.board);
            self.move_from = EMPTY;
            self.valid_moves = valid_moves;
            self.needs_draw = true;
            self.last_move = last_move;
        }
    }

    fn debug(&mut self, verbose: bool) {
        self.game.board.print_board();
        println!("{}", self.game.board.to_fen());

        self.game.board.pinned.print_bb("Pinned");
        self.game.board.checkers.print_bb("Checkers");
        self.game
            .board
            .attacked_squares
            .print_bb("Attacked Squares");

        if verbose {
            print!("Valid moves");
            for i in 0..self.valid_moves.len() {
                let cm = self.valid_moves[i];
                cm.from.print_bb("From");
                cm.to.print_bb("To");
            }
        }
    }

    fn make_move(&mut self, from: BitBoard, to: BitBoard) {
        let moving_piece = self.game.board.get_piece_at(from);
        if moving_piece == Pieces::WPawn && (to & RANK_8).is_not_empty() {
            self.handle_white_promotion(from, to);
        } else if moving_piece == Pieces::BPawn && (to & RANK_1).is_not_empty() {
            self.handle_black_promotion(from, to);
        } else {
            self.commit_move(from, to);
        }
        self.move_from = EMPTY;
    }

    fn commit_move(&mut self, from: BitBoard, to: BitBoard) {
        let moves = self.game.make_move(&ChessMove::new(from, to));
        self.valid_moves = MoveGen::gen_legal_moves(&self.game.board);
        self.last_move = (from, to);
        let mut moving_pieces = self.moving_pieces.borrow_mut();
        moves.iter().for_each(|move_tuple| {
            moving_pieces.insert(
                move_tuple.1,
                MovingPiece::new(
                    self.game.board.get_piece_at(move_tuple.1),
                    move_tuple.0,
                    move_tuple.1,
                    SQUARE_SIZE,
                    20,
                ),
            );
        });
    }

    fn handle_white_promotion(&mut self, from: BitBoard, to: BitBoard) {
        self.promotion_panel = Some(PromotionUI::new(from, to, true));
    }

    fn commit_promotion(&mut self, from: BitBoard, to: BitBoard, piece: Pieces) {
        let moves = self.game.make_move(&ChessMove::promote(from, to, piece));
        self.valid_moves = MoveGen::gen_legal_moves(&self.game.board);
        self.last_move = (from, to);
        let mut moving_pieces = self.moving_pieces.borrow_mut();
        moves.iter().for_each(|move_tuple| {
            moving_pieces.insert(
                move_tuple.1,
                MovingPiece::new(
                    self.game.board.get_piece_at(move_tuple.1),
                    move_tuple.0,
                    move_tuple.1,
                    SQUARE_SIZE,
                    20,
                ),
            );
        });
    }

    fn handle_black_promotion(&mut self, from: BitBoard, to: BitBoard) {
        self.promotion_panel = Some(PromotionUI::new(from, to, false));
    }

    fn restart_from_fen(&mut self, fen: &str) {
        let prev_board = self.game.board;
        self.game.restart_from_fen(fen);
        self.move_pieces_between_game_boards(&prev_board, &self.game.board);
        self.restart_game();
    }

    fn restart_game(&mut self) {
        let (checkers, pinned, attacked_squares) =
            MoveGen::calculate_derived_bitboards(&self.game.board);
        self.game.board.attacked_squares = attacked_squares;
        self.game.board.checkers = checkers;
        self.game.board.pinned = pinned;

        let valid_moves = MoveGen::gen_legal_moves(&self.game.board);

        self.last_move = (EMPTY, EMPTY);
        if self.game.board.en_passant.is_not_empty() {
            if (self.game.board.en_passant & RANK_6).is_not_empty() {
                self.last_move = (
                    self.game.board.en_passant.shl(8),
                    self.game.board.en_passant.shr(8),
                );
            } else {
                self.last_move = (
                    self.game.board.en_passant.shr(8),
                    self.game.board.en_passant.shl(8),
                );
            }
        }

        self.promotion_panel = None;
        self.dragged_piece = None;
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

    fn new_game(&mut self) {
        let old_board = self.game.board;
        self.game.restart_game();
        let new_board = self.game.board;
        self.move_pieces_between_game_boards(&old_board, &new_board);

        self.restart_game();
    }

    fn update_moving_pieces(&mut self) {
        let mut to_remove: Vec<BitBoard> = Vec::new();
        let mut moving_pieces = self.moving_pieces.borrow_mut();
        moving_pieces.iter_mut().for_each(|(bitboard, piece)| {
            piece.update();
            if piece.done == true {
                to_remove.push(*bitboard);
            }
        });

        to_remove.iter().for_each(|bitboard| {
            moving_pieces.remove(bitboard);
        });
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
                                if found_pieces[y][x] == false
                                    && old_pieces[y][x].piece == square.piece
                                {
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
                        if found_piece.is_not_empty() {
                            found_pieces[found_piece.row()][found_piece.col()] = true;
                            movers.insert(square.bitboard, found_piece);
                        } else {
                            movers.insert(square.bitboard, EMPTY);
                        }
                    }
                }
            });
        });

        let mut moving_pieces = self.moving_pieces.borrow_mut();

        movers.iter().for_each(|(to, from)| {
            moving_pieces.insert(
                *to,
                MovingPiece::new(
                    new_game_board.get_piece_at(*to),
                    *from,
                    *to,
                    SQUARE_SIZE,
                    20,
                ),
            );
        });
    }
}

pub fn is_fen(fen: &String) -> bool {
    use regex::Regex;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s*([rnbqkpRNBQKP1-8]+/){7}([rnbqkpRNBQKP1-8]+)\s[bw-]\s(([a-hkqA-HKQ]{1,4})|(-))\s(([a-h][36])|(-))\s\d+\s\d+\s*").unwrap();
    }
    RE.is_match(fen.as_str())
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let Some(ref mut dragged_piece) = self.dragged_piece {
            let pos = mouse::position(ctx);
            if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
                dragged_piece.2 .0 = pos.x;
                dragged_piece.2 .1 = pos.y;
            } else {
                let destination = coord_to_bitboard(pos.x, pos.y);
                if self.move_from.is_not_empty() && self.is_valid_destination(destination) {
                    self.make_move(self.move_from, destination);
                }
                self.dragged_piece = None;
            }
            self.needs_draw = true;
        }

        self.update_moving_pieces();

        let mut to_promote_from: BitBoard = EMPTY;
        let mut to_promote_to: BitBoard = EMPTY;
        let mut to_promote: Pieces = Pieces::Empty;

        if let Some(ref mut promotion_panel) = self.promotion_panel {
            promotion_panel
                .update(ctx)
                .expect("Error updating promotion panel");
            if promotion_panel.done {
                if let Some(promotion_piece) = promotion_panel.selected {
                    to_promote_from = promotion_panel.from;
                    to_promote_to = promotion_panel.to;
                    to_promote = promotion_piece;
                }
                self.promotion_panel = None;
            }
        }

        if to_promote != Pieces::Empty {
            self.commit_promotion(to_promote_from, to_promote_to, to_promote);
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if let Some(ref mut promotion_panel) = self.promotion_panel {
            return promotion_panel.mouse_button_down_event(ctx, button, x, y);
        }
        let destination = coord_to_bitboard(x, y);
        if self.move_from.is_not_empty() && self.is_valid_destination(destination) {
            self.make_move(self.move_from, destination);
        } else {
            self.move_from = destination;
            let moving_piece = self.game.board.get_piece_at(self.move_from);
            if moving_piece != Pieces::Empty {
                self.dragged_piece = Some((destination, moving_piece, (x, y)));
            }
        }
        self.needs_draw = true;
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, keycode: KeyCode, keymods: KeyMods) {
        match keycode {
            KeyCode::C => {
                if keymods.contains(KeyMods::CTRL) || keymods.contains(KeyMods::LOGO) {
                    let mut cp: ClipboardContext = ClipboardProvider::new().unwrap();
                    let fen = self.game.board.to_fen();
                    cp.set_contents(fen).expect("Failed to set fen contents");
                }
            }
            KeyCode::V => {
                if keymods.contains(KeyMods::CTRL) || keymods.contains(KeyMods::LOGO) {
                    let mut cp: ClipboardContext = ClipboardProvider::new().unwrap();
                    let contents = cp.get_contents().unwrap();
                    if is_fen(&contents) {
                        self.restart_from_fen(contents.as_str());
                    }
                }
            }
            KeyCode::R => {
                if keymods.contains(KeyMods::CTRL) || keymods.contains(KeyMods::LOGO) {
                    self.randomize_board();
                }
            }
            KeyCode::N => {
                if keymods.contains(KeyMods::CTRL) || keymods.contains(KeyMods::LOGO) {
                    self.new_game();
                }
            }
            KeyCode::Z => {
                if keymods.contains(KeyMods::CTRL) || keymods.contains(KeyMods::LOGO) {
                    self.go_back();
                }
            }
            KeyCode::Y => {
                if keymods.contains(KeyMods::CTRL) || keymods.contains(KeyMods::LOGO) {
                    self.go_forward();
                }
            }
            KeyCode::D => {
                if keymods.contains(KeyMods::CTRL) || keymods.contains(KeyMods::LOGO) {
                    self.debug(keymods.contains(KeyMods::SHIFT));
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.needs_draw == false {
            return Ok(());
        }

        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        let mut moving_pieces = self.moving_pieces.borrow_mut();
        let dragging_from = match self.dragged_piece {
            Some(dragged_piece) => dragged_piece.0,
            None => EMPTY,
        };

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
                        dragging_from == square.bitboard,
                    )
                    .expect("Failed to draw square");
                    if self.is_valid_destination(square.bitboard) {
                        draw_destination(ctx, row_idx, col_idx)
                            .expect("Failed to draw destination");
                    }
                })
            });

        draw_last_move_border(ctx, self.last_move.0.row(), self.last_move.0.col())
            .expect("Failed to draw destination");
        draw_last_move_border(ctx, self.last_move.1.row(), self.last_move.1.col())
            .expect("Failed to draw destination");
        if self.move_from.is_not_empty() {
            draw_move_from_border(ctx, self.move_from.row(), self.move_from.col())
                .expect("Failed to draw destination");
        }

        moving_pieces.iter_mut().for_each(|(_, piece)| {
            draw_piece_at(ctx, piece.pos.0, piece.pos.1, &piece.piece, false)
                .expect("Error drawing moving piece");
        });

        if let Some(dragged_piece) = self.dragged_piece {
            let destination = coord_to_bitboard(dragged_piece.2 .0, dragged_piece.2 .1);
            if self.is_valid_destination(destination) {
                draw_valid_drop_target_border(ctx, destination.row(), destination.col())
                    .expect("Failed to draw destination drop border");
            }
            draw_piece_at(
                ctx,
                dragged_piece.2 .0,
                dragged_piece.2 .1,
                &dragged_piece.1,
                false,
            )
            .expect("Error drawing dragged piece");
        }

        if let Some(ref mut promotion_panel) = self.promotion_panel {
            promotion_panel
                .draw(ctx)
                .expect("Error drawing promotion panel");
        } else if moving_pieces.len() == 0 && self.dragged_piece == None {
            self.needs_draw = false;
        }

        present(ctx).expect("error presenting");

        Ok(())
    }
}

fn main() -> ggez::GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Chess", "Rob Taussig")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .window_setup(ggez::conf::WindowSetup {
            title: "Rusty Chess".into(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: "".into(),
            srgb: true,
        })
        .build()
        .unwrap();
    let main_state = &mut MainState::new();
    run(ctx, event_loop, main_state)
}
