use bevy::prelude::*;

use crate::components::timer::{CountDownTimer, FrameCounter};

use super::system_labels::TimerSystemLabel;

pub fn timer_update(mut query: Query<&mut CountDownTimer>) {
    for mut timer in query.iter_mut() {
        timer.tick();
    }
}
/// 使い終わったタイマーは自動で削除される.
/// 通常のSystemが登録されるUpdateStageがすべて処理されたあとに処理される.
/// タイマーが終わったことを使うSystemには.after(TimerSystemLabel::TimerUpdate)の指定が必要.
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

pub struct TimersPlugin;
impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(timer_update.label(TimerSystemLabel::TimerUpdate));
        app.add_system(frame_counter_update.label(TimerSystemLabel::FrameCounterUpdate));
        app.add_system_to_stage(CoreStage::Last, delete_timer);
    }
}
