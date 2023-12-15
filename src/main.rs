use bevy::prelude::*;

use engine::prelude::*;

mod engine;
mod game;

#[macroquad::main(conf)]
async fn main() {
    let mut app = App::new();
    app.add_plugins((bevy::DefaultPlugins, engine::Plugin, game::Plugin));

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
