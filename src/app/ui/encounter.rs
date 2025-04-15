use crate::app::rng::DefaultRandomSource;
use crate::app::systems::roll_encounter;
use crate::app::ShadowtrackApp;
use egui::Ui;

pub fn draw_encounter_ui(app: &mut ShadowtrackApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.collapsing("🎲 Edit Encounter Table", |ui| {
            for entry in &mut app.data.encounter_table {
                ui.text_edit_singleline(entry);
            }
            if ui.button("Add Encounter").clicked() {
                app.data.encounter_table.push("New Encounter".into());
            }
        });
        if ui.button("Roll Random Encounter").clicked() {
            roll_encounter(&mut app.data, &mut DefaultRandomSource, true)
        }
    });
}
