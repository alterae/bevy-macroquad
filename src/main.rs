use bevy::prelude::*;

use engine::mq;

mod engine;

#[macroquad::main(conf)]
async fn main() {
    let mut app = App::new();
    app.add_plugins((bevy::DefaultPlugins, engine::Plugin))
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
