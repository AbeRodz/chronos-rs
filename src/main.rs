mod app;
mod ui;

use egui::{global_theme_preference_buttons,Color32};
use crate::app::TimeApp;
use std::time::{SystemTime, UNIX_EPOCH,Duration};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "chronos-rs",
        ui::options::set_options(),
        Box::new(|cc| Ok(Box::new(TimeApp::new(cc)))),
    )
}

impl eframe::App for TimeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Current time
        let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

        let local_time_seconds = now.as_secs() + 6 * 3600;// adjusted due to timezone
        let arg_time_seconds = now.as_secs() + 9 * 3600;

        let current_local_time = format!("{:02}:{:02}:{:02}", local_time_seconds / 3600 % 24, local_time_seconds / 60 % 60, local_time_seconds % 60);
        let arg_local_time = format!("{:02}:{:02}:{:02}", arg_time_seconds / 3600 % 24, arg_time_seconds / 60 % 60, arg_time_seconds % 60);
        
        // Stopwatch elapsed time 
        if self.running {
            if let Some(start_time) = self.stopwatch_start {
                self.elapsed_time = start_time.elapsed();
            }
        }

        self.should_trigger_alarm();

        let seconds = self.elapsed_time.as_secs();
        let millis = self.elapsed_time.subsec_millis();
        let formatted_time = format!("{:02}:{:02}.{:03}", seconds / 60, seconds % 60, millis);
        
        
        egui::CentralPanel::default().show(ctx, |ui| {
            global_theme_preference_buttons(ui);
            ui.separator();
            ui.heading("Current Time");
            ui.label(egui::RichText::new("Local Time").text_style(ui::styles::heading2()));
            ui.label(&current_local_time);

            ui.label(egui::RichText::new("Argentina Time").text_style(ui::styles::heading2()));
            ui.label(&arg_local_time);

            ui.separator();
            ui.heading("Stopwatch");
            ui.label(&formatted_time);
            
            ui.horizontal(|ui|{
                ui.set_max_size([200.0,10.0].into());
                if let Some(target) = self.target_duration {
                    let progress = self.elapsed_time.as_secs_f32() / target.as_secs_f32();
                    ui.add(egui::ProgressBar::new(progress.clamp(0.0, 1.0)).text(format!("{:.0}%", progress * 100.0)));
                }
            });
            // Buttons for controlling the stopwatch
            ui.horizontal(|ui| {
                ui.set_max_size([200.0,10.0].into());
                ui.label("Set timer (minutes):");
                ui.text_edit_singleline(&mut self.minutes_input);

                if ui.button("Set").clicked() {
                    if let Ok(minutes) = self.minutes_input.parse::<u64>() {
                        self.target_duration = Some(Duration::from_secs(minutes * 60));
                        self.alarm_triggered = false; // Reset alarm state
                    }
                }
            });

            if self.alarm_triggered {
                ctx.request_repaint(); 
                ui.label(egui::RichText::new("Time's up!").color(Color32::RED));
                self.send_notification();
                self.stop_stopwatch();
                self.alarm_triggered = false;
            }

            // Stopwatch control buttons
            ui.horizontal(|ui| {
                if ui.button("Start").clicked() && !self.running {
                    self.start_stopwatch();
                }
                if ui.button("Stop").clicked() && self.running {
                    self.stop_stopwatch();
                }
                if ui.button("Reset").clicked() {
                    self.reset_stopwatch();
                }
            });

        });
        egui::SidePanel::right("presets_panel")
        .max_width(250.0)
        .min_width(120.0)
        .show(ctx, |ui| {
            ui.heading("Presets");
            ui.separator();
            let presets = [1,5,10,15,20,30];
            for &preset in &presets{

                let is_selected = self.selected_preset == Some(preset);

                if ui
                    .add(egui::Button::new(format!("{} min",preset))
                    .rounding(4.0)
                    .fill(if is_selected{egui::Color32::DARK_GREEN} else {egui::Color32::DARK_GRAY}))
                    .clicked()
                    {
                        self.selected_preset = Some(preset);
                        self.set_preset(preset);
                    }
            }
        });

        ctx.request_repaint();
    }
}
