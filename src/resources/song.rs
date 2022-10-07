use std::collections::VecDeque;

use serde_derive::{Deserialize, Serialize};

use crate::components::note::{KeyColumn, NoteTime};

#[derive(Debug)]
pub struct SongConfig {
    pub name: String,
    pub music_filename: String,
    pub bpm: f32,
    pub notes: VecDeque<NoteTime>,
}

/// use for toml
#[derive(Deserialize, Debug)]
pub struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub bpm: f32,
    pub notes: Vec<NoteTimeToml>,
}

/// use for toml
#[derive(Deserialize, Serialize, Debug)]
pub struct NoteTimeToml {
    pub click_time: f64,
    pub key_column: KeyColumn,
}
