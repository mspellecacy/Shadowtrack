use eframe::epaint::Color32;
use crate::app::ShadowtrackApp;
use egui::Ui;

pub fn draw_log_ui(app: &ShadowtrackApp, ui: &mut Ui) {
    egui::ScrollArea::vertical()
        .show(ui, |ui| {
            for mut entry in app.data.event_log.iter().rev() {
                // Turn Label
                ui.label(egui::RichText::from(format!("📜 Turn: {}", entry.turn))
                    .size(18_f32)
                    .color(Color32::LIGHT_YELLOW)
                    .strong()
                );
                for event in entry.events.iter().rev() {
                    ui.label(format!("   {}", event));
                }
            }
            if app.data.event_log.is_empty() {
                ui.label("No events logged");
            }
        });
}
