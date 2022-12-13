use bevy::prelude::*;

use crate::{
    components::timer::{CountDownTimer, FrameCounter},
    resources::game_state::GameCount,
    AppState,
};

use super::system_labels::TimerSystemLabel;

pub fn timer_update(mut query: Query<&mut CountDownTimer>) {
    for mut timer in query.iter_mut() {
        timer.tick();
    }
}
/// 使い終わったタイマーは自動で削除される.
/// 通常のSystemが登録されるUpdateStageがすべて処理されたあとに処理される.
/// タイマーが終わったことを使うSystemには.after("count_down_update")の指定が必要になる.
fn delete_timer(mut commands: Commands, query: Query<(&CountDownTimer, Entity)>) {
    for (timer, ent) in query.iter() {
        if timer.is_finished() {
            if timer.auto_despawn() {
                commands.entity(ent).despawn_recursive();
            } else {
                commands.entity(ent).remove::<CountDownTimer>();
            }
        }
    }
}

fn frame_counter_update(mut query: Query<&mut FrameCounter>) {
    for mut counter in query.iter_mut() {
        counter.tick();
    }
}

// fn player_shot_timer(time: Res<Time>, mut timer_query: Query<&mut PlayerShotTimer>) {
//     for mut timer in timer_query.iter_mut() {
//         timer.0.tick(time.delta());
//     }
// }

// fn player_animation_timer(time: Res<Time>, mut timer_query: Query<&mut PlayerAnimationTimer>) {
//     for mut timer in timer_query.iter_mut() {
//         timer.0.tick(time.delta());
//     }
// }

/// ゲームカウントを増やす
fn update_game_count(mut game: ResMut<GameCount>) {
    game.0 += 1;
}

/// 常駐するタイマー類を初期化する
fn init_resident_timers(mut commands: Commands) {
    commands.insert_resource(GameCount(0));
    // commands.spawn().insert(PlayerShotTimer::new());
    // commands.spawn().insert(PlayerAnimationTimer::new());
}

pub struct TimersPlugin;
impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(timer_update.label(TimerSystemLabel::TimerUpdate));
        app.add_system(frame_counter_update.label(TimerSystemLabel::FrameCounterUpdate));
        app.add_system_to_stage(CoreStage::Last, delete_timer);

        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(init_resident_timers));
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                update_game_count
                    .label(TimerSystemLabel::UpdateGameCount)
                    .after(TimerSystemLabel::TimerUpdate)
                    .after(TimerSystemLabel::FrameCounterUpdate),
            ),
        );
    }
}
