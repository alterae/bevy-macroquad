use bevy::prelude::*;

use engine::{mq, text};

mod engine;
mod player;

#[macroquad::main(conf)]
async fn main() {
    let config = engine::Config::new().await;

    let font = text::Font::new(&config.font_path).await;
    let palette = text::Palette::new(&config.palette_path).await;

    let mut app = App::new();
    app.add_plugins((bevy::DefaultPlugins, text::Plugin::new(font, palette)))
        .add_systems(Startup, player::spawn)
        .add_systems(PreUpdate, engine::_stress_test)
        .add_systems(
            Update,
            (engine::command_q, engine::fps_display, player::render),
        );

    loop {
        app.update();

        mq::next_frame().await;
    }
}

fn conf() -> mq::Conf {
    mq::Conf {
        window_title: "Hello, world!".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}
