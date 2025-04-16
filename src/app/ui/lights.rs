use crate::app::rng::DefaultRandomSource;
use crate::app::state::ShadowtrackData;
use crate::app::systems::roll_light_event;
use crate::app::{
    state::{LightSource, LightSourceType},
    ShadowtrackApp,
};
use egui::{Color32, RichText, Ui};

fn new_light_type_label(app: &mut ShadowtrackData) -> String {
    match &app.new_light_type {
        LightSourceType::Torch => "Torch".to_string(),
        LightSourceType::Lantern => "Lantern".to_string(),
        LightSourceType::Spell(name) => format!("Spell ({})", name),
    }
}

pub fn draw_torch_ui(app: &mut ShadowtrackApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Active Light Sources");
        ui.separator();
        if ui.button("➕ Add").clicked() {
            app.show_add_light_modal = true;
        }
    });
    ui.separator();
    if app.data.light_sources.is_empty() {
        ui.label(
            RichText::new("No active light sources")
                .color(Color32::YELLOW)
                .size(18_f32),
        );
    } else {
        for light in &mut app.data.light_sources {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(&light.label)
                        .color(Color32::WHITE)
                        .size(18_f32),
                );
                ui.label(&light.light_type.to_string());
                ui.label(
                    RichText::new(format!("({}ft)", light.radius_feet.to_string()))
                        .color(Color32::WHITE)
                        .size(12_f32),
                );
                ui.label(format!("Time left: {} min", light.minutes_remaining));
            });
            if let Some(roll) = light.last_roll {
                ui.label(format!("Last Burn Roll: {}", roll));
            }
        }
    }
    ui.separator();
    ui.horizontal(|ui| {
        ui.collapsing("🎲 Edit Torch Event Table", |ui| {
            for entry in &mut app.data.torch_event_table {
                ui.text_edit_singleline(entry);
            }
            if ui.button("Add Torch Event").clicked() {
                app.data.torch_event_table.push("New Torch Event".into());
            }
        });
        if ui.button("Roll Torch Event").clicked() {
            roll_light_event(&mut app.data, &mut DefaultRandomSource)
        };
    });

    if app.show_add_light_modal {
        egui::Window::new("Add Light Source")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Owner:");
                    ui.text_edit_singleline(&mut app.data.new_light_label);
                });
                ui.horizontal(|ui| {
                    ui.label("Radius:");
                    ui.add(egui::DragValue::new(&mut app.data.new_light_range).range(5..=100));
                });

                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_salt("light_type")
                        .selected_text(new_light_type_label(&mut app.data))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut app.data.new_light_type,
                                LightSourceType::Torch,
                                "Torch",
                            );
                            ui.selectable_value(
                                &mut app.data.new_light_type,
                                LightSourceType::Lantern,
                                "Lantern",
                            );
                            ui.selectable_value(
                                &mut app.data.new_light_type,
                                LightSourceType::Spell(String::new()),
                                "Spell",
                            );
                        });
                });

                if let LightSourceType::Spell(name) = &mut app.data.new_light_type {
                    ui.horizontal(|ui| {
                        ui.label("Spell:");
                        ui.text_edit_singleline(name);
                    });
                }
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Add").clicked() {
                        if !app.data.new_light_label.is_empty() {
                            app.data.light_sources.push(LightSource {
                                label: app.data.new_light_label.clone(),
                                radius_feet: app.data.new_light_range,
                                light_type: app.data.new_light_type.clone(),
                                minutes_remaining: 60,
                                last_roll: None,
                            });
                            app.data.new_light_label.clear();
                            app.data.new_light_range = 20;
                            app.data.new_light_type = LightSourceType::Torch;
                            app.show_add_light_modal = false;
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        app.show_add_light_modal = false;
                    }
                });
            });
    }
}
