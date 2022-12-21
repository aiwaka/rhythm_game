use bevy::prelude::*;

use crate::{
    components::receptor::{prelude::*, PatternReceptor, PatternReceptorMarker},
    events::{AchievePatternEvent, CatchNoteEvent},
    resources::{
        config::{Bpm, GameDifficulty},
        score::ScoreResource,
        song::SongStartTime,
    },
    systems::system_labels::PatternReceptorSystemLabel,
    AppState,
};

/// 難易度Expert, Masterの場合はレセプタをここで登録.
fn setup_receptor(mut commands: Commands, diff: Res<GameDifficulty>) {
    /// PatternReceptorを実装した構造体を入れる.
    /// TODO: ここでトレイト境界を設定する方法を調べる（無いかも）
    macro_rules! spawn_receptor {
        ($x:ty) => {
            commands.spawn((
                <$x>::default(),
                PatternReceptorMarker(<$x>::NAME.to_string()),
            ));
        };
    }
    // GameDifficultyにはOrdを実装しているので不等号で表現できる
    if GameDifficulty::Normal < *diff {
        spawn_receptor!(FullSyncReceptor);
        spawn_receptor!(StepRightReceptor);
        spawn_receptor!(StepLeftReceptor);
        spawn_receptor!(DoubleTapReceptor);
        spawn_receptor!(TrillReceptor);
    }
}

/// レセプタにノーツを入力して更新する.
/// PatternReceptorで実装を要求する初期状態・入力更新パターン・終了条件を使って一般的な動作を記述する.
fn receptor_pipeline<T: PatternReceptor>(
    mut q: Query<&mut T>,
    mut note_ev_reader: EventReader<CatchNoteEvent>,
    mut achieve_ev_writer: EventWriter<AchievePatternEvent>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
    bpm: Res<Bpm>,
) {
    if let Ok(mut receptor) = q.get_single_mut() {
        let time_after_start = time.elapsed_seconds_f64() - start_time.0;
        if receptor.is_available() {
            // 初期化状態でないなら初期化するかどうか尋ねる
            if !receptor.initialized() {
                receptor.initialize_or_defer(time_after_start, **bpm);
            }
            // ノーツを入力
            for note_ev in note_ev_reader.iter() {
                receptor.input(note_ev)
            }
            // 条件を満たしていたらイベントを送信して初期化
            if let Some(pattern) = receptor.achieved() {
                achieve_ev_writer.send(AchievePatternEvent(pattern));
                // 達成イベントを送ったら重複しないように必ず初期化
                receptor.init();
            }
        }
    }
}

fn achieve_pattern(
    mut ev_reader: EventReader<AchievePatternEvent>,
    mut score: ResMut<ScoreResource>,
) {
    for ev in ev_reader.iter() {
        info!("{:?}", ev.0);
        score.push_pattern(ev.0);
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
                        .with_system(receptor_pipeline::<$receptor>)
                        .label(PatternReceptorSystemLabel::Recept),
                );
            };
        }

        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_receptor));
        add_receptor_to_system!(FullSyncReceptor);
        add_receptor_to_system!(StepRightReceptor);
        add_receptor_to_system!(StepLeftReceptor);
        add_receptor_to_system!(DoubleTapReceptor);
        add_receptor_to_system!(TrillReceptor);

        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(achieve_pattern)
                .after(PatternReceptorSystemLabel::Recept),
        );
    }
}
