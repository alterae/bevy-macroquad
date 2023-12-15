use bevy::prelude::*;
use futures::executor;
use kdl::{KdlDocument, KdlValue};

use crate::engine::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub font_path: String,
    pub ui_font_path: String,
    pub palette_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let config = executor::block_on(mq::load_string("init.kdl")).unwrap_or_else(|e| {
            log::error!("Error loading config file `init.kdl`: {e}");
            default()
        });
        let config: KdlDocument = config.parse().unwrap_or_else(|e| {
            log::error!("Error parsing config file `init.kdl`: {e}");
            default()
        });

        let font = config
            .get_arg("font")
            .and_then(KdlValue::as_string)
            .unwrap_or_else(|| {
                log::warn!("No main font specified. Defaulting to `curses_800x600.png`.");
                "curses_800x600.png"
            });

        let ui_font = config
            .get_arg("ui-font")
            .and_then(KdlValue::as_string)
            .unwrap_or_else(|| {
                log::warn!("No UI font specified. Defaulting to main font.");
                font
            });

        let palette = config
            .get_arg("palette")
            .and_then(KdlValue::as_string)
            .unwrap_or_else(|| {
                log::warn!("No color palette specified. Defaulting to `v50.kdl`.");
                "v50.kdl"
            });

        Self {
            font_path: format!("assets/fonts/{font}"),
            ui_font_path: format!("assets/fonts/{ui_font}"),
            palette_path: format!("assets/colors/{palette}"),
        }
    }
}
