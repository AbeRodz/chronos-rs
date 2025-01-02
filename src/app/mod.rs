
pub mod stopwatch;
use std::time::{Duration, Instant};
use crate::ui::styles::configure_styles;
pub struct TimeApp {
    pub selected_preset: Option<u64>,
    pub stopwatch_start: Option<Instant>,
    pub elapsed_time: Duration,
    pub running: bool,
    pub target_duration: Option<Duration>, // Target duration for the alarm
    pub alarm_triggered: bool,             // Indicates if the alarm has triggered
    pub minutes_input: String,             // User input for minutes
}

impl TimeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_styles(&cc.egui_ctx);
        Self {
            selected_preset: None,
            stopwatch_start: None,
            elapsed_time: Duration::default(),
            running: false,
            target_duration: None,
            alarm_triggered: false,
            minutes_input: String::new(),
        }
    }
}