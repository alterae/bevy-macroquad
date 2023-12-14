use bevy::ecs::prelude::*;

pub use Color::*;

#[allow(unused)]
#[derive(Clone, Copy)]
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

#[derive(Resource)]
pub struct Palette {
    colors: [macroquad::color::Color; 16],
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
    type Output = macroquad::color::Color;

    fn index(&self, color: Color) -> &Self::Output {
        &self.colors[color as usize]
    }
}
