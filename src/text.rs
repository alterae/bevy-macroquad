use bevy::{app::prelude::*, ecs::prelude::*, log};

pub use color::{Color, Palette};
pub use console::{Cell, Console};
pub use font::Font;

mod color;
mod console;
mod font;

pub struct Plugin {
    font: Font,
    palette: Palette,
}

impl Plugin {
    pub fn new(font: Font, palette: impl Into<Option<Palette>>) -> Self {
        Self {
            font,
            palette: palette.into().unwrap_or_default(),
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.font.clone())
            .insert_resource(self.palette.clone())
            .init_resource::<Console>()
            .add_systems(Startup, init)
            .add_systems(PostUpdate, console::draw);
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
