use std::time::{Duration, Instant};

use ecs_macro::EntityComponent;

#[derive(EntityComponent)]
pub struct TimeDelta {
    last_time: Instant,
}

impl TimeDelta {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
        }
    }

    pub fn update_time_delta(&mut self) {
        self.last_time = Instant::now();
    }

    pub fn get_time_delta(&mut self) -> Duration {
        let current_time = Instant::now();
        let last_time = self.last_time;

        current_time.duration_since(last_time)
    }

    pub fn get_time_delta_nanos(&mut self) -> u128 {
        self.get_time_delta().as_nanos()
    }

    pub fn get_time_delta_micros(&mut self) -> u128 {
        self.get_time_delta().as_micros()
    }

    pub fn get_time_delta_millis(&mut self) -> u128 {
        self.get_time_delta().as_millis()
    }

    pub fn get_time_delta_sec(&mut self) -> f32 {
        self.get_time_delta().as_secs_f32()
    }
}
