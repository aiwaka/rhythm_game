use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct EditNote {
    pub key: i32,
    pub bar: u32,
    pub beat: f64,
    // pub time_after_start: f64,
}

/// エディタ中における小節番号
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct EditorBar(pub u32);
/// エディタ中における拍番号
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct EditorBeat(pub f64);

/// 存在していればエディタを終了している状態を表す.
#[derive(Resource, Debug)]
pub struct QuittingEditor;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct EditorNotesQueue(pub VecDeque<EditNote>);
