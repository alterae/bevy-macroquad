use bevy::prelude::*;

use engine::{mq, text, ui};

mod engine;

#[macroquad::main(conf)]
async fn main() {
    let config = engine::Config::new().await;

    let font = text::Font::new(&config.font_path).await;
    let ui_font = text::Font::new(&config.ui_font_path).await;
    let palette = text::Palette::new(&config.palette_path).await;

    let mut app = App::new();
    app.add_plugins((
        bevy::DefaultPlugins,
        engine::Plugin,
        text::Plugin::new(font, palette),
        ui::Plugin::new(ui_font),
    ))
    .add_systems(Update, engine::stress_test);

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
