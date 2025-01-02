use egui::{style::Selection,FontFamily,FontId, Style, TextStyle, Color32, Stroke, Theme};
use std::collections::BTreeMap;

pub fn configure_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (heading2(), FontId::new(24.0, Proportional)),
        (TextStyle::Body, FontId::new(20.0, Proportional)),
        (TextStyle::Monospace, FontId::new(20.0, Monospace)),
        (TextStyle::Button, FontId::new(14.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
    setup_custom_style(ctx);
}

pub fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}


fn setup_custom_style(ctx: &egui::Context) {
    ctx.style_mut_of(Theme::Light, use_light_green_accent);
    ctx.style_mut_of(Theme::Dark, use_dark_purple_accent);
}

fn use_dark_purple_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(202, 135, 227);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(234, 208, 244);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(105, 67, 119),
        stroke: Stroke::new(1.0, Color32::from_rgb(234, 208, 244)),
    };
}

fn use_light_green_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(18, 180, 85);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(28, 92, 48);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(157, 218, 169),
        stroke: Stroke::new(1.0, Color32::from_rgb(28, 92, 48)),
    };
}