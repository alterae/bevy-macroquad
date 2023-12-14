use bevy::ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Clone, Resource)]
pub struct Font {
    pub path: String,
    pub texture: Texture2D,
    pub width: f32,
    pub height: f32,
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

    pub fn get_rect(&self, glyph: u8) -> Rect {
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
