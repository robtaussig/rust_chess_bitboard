use bitboard::BitBoard;
use piece::Pieces;
use constants::*;
use ggez::{self, graphics::{Color, DrawMode}};
use ggez::input::mouse;
use ggez::event::{EventHandler};
use crate::draw::{coord_to_bitboard, draw_arbitrary_rectangle, draw_piece};

pub struct PromotionUI {
    pub from: BitBoard,
    pub to: BitBoard,
    pub selected: Option<Pieces>,
    pub done: bool,
    pub is_white: bool,
    highlighted_square: Option<BitBoard>,
    panel_expanded: bool,
    y_top: f32,
    y_bottom: f32,
    y_top_vel: f32,
    y_bottom_vel: f32,
}

const PANEL_COLOR: Color = Color::new(0.7, 0.7, 0.7, 1.0);
const HIGHLIGHTED_PANEL_COLOR: Color = Color::new(0.6, 0.6, 0.8, 1.0);
const PANEL_BORDER_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);

impl PromotionUI {
    pub fn new(from: BitBoard, to: BitBoard, is_white: bool) -> PromotionUI {
        let (y_top, y_bottom, y_top_vel, y_bottom_vel) = match is_white {
            true => (
                to.row() as f32 * SQUARE_SIZE,
                to.row() as f32 * SQUARE_SIZE,
                0f32,
                30f32,
            ),
            false => (
                to.row() as f32 * SQUARE_SIZE + SQUARE_SIZE,
                to.row() as f32 * SQUARE_SIZE + SQUARE_SIZE,
                -30f32,
                0f32
            ),
        };

        PromotionUI {
            from,
            to,
            selected: None,
            done: false,
            is_white,
            y_top,
            y_bottom,
            y_top_vel,
            y_bottom_vel,
            panel_expanded: false,
            highlighted_square: None,
        }
    }
}

pub fn absolute_difference(left: usize, right: usize) -> usize {
    if left > right {
        left - right
    } else if right > left {
        right - left
    } else {
        0
    }
}

impl EventHandler for PromotionUI {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mouse_pos = mouse::position(ctx);
        let destination = coord_to_bitboard(mouse_pos.x, mouse_pos.y);
        if destination.col() == self.to.col() {
            let distance = absolute_difference(self.to.row(),destination.row());
            if distance <= 3 {
                self.highlighted_square = Some(destination);
            }
        }

        if (self.y_bottom + self.y_bottom_vel) - (self.y_top + self.y_top_vel) >= SQUARE_SIZE * 4f32 {
            self.panel_expanded = true;
            if self.is_white {
                self.y_bottom = self.y_top + SQUARE_SIZE * 4f32;
            } else {
                self.y_top = self.y_bottom - SQUARE_SIZE * 4f32;
            }
        } else {
            self.y_top += self.y_top_vel;
            self.y_bottom += self.y_bottom_vel;
            if self.is_white {
                self.y_bottom_vel -= 1f32;
                if self.y_bottom_vel < 10f32 {
                    self.y_bottom_vel = 10f32;
                }
            } else {
                self.y_top_vel += 1f32;
                if self.y_top_vel > -10f32 {
                    self.y_top_vel = -10f32;
                }
            }
        }
        
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if self.panel_expanded && button == ggez::event::MouseButton::Left {
            let destination = coord_to_bitboard(x, y);
            if destination.col() == self.to.col() {
                let distance = absolute_difference(self.to.row(),destination.row());
                let promotion_piece = match distance {
                    0 => match self.is_white {
                        true => Pieces::WQueen,
                        false => Pieces::BQueen,
                    },
                    1 => match self.is_white {
                        true => Pieces::WRook,
                        false => Pieces::BRook,
                    },
                    2 => match self.is_white {
                        true => Pieces::WBishop,
                        false => Pieces::BBishop,
                    },
                    3 => match self.is_white {
                        true => Pieces::WKnight,
                        false => Pieces::BKnight,
                    },
                    _ => Pieces::Empty,
                };

                match promotion_piece {
                    Pieces::Empty => (),
                    _ => {
                        self.selected = Some(promotion_piece);
                        self.done = true;
                    },
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let col = self.to.col();
        let x = col as f32 * SQUARE_SIZE;
        let y = self.y_top;
        let width = SQUARE_SIZE;
        let height = self.y_bottom - self.y_top;

        draw_arbitrary_rectangle(
            ctx,
            x,
            y,
            width,
            height,
            PANEL_COLOR,
            None,
            None,
        ).expect("Error drawing panel");

        if self.panel_expanded {
            if let Some(highlighted_square) = self.highlighted_square {
                draw_arbitrary_rectangle(
                    ctx,
                    highlighted_square.col() as f32 * SQUARE_SIZE,
                    highlighted_square.row() as f32 * SQUARE_SIZE,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    HIGHLIGHTED_PANEL_COLOR,
                    None,
                    None,
                ).expect("Error drawing panel");
            }
            let (queen, rook, bishop, knight) = match self.is_white {
                true => (
                    (Pieces::WQueen, (self.to.row(), self.to.col())),
                    (Pieces::WRook, (self.to.shr(8).row(), self.to.col())),
                    (Pieces::WBishop, (self.to.shr(16).row(), self.to.col())),
                    (Pieces::WKnight, (self.to.shr(24).row(), self.to.col())),
                ),
                false => (
                    (Pieces::BQueen, (self.to.row(), self.to.col())),
                    (Pieces::BRook, (self.to.shl(8).row(), self.to.col())),
                    (Pieces::BBishop, (self.to.shl(16).row(), self.to.col())),
                    (Pieces::BKnight, (self.to.shl(24).row(), self.to.col())),
                ),
            };

            draw_piece(ctx, queen.1.0, queen.1.1, &queen.0, false).expect("Error drawing queen");
            draw_piece(ctx, rook.1.0, rook.1.1, &rook.0, false).expect("Error drawing rook");
            draw_piece(ctx, bishop.1.0, bishop.1.1, &bishop.0, false).expect("Error drawing bishop");
            draw_piece(ctx, knight.1.0, knight.1.1, &knight.0, false).expect("Error drawing knight");
        }

        draw_arbitrary_rectangle(
            ctx,
            x,
            y,
            width,
            height,
            PANEL_BORDER_COLOR,
            Some(DrawMode::stroke(3f32)),
            None,
        ).expect("Error drawing panel");
    
        Ok(())
    }
}
