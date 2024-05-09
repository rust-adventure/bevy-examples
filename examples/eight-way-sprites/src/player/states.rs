use bevy::prelude::*;

#[derive(Component, Clone)]
#[component(storage = "SparseSet")]
pub struct Idling {
    pub timer: Timer,
    pub frame: u32,
    pub total_frames: u32,
}

impl Default for Idling {
    fn default() -> Self {
        // 2 seconds to play entire idle animation
        let idle_frame_duration = 1. / 30.;
        let idle_frames = 60.;

        Self {
            timer: Timer::from_seconds(
                idle_frame_duration,
                TimerMode::Repeating,
            ),
            frame: default(),
            total_frames: idle_frames as u32,
        }
    }
}

#[derive(Component, Clone)]
#[component(storage = "SparseSet")]
pub struct Running {
    pub timer: Timer,
    pub frame: u32,
    pub total_frames: u32,
}

impl Default for Running {
    fn default() -> Self {
        // 2 seconds to play entire running animation
        let running_frame_duration = 1. / 48.;
        let running_frames = 48.;

        Self {
            timer: Timer::from_seconds(
                running_frame_duration,
                TimerMode::Repeating,
            ),
            frame: default(),
            total_frames: running_frames as u32,
        }
    }
}

#[derive(Component, Clone)]
#[component(storage = "SparseSet")]
pub struct Dashing {
    pub timer: Timer,
    pub frame: u32,
    pub total_frames: u32,
}

impl Default for Dashing {
    fn default() -> Self {
        // 1/5 == 200ms
        let total_dash_duration = 1. / 5.;
        let dash_frames = 24.;

        Self {
            timer: Timer::from_seconds(
                total_dash_duration / dash_frames,
                TimerMode::Repeating,
            ),
            frame: default(),
            total_frames: dash_frames as u32,
        }
    }
}
