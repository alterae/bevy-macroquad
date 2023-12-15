use bevy::prelude::*;

use crate::engine::{log, mq};

pub struct Plugin {
    font: Font,
}

impl Plugin {
    pub fn new(font: super::Font) -> Self {
        Self { font: Font(font) }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.font.clone())
            .init_resource::<UI>()
            .add_systems(Startup, init)
            .add_systems(PostUpdate, draw.after(super::draw));
    }
}

#[derive(Default, Deref, DerefMut, Resource)]
pub struct UI(super::Console);

#[derive(Clone, Deref, Resource)]
struct Font(super::Font);

fn init(mut ui: ResMut<UI>, font: Res<Font>) {
    ui.clear(&font);

    log::info!(
        "Setting initial UI dimensions to {} x {}.",
        ui.width,
        ui.height
    );
    log::info!("UI font: {} ({} x {})", font.path, font.width, font.height);
}

fn draw(mut ui: ResMut<UI>, font: Res<Font>, palette: Res<super::Palette>) {
    for (idx, cell) in ui.buffer.iter().enumerate() {
        if let Some(cell) = cell {
            let (x, y) = ui.idx_to_pos(idx);

            mq::draw_texture_ex(
                &font.texture,
                x as f32 * font.width,
                y as f32 * font.height,
                palette[cell.bg],
                mq::DrawTextureParams {
                    source: Some(font.get_rect(219)),
                    ..Default::default()
                },
            );

            if cell.fg != cell.bg {
                mq::draw_texture_ex(
                    &font.texture,
                    x as f32 * font.width,
                    y as f32 * font.height,
                    palette[cell.fg],
                    mq::DrawTextureParams {
                        source: Some(font.get_rect(cell.glyph)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    ui.clear(&font);
}
