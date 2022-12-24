use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct EditNote {
    pub key: i32,
    pub time_after_start: f64,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct EditorNotesQueue(pub VecDeque<EditNote>);
