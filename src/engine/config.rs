use futures::executor;
use kdl::{KdlDocument, KdlValue};

use crate::engine::mq;

pub struct Config {
    pub font_path: String,
    pub ui_font_path: String,
    pub palette_path: String,
}

impl Config {
    pub fn new() -> Self {
        let config = executor::block_on(mq::load_string("init.kdl")).unwrap_or_default();
        let config: KdlDocument = config.parse().unwrap();

        let font = config
            .get_arg("font")
            .and_then(KdlValue::as_string)
            .unwrap_or("curses_800x600.png");

        let ui_font = config
            .get_arg("ui-font")
            .and_then(KdlValue::as_string)
            .unwrap_or({
                eprintln!("No UI font specified. Defaulting to main font.");
                font
            });

        let palette = config
            .get_arg("palette")
            .and_then(KdlValue::as_string)
            .unwrap_or("taffer.kdl");

        Self {
            font_path: format!("assets/fonts/{font}"),
            ui_font_path: format!("assets/fonts/{ui_font}"),
            palette_path: format!("assets/colors/{palette}"),
        }
    }
}
