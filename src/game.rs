use bevy::prelude::*;

use crate::engine::prelude::*;

mod calendar;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(calendar::Plugin)
            .add_systems(Update, show_date);
    }
}

fn show_date(date: Res<calendar::Date>, mut ui: ResMut<ui::UI>) {
    let pos = (1, ui.height - 2);
    let text = format!("{}", *date);

    ui.put_str(pos, &text, BrightWhite, Transparent);
}
