use bevy::prelude::*;

use crate::engine::{log, math, text};

#[derive(Clone, Copy, Component)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl math::Vec2 for Position {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl math::Vec3 for Position {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn z(&self) -> i32 {
        self.z
    }
}

#[derive(Component, Deref)]
pub struct Sprite(text::Cell);

impl Sprite {
    fn new(glyph: char, fg: text::Color, bg: text::Color) -> Self {
        Self(text::Cell {
            glyph: glyph as u8,
            fg,
            bg,
        })
    }
}

pub fn spawn(mut commands: Commands, console: Res<text::Console>) {
    log::info!("Spawning player...");
    commands.spawn((
        Position {
            x: console.width as i32 / 2,
            y: console.height as i32 / 2,
            z: 0,
        },
        Sprite::new('@', text::Color::BrightYellow, text::Color::Black),
    ));
}

pub fn render(mut console: ResMut<text::Console>, query: Query<(&Position, &Sprite)>) {
    for (pos, sprite) in &query {
        console.put_char(*pos, sprite.glyph, sprite.fg, sprite.bg);
    }
}
