use std::collections::VecDeque;

use bevy::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::components::note::NoteInfo;

use super::note::{NoteSpawn, NoteSpawnParser};

/// 曲再生を開始するゲーム開始からの時間（秒）
#[derive(Resource)]
pub struct SongStartTime(pub f64);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SongConfigParser {
    pub name: String,
    pub filename: String,
    /// 曲の尺（秒）
    pub length: f64,
    /// 曲開始時点で一小節に何拍あるか
    pub initial_beat: u32,
    pub initial_bpm: f32,
    pub notes: Vec<NoteSpawnParser>,
}

#[derive(Debug, Clone)]
pub struct SongConfig {
    pub name: String,
    pub filename: String,
    /// 曲の尺（秒）
    pub length: f64,
    /// 曲開始時点で一小節に何拍あるか
    pub initial_beat: u32,
    pub initial_bpm: f32,
    pub notes: Vec<NoteSpawn>,
}
impl From<SongConfigParser> for SongConfig {
    fn from(data: SongConfigParser) -> Self {
        Self {
            name: data.name,
            filename: data.filename,
            length: data.length,
            initial_beat: data.initial_beat,
            initial_bpm: data.initial_bpm,
            // map(NoteSpawn::from)でも動く
            notes: data.notes.into_iter().map(|note| note.into()).collect_vec(),
        }
    }
}
impl From<SongConfig> for SongConfigParser {
    fn from(data: SongConfig) -> Self {
        Self {
            name: data.name,
            filename: data.filename,
            length: data.length,
            initial_beat: data.initial_beat,
            initial_bpm: data.initial_bpm,
            notes: data.notes.into_iter().map(|note| note.into()).collect_vec(),
        }
    }
}

/// リソースとして追加する曲データ構造体
#[derive(Resource, Debug, Clone)]
pub struct SongConfigResource {
    pub name: String,
    pub song_filename: String,
    /// 曲の尺（秒）
    pub length: f64,
}
impl From<SongConfig> for SongConfigResource {
    fn from(config: SongConfig) -> Self {
        Self {
            name: config.name,
            song_filename: config.filename,
            length: config.length,
        }
    }
}

/// リソースとして追加するノーツ情報
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct SongNotes(pub VecDeque<NoteInfo>);
