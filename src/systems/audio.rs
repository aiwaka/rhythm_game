use bevy::prelude::*;

use crate::{resources::handles::GameAssetsHandles, AppState};

// TODO: use timer resource for get time
fn start_song(audio: Res<Audio>, time: Res<Time>, handles: Res<GameAssetsHandles>) {
    // 実時間でゲーム起動から3秒後にスタート
    let sec = time.seconds_since_startup();
    let sec_last = sec - time.delta_seconds_f64();

    if sec_last <= 3.0 && 3.0 <= sec {
        audio.play(handles.music.clone());
    }
}

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(start_song));
    }
}
