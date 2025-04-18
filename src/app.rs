use crate::app::rng::DefaultRandomSource;
use crate::app::save::{load_from_file, save_to_file};
use crate::app::state::ShadowtrackData;
use crate::app::systems::{process_light_burn, roll_encounter, roll_light_event};
use crate::app::ui::clock::{draw_clock, draw_clock_controls};
use crate::app::ui::log::draw_log_ui;
use crate::app::ui::{encounter::draw_encounter_ui, lights::draw_torch_ui};
use eframe::{egui, App};
use log::debug;
use num_integer::Integer;
use std::time::{Duration, Instant};

pub mod rng;
pub mod save;
pub mod state;
pub mod systems;
pub mod ui;

pub struct ShadowtrackApp {
    pub show_add_light_modal: bool,
    pub data: ShadowtrackData,
    pub clock_running: bool,
    pub last_tick: Instant,
    pub rng: DefaultRandomSource,
}

impl Default for ShadowtrackApp {
    fn default() -> Self {
        Self {
            show_add_light_modal: false,
            data: ShadowtrackData::default(),
            clock_running: false,
            last_tick: Instant::now(),
            rng: DefaultRandomSource,
        }
    }
}

impl App for ShadowtrackApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_clock_tick();
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Game State", |ui| {
                    if ui.button("Save").clicked() {
                        ui.close_menu();
                        if let Err(e) = save_to_file(&self.data) {
                            eprintln!("Failed to save the game: {}", e);
                        }
                    }

                    if ui.button("Load").clicked() {
                        ui.close_menu();
                        match load_from_file() {
                            Ok(data) => self.data = data,
                            Err(e) => eprintln!("Failed to load game: {}", e),
                        }
                    }

                    if ui.button("Reset").clicked() {
                        ui.close_menu();
                        self.reset();
                    }
                });
                ui.separator();
                draw_clock_controls(self, ui);
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(250.0)
            .width_range(180.0..=500.0)
            .show(&ctx, |ui| {
                ctx.request_repaint();
                draw_clock(self, ui);
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    draw_log_ui(self, ui);
                });
            });
        egui::CentralPanel::default().show(&ctx, |ui| {
            draw_torch_ui(self, ui);
            draw_encounter_ui(self, ui);
        });
    }
}

impl ShadowtrackApp {
    fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn toggle_clock(&mut self) {
        self.clock_running = !self.clock_running;
    }

    pub fn advance_clock_secs(&mut self, secs: u64) {
        self.data.clock_elapsed += secs;
        debug!(
            "Advanced Clock: {secs} Seconds\nNew Value: {}",
            self.data.clock_elapsed
        );
    }

    #[inline]
    pub(crate) fn should_tick(&self, now: Instant) -> Option<u64> {
        let elapsed = now.duration_since(self.last_tick);
        if elapsed >= Duration::from_secs(1) {
            Some(elapsed.as_secs())
        } else {
            None
        }
    }

    pub(crate) fn handle_clock_tick(&mut self) {
        // Tick every second
        if let Some(secs) = self.should_tick(Instant::now()) {
            self.last_tick += Duration::from_secs(secs);

            if self.clock_running {
                // Advance game clock
                self.data.clock_elapsed += secs;
                let (elapsed_minutes, _seconds) = self.data.clock_elapsed.div_rem(&60);
                match self.data.next_process_minutes {
                    Some(next_minutes) if elapsed_minutes >= next_minutes => {
                        self.data.turn += 1;

                        process_light_burn(&mut self.data, &mut self.rng);
                        roll_light_event(&mut self.data, &mut self.rng);
                        roll_encounter(&mut self.data, &mut self.rng, false);

                        self.data.next_process_minutes =
                            Some(elapsed_minutes + self.data.process_interval_minutes);
                    }
                    None => {
                        // First activation of process timer, we need to set a value to check against.
                        // TODO: Implement user-configurable value for process interval.
                        self.data.next_process_minutes = Some(self.data.process_interval_minutes);
                    }
                    _ => { /* do nothing */ }
                }
            }
        }
    }
}
