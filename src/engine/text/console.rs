use bevy::prelude::*;

use crate::engine::{math, mq};

use super::Color;

#[derive(Default, Resource)]
pub struct Console {
    pub buffer: Vec<Option<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Console {
    fn pos_to_idx(&self, pos: impl math::Vec2) -> usize {
        pos.y() as usize * self.width + pos.x() as usize
    }

    pub fn idx_to_pos(&self, idx: usize) -> (i32, i32) {
        ((idx % self.width) as i32, (idx / self.width) as i32)
    }

    pub fn put_char(
        &mut self,
        pos: impl math::Vec2,
        c: impl Copy + TryInto<u8>,
        fg: super::Color,
        bg: super::Color,
    ) {
        if pos.x() < 0
            || pos.x() >= self.width as i32
            || pos.y() < 0
            || pos.y() >= self.height as i32
        {
            return;
        }

        let idx = self.pos_to_idx(pos);

        self.buffer[idx] = Some(Cell {
            glyph: c.try_into().unwrap_or(0),
            fg,
            bg,
        })
    }

    pub fn put_str(
        &mut self,
        pos: impl math::Vec2,
        text: &str,
        fg: super::Color,
        bg: super::Color,
    ) {
        for (i, c) in text.char_indices() {
            self.put_char((pos.x() + i as i32, pos.y()), c, fg, bg);
        }
    }

    pub fn clear(&mut self, font: &super::Font) {
        let width = (mq::screen_width() / font.width) as usize;
        let height = (mq::screen_height() / font.height) as usize;

        self.buffer = vec![None; width * height];
        self.width = width;
        self.height = height;
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub glyph: u8,
    pub fg: super::Color,
    pub bg: super::Color,
}

pub fn draw(mut console: ResMut<Console>, font: Res<super::Font>, palette: Res<super::Palette>) {
    mq::clear_background(palette[super::Color::Black]);

    for (idx, cell) in console.buffer.iter().enumerate() {
        if let Some(cell) = cell {
            let (x, y) = console.idx_to_pos(idx);

            if cell.bg != Color::Black {
                mq::draw_texture_ex(
                    &font.texture,
                    x as f32 * font.width,
                    y as f32 * font.height,
                    palette[cell.bg],
                    mq::DrawTextureParams {
                        source: Some(font.get_rect(219)),
                        ..Default::default()
                    },
                );
            }

            if cell.fg != cell.bg {
                mq::draw_texture_ex(
                    &font.texture,
                    x as f32 * font.width,
                    y as f32 * font.height,
                    palette[cell.fg],
                    mq::DrawTextureParams {
                        source: Some(font.get_rect(cell.glyph)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    console.clear(&font);
}
