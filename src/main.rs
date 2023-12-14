use bevy::{app::prelude::*, ecs::prelude::*, log};
use macroquad::prelude::*;

use text::Color;

mod config;
mod math;
mod text;

#[macroquad::main(conf)]
async fn main() {
    let config = config::Config::new().await;

    let font = text::Font::new(&config.font_path).await;
    let palette = text::Palette::new(&config.palette_path).await;

    let mut app = App::new();
    app.add_plugins((bevy::DefaultPlugins, text::Plugin::new(font, palette)))
        .add_systems(Update, (fps_display, stress_test.before(fps_display)));

    loop {
        if is_key_down(KeyCode::LeftSuper) && is_key_pressed(KeyCode::Q) {
            log::info!("Exiting!");
            break;
        }

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

fn fps_display(mut console: ResMut<text::Console>) {
    let fps = macroquad::time::get_fps();

    console.put_str(
        (1, 1),
        &format!("FPS: {fps}"),
        Color::BrightWhite,
        Color::Black,
    );
}

fn stress_test(mut console: ResMut<text::Console>) {
    for x in 0..console.width {
        for y in 0..console.height {
            console.put_char((x, y), '!', Color::random(), Color::random());
        }
    }
}
