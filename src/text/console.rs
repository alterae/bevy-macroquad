use bevy::ecs::prelude::*;
use macroquad::prelude::*;

use crate::math;

#[derive(Default, Resource)]
pub struct Console {
    buffer: Vec<Option<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Console {
    fn pos_to_idx(&self, pos: impl math::Position) -> usize {
        pos.y() as usize * self.width + pos.x() as usize
    }

    fn idx_to_pos(&self, idx: usize) -> (i32, i32) {
        ((idx % self.width) as i32, (idx / self.width) as i32)
    }

    pub fn put_char(
        &mut self,
        pos: impl math::Position,
        c: impl Copy + TryInto<u8>,
        fg: super::Color,
        bg: super::Color,
    ) {
        let idx = self.pos_to_idx(pos);

        self.buffer[idx] = Some(Cell {
            glyph: c.try_into().unwrap_or(0),
            fg,
            bg,
        })
    }

    pub fn put_str(
        &mut self,
        pos: impl math::Position,
        text: &str,
        fg: super::Color,
        bg: super::Color,
    ) {
        for (i, c) in text.char_indices() {
            self.put_char((pos.x() + i as i32, pos.y()), c, fg, bg);
        }
    }

    pub fn clear(&mut self, font: &super::Font) {
        let width = (screen_width() / font.width) as usize;
        let height = (screen_height() / font.height) as usize;

        self.buffer = vec![None; width * height];
        self.width = width;
        self.height = height;
    }
}

#[derive(Clone, Copy)]
struct Cell {
    glyph: u8,
    fg: super::Color,
    bg: super::Color,
}

pub fn draw(mut console: ResMut<Console>, font: Res<super::Font>, palette: Res<super::Palette>) {
    clear_background(palette[super::Color::Black]);

    for (idx, cell) in console.buffer.iter().enumerate() {
        if let Some(cell) = cell {
            let (x, y) = console.idx_to_pos(idx);

            draw_rectangle(
                x as f32 * font.width,
                y as f32 * font.height,
                font.width,
                font.height,
                palette[cell.bg],
            );

            draw_texture_ex(
                &font.texture,
                x as f32 * font.width,
                y as f32 * font.height,
                palette[cell.fg],
                DrawTextureParams {
                    source: Some(font.get_rect(cell.glyph)),
                    ..Default::default()
                },
            );
        }
    }

    console.clear(&font);
}
