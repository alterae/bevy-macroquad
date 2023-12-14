use bevy::{app::prelude::*, ecs::prelude::*, log};

pub use color::{Color, Palette};
pub use console::Console;
pub use font::Font;

mod color;
mod console;
mod font;

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
            .init_resource::<Palette>()
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
