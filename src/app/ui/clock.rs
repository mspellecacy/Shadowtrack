use crate::app::ShadowtrackApp;
use egui::text::LayoutJob;
use egui::{Align, Color32, FontSelection, RichText, Style, Ui};
use num_integer::Integer;

pub fn draw_clock_controls(app: &mut ShadowtrackApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let button_label = if app.clock_running {
            "Stop Clock"
        } else {
            "Start Clock"
        };

        if ui.button(button_label).clicked() {
            app.toggle_clock()
        }

        if ui.button("Reset Clock").clicked() {
            app.data.clock_elapsed = 0;
            app.clock_running = false;
            app.data.next_process_minutes = None;
        }

        ui.separator();
        if ui.button("+1 min").clicked() {
            app.advance_clock_secs(60);
        }
        if ui.button("+5 min").clicked() {
            app.advance_clock_secs(300);
        }
        if ui.button("+10 min").clicked() {
            app.advance_clock_secs(600);
        }
    });
}

pub fn draw_clock(app: &mut ShadowtrackApp, ui: &mut Ui) {
    let style = Style::default();
    let mut layout = LayoutJob::default();
    let (minutes, seconds) = app.data.clock_elapsed.div_rem(&60);

    // Timer Label
    RichText::from("Game Time: ")
        .size(24_f32)
        .color(Color32::RED)
        .strong()
        .append_to(&mut layout, &style, FontSelection::Default, Align::Center);
    // Timer
    RichText::from(format!("{minutes:02}:{seconds:02}"))
        .size(24_f32)
        .color(Color32::LIGHT_RED)
        .strong()
        .append_to(&mut layout, &style, FontSelection::Default, Align::Center);

    ui.label(layout);
}
