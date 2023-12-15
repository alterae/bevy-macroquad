use bevy::app::prelude::*;
use macroquad::prelude::*;

mod config;
mod math;
mod player;
mod text;
mod util;

#[macroquad::main(conf)]
async fn main() {
    let config = config::Config::new().await;

    let font = text::Font::new(&config.font_path).await;
    let palette = text::Palette::new(&config.palette_path).await;

    let mut app = App::new();
    app.add_plugins((bevy::DefaultPlugins, text::Plugin::new(font, palette)))
        .add_systems(Startup, player::spawn)
        .add_systems(PreUpdate, util::_stress_test)
        .add_systems(Update, (util::command_q, util::fps_display, player::render));

    loop {
        app.update();

        next_frame().await;
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Hello, world!".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}
