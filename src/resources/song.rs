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
    /// エディットモードで扱えるかどうかのフラグ. 立てなくても良いようにOption付き.
    /// Noneはtrueとして扱い, trueなら編集不可とする. 編集可能にする場合falseにする.
    pub edit_freeze: Option<bool>,
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
    /// trueならエディット不可
    pub edit_freeze: bool,
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
            edit_freeze: data.edit_freeze.unwrap_or(true),
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
    pub edit_freeze: bool,
}
impl From<SongConfig> for SongConfigResource {
    fn from(config: SongConfig) -> Self {
        Self {
            name: config.name,
            song_filename: config.filename,
            length: config.length,
            edit_freeze: config.edit_freeze,
        }
    }
}

/// リソースとして追加するノーツ情報
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct SongNotes(pub VecDeque<NoteInfo>);
