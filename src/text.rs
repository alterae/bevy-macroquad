use bevy::{app, ecs::system, log};

pub struct Plugin {
    font: Font,
}

impl Plugin {
    pub fn new(font: Font) -> Self {
        Self { font }
    }
}

impl app::Plugin for Plugin {
    fn build(&self, app: &mut app::App) {
        app.insert_resource(self.font.clone())
            .init_resource::<Console>()
            .add_systems(app::Startup, init)
            .add_systems(app::PostUpdate, draw);
    }
}

fn init(mut console: system::ResMut<Console>, font: system::Res<Font>) {
    console.clear(&font);

    log::info!(
        "Setting initial console dimensions to {} x {}.",
        console.width,
        console.height
    );
    log::info!("Font: {} ({} x {})", font.path, font.width, font.height);
}

#[derive(Clone, system::Resource)]
pub struct Font {
    path: String,
    texture: macroquad::texture::Texture2D,
    width: f32,
    height: f32,
}

impl Font {
    pub async fn new(path: &str) -> Self {
        let texture = macroquad::texture::load_texture(path).await.unwrap();
        texture.set_filter(macroquad::miniquad::FilterMode::Nearest);
        let width = texture.width() / 16.;
        let height = texture.height() / 16.;

        Self {
            path: path.to_owned(),
            texture,
            width,
            height,
        }
    }

    fn get_rect(&self, glyph: u8) -> macroquad::math::Rect {
        let x = (glyph % 16) as f32 * self.width;
        let y = (glyph / 16) as f32 * self.height;

        macroquad::math::Rect {
            x,
            y,
            w: self.width,
            h: self.height,
        }
    }
}

#[derive(Default, system::Resource)]
pub struct Console {
    buffer: Vec<Option<Cell>>,
    width: usize,
    height: usize,
}

impl Console {
    fn pos_to_idx(&self, pos: impl Position) -> usize {
        pos.y() as usize * self.width + pos.x() as usize
    }

    fn idx_to_pos(&self, idx: usize) -> (i32, i32) {
        ((idx % self.width) as i32, (idx / self.width) as i32)
    }

    pub fn put_char(&mut self, pos: impl Position, c: impl Copy + TryInto<u8>) {
        let idx = self.pos_to_idx(pos);

        self.buffer[idx] = Some(Cell {
            glyph: c.try_into().unwrap_or(0),
        })
    }

    pub fn put_str(&mut self, pos: impl Position, text: &str) {
        for (i, c) in text.char_indices() {
            self.put_char((pos.x() + i as i32, pos.y()), c);
        }
    }

    fn clear(&mut self, font: &Font) {
        let width = (macroquad::window::screen_width() / font.width) as usize;
        let height = (macroquad::window::screen_height() / font.height) as usize;

        self.buffer = vec![None; width * height];
        self.width = width;
        self.height = height;
    }
}

#[derive(Clone, Copy)]
struct Cell {
    glyph: u8,
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

fn draw(mut console: system::ResMut<Console>, font: system::Res<Font>) {
    for (idx, cell) in console.buffer.iter().enumerate() {
        if let Some(cell) = cell {
            let (x, y) = console.idx_to_pos(idx);
            macroquad::texture::draw_texture_ex(
                &font.texture,
                x as f32 * font.width,
                y as f32 * font.height,
                macroquad::color::WHITE,
                macroquad::texture::DrawTextureParams {
                    source: Some(font.get_rect(cell.glyph)),
                    ..Default::default()
                },
            );
        }
    }

    console.clear(&font);
}
