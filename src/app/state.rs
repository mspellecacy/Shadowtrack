use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

const DEFAULT_ENCOUNTER_TABLE: &[&str] = &[
    "Goblin scouts",
    "Skeleton patrol",
    "Oozing slime",
    "Lost adventurer",
    "Swarm of bats",
    "Mimic chest",
];

const DEFAULT_TORCH_EVENTS_TABLE: &[&str] = &[
    "You hear a distant moan in the dark...",
    "A gust of wind threatens to blow out a torch.",
    "You stumble over loose stones, nearly falling.",
    "The smell of sulfur fills the air.",
    "Whispers echo from nowhere.",
    "A rat darts between your feet.",
];

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum LightSourceType {
    Torch,
    Lantern,
    Spell(String),
}

impl Default for LightSourceType {
    fn default() -> Self {
        Self::Torch
    }
}

impl Display for LightSourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LightSourceType::Torch => {
                write!(f, "Torch")
            }
            LightSourceType::Lantern => {
                write!(f, "Lantern")
            }
            LightSourceType::Spell(name) => {
                write!(f, "Spell( {} )", name)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct LightSource {
    pub label: String,
    pub light_type: LightSourceType,
    pub radius_feet: u32,
    pub minutes_remaining: u32,
    pub last_roll: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TurnEntry {
    pub turn: u32,
    pub events: Vec<String>,
}

impl Default for TurnEntry {
    fn default() -> Self {
        TurnEntry {
            turn: 0,
            events: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShadowtrackData {
    pub turn: u32,
    pub light_sources: Vec<LightSource>,
    pub encounter_table: Vec<String>,
    pub torch_event_table: Vec<String>,
    pub encounter_roll: Option<u8>,
    pub event_log: Vec<TurnEntry>,
    pub clock_elapsed: u32,

    pub new_light_type: LightSourceType,
    pub new_light_label: String,
    pub new_light_minutes: u32,
    pub new_light_range: u32,
}

impl Default for ShadowtrackData {
    fn default() -> Self {
        Self {
            new_light_type: LightSourceType::default(),
            turn: 0,
            encounter_roll: None,
            event_log: vec![],
            light_sources: vec![],
            new_light_label: String::new(),
            new_light_minutes: 60,
            new_light_range: 30,
            encounter_table: DEFAULT_ENCOUNTER_TABLE.iter().map(|s| s.to_string()).collect(),
            torch_event_table: DEFAULT_TORCH_EVENTS_TABLE.iter().map(|s| s.to_string()).collect(),
            clock_elapsed: 0_u32,
        }
    }
}

