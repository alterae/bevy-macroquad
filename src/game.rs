use bevy::prelude::*;

use crate::engine::{text::Color, ui};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.))
            .insert_resource(TimeScale(1))
            .init_resource::<Date>()
            .add_systems(FixedUpdate, tick_date)
            .add_systems(Update, display_date);
    }
}

#[derive(Default, Resource)]
struct Date {
    total_seconds: i64,
    total_minutes: i64,
    total_hours: i64,
    total_sols: i64,
    total_tensols: i64,
    total_seasons: i64,
    total_years: i64,
}

impl Date {
    fn tick(&mut self, seconds: i64) {
        self.total_seconds += seconds;
        self.total_minutes = self.total_seconds / 60;
        self.total_hours = self.total_minutes / 60;
        self.total_sols = self.total_hours / 24;
        self.total_tensols = self.total_sols / 10;
        self.total_seasons = self.total_sols / 167;
        self.total_years = self.total_seasons / 4;
    }
}

fn tick_date(mut date: ResMut<Date>, time_scale: Res<TimeScale>) {
    date.tick(time_scale.0);
}

#[derive(Deref, Resource)]
struct TimeScale(i64);

fn display_date(date: Res<Date>, mut ui: ResMut<ui::UI>) {
    let pos = (1_usize, ui.height - 2);

    let text = format!(
        "{} sec = {} min = {} hrs = {} sols = {} tensols = {} seasons = {} years",
        date.total_seconds,
        date.total_minutes,
        date.total_hours,
        date.total_sols,
        date.total_tensols,
        date.total_seasons,
        date.total_years
    );

    ui.put_str(pos, &text, Color::White, Color::Transparent);
}
