use bevy::prelude::*;

use crate::{add_update_system, events::NoteEvalEvent, resources::score::ScoreResource, AppState};

fn update_score(mut ev_reader: EventReader<NoteEvalEvent>, mut score: ResMut<ScoreResource>) {
    for ev in ev_reader.iter() {
        score.update_score(&ev.eval, &ev.note.note_type);
    }
}

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        add_update_system!(app, Game, update_score);
    }
}
