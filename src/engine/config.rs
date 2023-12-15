use kdl::{KdlDocument, KdlValue};

use crate::engine::mq;

pub struct Config {
    pub font_path: String,
    pub palette_path: String,
}

impl Config {
    pub async fn new() -> Self {
        let config = mq::load_string("init.kdl").await.unwrap_or_default();
        let config: KdlDocument = config.parse().unwrap();

        let font = config
            .get_arg("font")
            .and_then(KdlValue::as_string)
            .unwrap_or("curses_800x600.png");

        let palette = config
            .get_arg("palette")
            .and_then(KdlValue::as_string)
            .unwrap_or("taffer.kdl");

        Self {
            font_path: format!("assets/fonts/{font}"),
            palette_path: format!("assets/colors/{palette}"),
        }
    }
}
