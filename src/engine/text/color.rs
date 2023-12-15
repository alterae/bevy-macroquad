use bevy::prelude::*;
use kdl::KdlDocument;

use crate::engine::mq::{self, rand::ChooseRandom as _};

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
}

#[allow(unused)]
impl Color {
    const VARIANTS: [Color; 16] = [
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
    ];

    pub fn random() -> Self {
        *Vec::from(Self::VARIANTS).choose().unwrap_or(&Self::Black)
    }
}

#[derive(Clone, Resource)]
pub struct Palette {
    colors: [mq::Color; 16],
}

impl Palette {
    pub async fn new(path: &str) -> Self {
        let palette = mq::load_string(path).await.unwrap();
        let palette: KdlDocument = palette.parse().unwrap();

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
            ]
            .map(|c| {
                palette
                    .get_args(c)
                    .iter()
                    .map(|arg| arg.as_i64().unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .map(|args| [args[0], args[1], args[2], 255].into()),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            colors: [
                [25, 25, 25, 255],    // black
                [187, 34, 34, 255],   // red
                [85, 153, 85, 255],   // green
                [0, 99, 177, 255],    // blue
                [77, 158, 161, 255],  // cyan
                [170, 51, 119, 255],  // magenta
                [140, 117, 100, 255], // yellow
                [157, 174, 178, 255], // white
                [91, 112, 117, 255],  // bright black
                [246, 114, 128, 255], // bright red
                [153, 238, 119, 255], // bright green
                [48, 165, 255, 255],  // bright blue
                [128, 212, 215, 255], // bright cyan
                [255, 136, 187, 255], // bright magenta
                [255, 255, 102, 255], // bright yellow
                [255, 250, 232, 255], // bright white
            ]
            .map(<[u8; 4]>::into),
        }
    }
}

impl std::ops::Index<Color> for Palette {
    type Output = mq::Color;

    fn index(&self, color: Color) -> &Self::Output {
        &self.colors[color as usize]
    }
}
