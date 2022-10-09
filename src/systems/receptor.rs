use bevy::prelude::*;

use crate::{
    components::receptor::{prelude::*, PatternReceptor},
    events::{AchievePatternEvent, CatchNoteEvent},
    resources::note::AudioStartTime,
    AppState,
};

/// レセプタをここで登録
fn setup_receptor(mut commands: Commands) {
    macro_rules! spawn_receptor {
        ($x:expr) => {
            commands.spawn().insert($x);
        };
    }
    spawn_receptor!(AllSyncReceptor::default());
}

/// レセプタにノーツを入力して更新する
fn receptor_pipeline<T: PatternReceptor>(
    mut q: Query<&mut T>,
    mut note_ev_reader: EventReader<CatchNoteEvent>,
    mut achieve_ev_writer: EventWriter<AchievePatternEvent>,
    start_time: Res<AudioStartTime>,
    time: Res<Time>,
) {
    if let Ok(mut receptor) = q.get_single_mut() {
        let time_after_start = time.seconds_since_startup() - start_time.0;
        if receptor.is_available() {
            receptor.init_or_defer(time_after_start);
            for note_ev in note_ev_reader.iter() {
                receptor.input(note_ev)
            }
            if let Some(pattern) = receptor.achieved() {
                achieve_ev_writer.send(AchievePatternEvent(pattern));
                // 達成イベントを送ったら重複しないように必ず初期化
                receptor.init();
            }
        }
    }
}

fn achieve_pattern(mut ev_reader: EventReader<AchievePatternEvent>) {
    for ev in ev_reader.iter() {
        info!("{:?}", ev.0);
    }
}

pub struct PatternReceptorPlugin;
impl Plugin for PatternReceptorPlugin {
    fn build(&self, app: &mut App) {
        /// レセプタ構造体をappに追加するマクロ.
        macro_rules! add_receptor_to_system {
            ($receptor:ty) => {
                app.add_system_set(
                    SystemSet::on_update(AppState::Game)
                        .with_system(receptor_pipeline::<$receptor>),
                );
            };
        }

        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_receptor));
        // app.add_system_set(
        //     SystemSet::on_update(AppState::Game).with_system(receptor_pipeline::<AllSyncReceptor>),
        // );
        add_receptor_to_system!(AllSyncReceptor);

        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(achieve_pattern));
    }
}
