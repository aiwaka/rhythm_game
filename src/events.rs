use bevy::prelude::*;

/// 曲を再生し始める瞬間を伝えるイベント
pub(super) struct AudioStartEvent;

pub(super) fn add_events_to_game(app: &mut App) {
    app.add_event::<AudioStartEvent>();
}
