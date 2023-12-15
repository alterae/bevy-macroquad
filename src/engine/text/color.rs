use bevy::prelude::*;
use futures::executor;
use kdl::KdlDocument;

use crate::engine::{
    self, log,
    mq::{self, rand::ChooseRandom as _},
};

#[allow(unused)]
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black = 0,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,

    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightBlue,
    BrightCyan,
    BrightMagenta,
    BrightYellow,
    BrightWhite,

    Transparent,
}

#[allow(unused)]
impl Color {
    const VARIANTS: [Color; 17] = [
        Self::Black,
        Self::Red,
        Self::Green,
        Self::Blue,
        Self::Cyan,
        Self::Magenta,
        Self::Yellow,
        Self::White,
        Self::BrightBlack,
        Self::BrightRed,
        Self::BrightGreen,
        Self::BrightBlue,
        Self::BrightCyan,
        Self::BrightMagenta,
        Self::BrightYellow,
        Self::BrightWhite,
        Self::Transparent,
    ];

    pub fn random() -> Self {
        *Vec::from(Self::VARIANTS).choose().unwrap_or(&Self::Black)
    }
}

#[derive(Clone, Resource)]
pub struct Palette {
    colors: [mq::Color; 17],
}

impl Palette {
    pub fn new(path: &str) -> Self {
        let palette = executor::block_on(mq::load_string(path))
            .map_err(|e| log::error!("Error loading palette {path}: {e}"))
            .unwrap();
        let palette: KdlDocument = palette
            .parse()
            .map_err(|e| log::error!("Error parsing palette {path}: {e}"))
            .unwrap();

        Self {
            colors: [
                "black",
                "red",
                "green",
                "blue",
                "cyan",
                "magenta",
                "yellow",
                "white",
                "bright-black",
                "bright-red",
                "bright-green",
                "bright-blue",
                "bright-cyan",
                "bright-magenta",
                "bright-yellow",
                "bright-white",
                "transparent",
            ]
            .map(|c| {
                if c != "transparent" {
                    palette
                        .get_args(c)
                        .iter()
                        .map(|arg| arg.as_i64().unwrap() as u8)
                        .collect::<Vec<u8>>()
                } else {
                    vec![0, 0, 0, 0]
                }
            })
            .map(|args| [args[0], args[1], args[2], *args.get(3).unwrap_or(&255)].into()),
        }
    }
}

impl std::ops::Index<Color> for Palette {
    type Output = mq::Color;

    fn index(&self, color: Color) -> &Self::Output {
        &self.colors[color as usize]
    }
}

impl FromWorld for Palette {
    fn from_world(world: &mut World) -> Self {
        let config = world.resource::<engine::Config>();

        Self::new(&config.palette_path)
    }
}
