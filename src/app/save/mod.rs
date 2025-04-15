use crate::app::state::ShadowtrackData;
use rfd::FileDialog;
use serde_json;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::{fmt, io};

const DEFAULT_SAVE_FILE: &str = "save.json";

#[derive(Debug)]
pub enum SaveError {
    Serialization(serde_json::Error),
    Io(io::Error),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::Serialization(e) => write!(f, "Serialization error: {}", e),
            SaveError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(err: serde_json::Error) -> Self {
        SaveError::Serialization(err)
    }
}

impl From<io::Error> for SaveError {
    fn from(err: io::Error) -> Self {
        SaveError::Io(err)
    }
}

pub fn write_save(save_file: &PathBuf, game_data: &ShadowtrackData) -> Result<(), SaveError> {
    let save_data = serde_json::to_string_pretty(game_data)?;
    write(save_file, save_data)?;
    Ok(())
}

pub fn load_save(save_file: &PathBuf) -> Result<ShadowtrackData, SaveError> {
    let content = read_to_string(save_file)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn save_to_file(game_data: &ShadowtrackData) -> Result<(), SaveError> {
    if let Some(save_file) = FileDialog::new()
        .set_title("Save game data to?")
        .set_file_name(DEFAULT_SAVE_FILE)
        .set_directory("./")
        .save_file()
    {
        write_save(&save_file, game_data)?;
    }
    Ok(())
}

pub fn load_from_file() -> Result<ShadowtrackData, SaveError> {
    let save_file = FileDialog::new()
        .add_filter("text", &["txt", "json"])
        .set_directory("./")
        .pick_file()
        .ok_or_else(|| SaveError::Io(io::Error::new(io::ErrorKind::Other, "No file selected")))?;

    load_save(&save_file)
}
