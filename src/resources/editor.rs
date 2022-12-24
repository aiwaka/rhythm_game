use std::collections::VecDeque;

use bevy::prelude::*;

pub struct EditNote {
    pub key: i32,
    pub time_after_start: f64,
}

#[derive(Resource, Default)]
pub struct EditorNotesQueue(pub VecDeque<EditNote>);
