use crate::app::TimeApp;
use std::time::{Instant, Duration};
use notify_rust::Notification;

impl TimeApp {

    pub fn start_stopwatch(&mut self) {
        self.stopwatch_start = Some(Instant::now() - self.elapsed_time);
        self.running = true;
    }

    pub fn stop_stopwatch(&mut self) {
        self.running = false;
    }

    pub fn reset_stopwatch(&mut self) {
        self.stopwatch_start = None;
        self.elapsed_time = Duration::default();
        self.running = false;
        self.alarm_triggered = false;
    }
    pub fn set_preset(&mut self, minutes: u64) {
        self.reset_stopwatch();
        self.target_duration = Some(Duration::from_secs(minutes * 60));
        self.start_stopwatch();
    }
    pub fn send_notification(&self) {
        Notification::new()
            .summary("Time's Up!")
            .body("The timer has completed.")
            .show().unwrap();
    }
    pub fn should_trigger_alarm(&mut self){
        if let Some(target) = self.target_duration {
            if self.elapsed_time >= target && !self.alarm_triggered {
                self.alarm_triggered = true;
            }
        }
    }
}