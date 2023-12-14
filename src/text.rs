use bevy::{app::prelude::*, ecs::prelude::*, log};
use macroquad::prelude::*;

use crate::color;

pub struct Plugin {
    font: Font,
}

impl Plugin {
    pub fn new(font: Font) -> Self {
        Self { font }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.font.clone())
            .init_resource::<color::Palette>()
            .init_resource::<Console>()
            .add_systems(Startup, init)
            .add_systems(PostUpdate, draw);
    }
}

fn init(mut console: ResMut<Console>, font: Res<Font>) {
    console.clear(&font);

    log::info!(
        "Setting initial console dimensions to {} x {}.",
        console.width,
        console.height
    );
    log::info!("Font: {} ({} x {})", font.path, font.width, font.height);
}

#[derive(Clone, Resource)]
pub struct Font {
    path: String,
    texture: Texture2D,
    width: f32,
    height: f32,
}

impl Font {
    pub async fn new(path: &str) -> Self {
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        let width = texture.width() / 16.;
        let height = texture.height() / 16.;

        Self {
            path: path.to_owned(),
            texture,
            width,
            height,
        }
    }

    fn get_rect(&self, glyph: u8) -> Rect {
        let x = (glyph % 16) as f32 * self.width;
        let y = (glyph / 16) as f32 * self.height;

        Rect {
            x,
            y,
            w: self.width,
            h: self.height,
        }
    }
}

#[derive(Default, Resource)]
pub struct Console {
    buffer: Vec<Option<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Console {
    fn pos_to_idx(&self, pos: impl Position) -> usize {
        pos.y() as usize * self.width + pos.x() as usize
    }

    fn idx_to_pos(&self, idx: usize) -> (i32, i32) {
        ((idx % self.width) as i32, (idx / self.width) as i32)
    }

    pub fn put_char(
        &mut self,
        pos: impl Position,
        c: impl Copy + TryInto<u8>,
        fg: color::Color,
        bg: color::Color,
    ) {
        let idx = self.pos_to_idx(pos);

        self.buffer[idx] = Some(Cell {
            glyph: c.try_into().unwrap_or(0),
            fg,
            bg,
        })
    }

    pub fn put_str(&mut self, pos: impl Position, text: &str, fg: color::Color, bg: color::Color) {
        for (i, c) in text.char_indices() {
            self.put_char((pos.x() + i as i32, pos.y()), c, fg, bg);
        }
    }

    fn clear(&mut self, font: &Font) {
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
    fg: color::Color,
    bg: color::Color,
}

pub trait Position {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

impl<T> Position for (T, T)
where
    T: Copy + TryInto<i32>,
{
    fn x(&self) -> i32 {
        self.0.try_into().unwrap_or(0)
    }

    fn y(&self) -> i32 {
        self.1.try_into().unwrap_or(0)
    }
}

fn draw(mut console: ResMut<Console>, font: Res<Font>, palette: Res<color::Palette>) {
    clear_background(palette[color::Black]);

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
