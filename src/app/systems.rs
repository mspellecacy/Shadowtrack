use crate::app::rng::RandomSource;
use crate::app::state::{ShadowtrackData, TurnEntry};

pub fn log_event(data: &mut ShadowtrackData, event_desc: &str) {
    match data.event_log.iter_mut().find(|e| e.turn == data.turn) {
        // Need a new TurnEntry for the log.
        None => {
            let mut entry = TurnEntry::default();
            entry.turn = data.turn;
            entry.events.push(event_desc.to_string());

            data.event_log.push(entry);
        }
        // Update existing TurnEntry in log.
        Some(entry) => {
            entry.events.push(event_desc.to_string());
        }
    }
}

pub fn process_light_burn(data: &mut ShadowtrackData, rng: &mut impl RandomSource) {
    for light in &mut data.light_sources {
        light.minutes_remaining = light.minutes_remaining.saturating_sub(10);
        let roll = rng.roll_range(1, 6);
        light.last_roll = Some(roll as u8);
        if roll <= 2 {
            light.minutes_remaining = light.minutes_remaining.saturating_sub(10);
        }
    }
}

/// Selects a torch event from the event table
pub fn roll_light_event(data: &mut ShadowtrackData, rng: &mut impl RandomSource) {
    if let Some(event) = rng.choose(&data.torch_event_table) {
        let event_log_entry = format!("{}", event);
        log_event(data, event_log_entry.as_str());
    }
}

/// Rolls for a random encounter using 1d6 logic
pub fn roll_encounter(
    mut data: &mut ShadowtrackData,
    rng: &mut impl RandomSource,
    forced_encounter: bool,
) {
    if forced_encounter || rng.roll_range(1, 6) == 1 {
        let log_entry = if let Some(encounter) = rng.choose(&*data.encounter_table.clone()) {
            format!("!ENCOUNTER! - {}", encounter)
        } else {
            "[Error] Encounter table empty!".to_string()
        };

        log_event(&mut data, log_entry.as_str());
    } else {
        log_event(&mut data, "No encounter");
    }
}
