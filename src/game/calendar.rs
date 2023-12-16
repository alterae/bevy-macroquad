use bevy::prelude::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.))
            .init_resource::<Date>()
            .insert_resource(TimeScale(1))
            .add_systems(FixedUpdate, tick);
    }
}

#[derive(Default, Resource)]
pub struct Date {
    total_seconds: i64,
}

impl Date {
    fn tick(&mut self, seconds: i64) {
        self.total_seconds += seconds;
    }

    pub fn total_seconds(&self) -> i64 {
        self.total_seconds
    }

    pub fn total_minutes(&self) -> i64 {
        self.total_seconds() / 60
    }

    pub fn total_hours(&self) -> i64 {
        self.total_minutes() / 60
    }

    pub fn total_sols(&self) -> i64 {
        self.total_hours() / 24
    }

    pub fn total_tensols(&self) -> i64 {
        self.total_sols() / 10
    }

    pub fn total_seasons(&self) -> i64 {
        self.total_tensols() / 16
    }

    pub fn total_years(&self) -> i64 {
        self.total_seasons() / 4
    }

    pub fn current_year(&self) -> i64 {
        self.total_years()
    }

    pub fn current_season(&self) -> i64 {
        (self.total_seasons() - self.total_years() * 4) + 1
    }

    pub fn current_tensol(&self) -> i64 {
        (self.total_tensols() - self.total_seasons() * 16) + 1
    }

    pub fn current_sol(&self) -> i64 {
        (self.total_sols() - self.total_tensols() * 10) + 1
    }

    pub fn current_hour(&self) -> i64 {
        self.total_hours() - self.total_sols() * 24
    }

    pub fn current_minute(&self) -> i64 {
        self.total_minutes() - self.total_hours() * 60
    }

    pub fn current_second(&self) -> i64 {
        self.total_seconds() - self.total_minutes() * 60
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minute = self.current_minute();
        let hour = self.current_hour();
        let sol = self.current_sol();
        let tensol = self.current_tensol();
        let season = match self.current_season() {
            1 => "First Ascent",
            2 => "Second Ascent",
            3 => "First Descent",
            4 => "Second Descent",
            _ => "INVALID SEASON",
        };
        let year = self.current_year();

        write!(
            f,
            "{hour:02}:{minute:02}, {tensol}-{sol} {season} {year} AL"
        )
    }
}

fn tick(mut date: ResMut<Date>, time_scale: Res<TimeScale>) {
    date.tick(time_scale.0);
}

#[derive(Deref, Resource)]
pub struct TimeScale(i64);
