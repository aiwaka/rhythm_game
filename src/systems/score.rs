use bevy::prelude::*;

use crate::{events::NoteEvalEvent, resources::score::ScoreResource, AppState};

fn update_score(mut ev_reader: EventReader<NoteEvalEvent>, mut score: ResMut<ScoreResource>) {
    for ev in ev_reader.iter() {
        score.update_score(ev);
    }
}

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(update_score));
    }
}
