use bevy::prelude::*;

use crate::engine::prelude::*;

pub use color::{Color, Palette};
pub use console::{draw, Cell, Console};
pub use font::Font;

mod color;
mod console;
mod font;
pub mod ui;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Font>()
            .init_resource::<Palette>()
            .init_resource::<Console>()
            .add_systems(Startup, init)
            .add_systems(PostUpdate, console::draw);
    }
}

pub fn init(mut console: ResMut<Console>, font: Res<Font>) {
    console.clear(&font);

    log::info!(
        "Setting initial console dimensions to {} x {}.",
        console.width,
        console.height
    );
    log::info!("Font: {} ({} x {})", font.path, font.width, font.height);
}
