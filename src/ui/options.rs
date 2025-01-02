use egui::IconData;
use eframe::icon_data;
use crate::ui::consts;

pub fn set_options() -> eframe::NativeOptions{
    eframe::NativeOptions{
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([consts::VIEWPORT_WIDTH, consts::VIEWPORT_HEIGHT])
        .with_max_inner_size([consts::VIEWPORT_WIDTH, consts::VIEWPORT_HEIGHT])
        .with_min_inner_size([consts::VIEWPORT_WIDTH, consts::VIEWPORT_HEIGHT])
        .with_resizable(false)
        .with_maximize_button(false)
        .with_minimize_button(true)
        .with_icon(
            match icon_data::from_png_bytes(include_bytes!("./icons/stopwatch.png")) {
                Ok(icon) => icon,
                Err(err) => {
                    eprintln!("Failed to load icon: {}", err);
                    IconData{
                        ..Default::default()
                    } 
                }
            },
        ),
        
    centered: true,
    ..Default::default()
    }
}