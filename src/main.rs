use bevy::{app, ecs::system, log};
use macroquad::input;

mod text;

#[macroquad::main(conf)]
async fn main() {
    let font = text::Font::new("assets/fonts/curses_800x600.png").await;

    let mut app = app::App::new();
    app.add_plugins((bevy::DefaultPlugins, text::Plugin::new(font)))
        .add_systems(app::Update, hello);

    loop {
        if input::is_key_down(input::KeyCode::LeftSuper) && input::is_key_pressed(input::KeyCode::Q)
        {
            log::info!("Exiting!");
            break;
        }

        app.update();
        macroquad::window::next_frame().await;
    }
}

fn conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Hello, world!".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

fn hello(mut console: system::ResMut<text::Console>) {
    let fps = macroquad::time::get_fps();

    console.put_str((1, 1), "\x03 Hello, world!");
    console.put_str((1, 2), &format!("FPS: {fps}"))
}
