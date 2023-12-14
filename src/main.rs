use bevy::{app::prelude::*, ecs::prelude::*, log};
use macroquad::prelude::*;

mod color;
mod text;

#[macroquad::main(conf)]
async fn main() {
    let font = text::Font::new("assets/fonts/curses_800x600.png").await;

    let mut app = App::new();
    app.add_plugins((bevy::DefaultPlugins, text::Plugin::new(font)))
        .add_systems(Update, fps_display);

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
        color::BrightWhite,
        color::Black,
    );
}
