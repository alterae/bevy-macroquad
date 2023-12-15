use bevy::prelude::*;

pub use bevy::log;
pub use macroquad::prelude as mq;

pub use config::Config;
pub use text::ui;

pub mod config;
pub mod math;
pub mod text;

fn fps_display(mut ui: ResMut<ui::UI>) {
    let fps = macroquad::time::get_fps();

    ui.put_str(
        (1, 1),
        &format!("FPS: {fps}"),
        text::Color::BrightWhite,
        text::Color::Black,
    );
}

fn command_q() {
    #[cfg(target_os = "macos")]
    if mq::is_key_down(mq::KeyCode::LeftSuper) && mq::is_key_pressed(mq::KeyCode::Q) {
        log::info!("Command-Q received. Requesting quit...");
        mq::miniquad::window::request_quit();
    }
}

pub fn _stress_test(mut console: ResMut<text::Console>) {
    for x in 0..console.width {
        for y in 0..console.height {
            console.put_char(
                (x, y),
                mq::rand::rand() as u8,
                text::Color::random(),
                text::Color::random(),
            );
        }
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Config>()
            .add_plugins(text::Plugin)
            .add_plugins(ui::Plugin)
            .add_systems(Update, command_q)
            .add_systems(PostUpdate, fps_display.before(text::draw));
    }
}
