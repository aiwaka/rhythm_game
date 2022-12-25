use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct EditNote {
    pub key: i32,
    pub time_after_start: f64,
}

/// 存在していればエディタを終了している状態を表す.
#[derive(Resource, Debug)]
pub struct QuittingEditor;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct EditorNotesQueue(pub VecDeque<EditNote>);
