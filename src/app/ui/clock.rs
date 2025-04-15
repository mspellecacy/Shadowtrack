use crate::app::ShadowtrackApp;
use egui::{Align, Color32, FontSelection, RichText, Style, Ui, Widget};
use egui::text::LayoutJob;

pub fn draw_clock_controls(app: &mut ShadowtrackApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let button_label = if app.clock_running {
            "Stop Clock"
        } else {
            "Start Clock"
        };

        if ui.button(button_label).clicked() {
            app.clock_running = !app.clock_running;
        }

        if ui.button("Reset Clock").clicked() {
            app.data.clock_elapsed = 0;
            app.clock_running = false;
            app.last_processed_minutes = 0;
        }

        ui.separator();
        if ui.button("+5 min").clicked() {
            app.data.clock_elapsed += 5 * 60;
        }
    });
}

pub fn draw_clock(app: &mut ShadowtrackApp, ui: &mut Ui) {
    let style = Style::default();
    let mut layout = LayoutJob::default();
    let minutes = app.data.clock_elapsed / 60;
    let seconds = app.data.clock_elapsed % 60;
    
    // Timer Label
    RichText::from("Game Time: ")
        .size(24_f32)
        .color(Color32::LIGHT_BLUE)
        .strong()
        .append_to(&mut layout, &style, FontSelection::Default, Align::TOP);
    // Timer
    RichText::from(format!("{minutes:02}:{seconds:02}"))
        .size(24_f32)
        .color(Color32::DARK_RED)
        .strong()
        .append_to(&mut layout, &style, FontSelection::Default, Align::TOP);
    
    ui.label(layout);
    
}
