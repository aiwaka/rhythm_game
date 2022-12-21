use bevy::prelude::*;
use bevy::time::FixedTimestep;

use crate::components::note::{KeyLane, MissingNote, NoteInfo};
use crate::constants::{BASIC_NOTE_SPEED, FRAMERATE, MISS_THR, NOTE_SPAWN_Y, TARGET_Y};
use crate::events::{CatchNoteEvent, NoteEvalEvent};
use crate::resources::note::NoteType;
use crate::resources::{
    config::{Beat, Bpm, NoteSpeed},
    handles::GameAssetsHandles,
    score::CatchEval,
    song::{SongNotes, SongStartTime},
};
use crate::AppState;

use super::system_labels::TimerSystemLabel;

fn spawn_notes(
    mut commands: Commands,
    game_assets: Option<Res<GameAssetsHandles>>,
    notes: Option<ResMut<SongNotes>>,
    start_time: Option<Res<SongStartTime>>,
    time: Option<Res<Time>>,
    state: Res<State<AppState>>,
) {
    // FixedTimeStepを利用するためステート依存を外しているため特殊な引数となっている.
    // ゲームステートかどうかを判定し, そうでないならまるごと実行しない.
    if !matches!(state.current(), &AppState::Game) {
        return;
    }
    // エラー回避のためにリソースにOptionを付けていたが, ゲームステートなら存在するはずなのでunwrapする.
    let game_assets = game_assets.unwrap();
    let mut notes = notes.unwrap();
    let start_time = start_time.unwrap();
    let time = time.unwrap();

    // 現在スタートから何秒経ったかと前の処理が何秒だったかを取得する.
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    // let time_last = time_after_start - time.delta_seconds_f64();

    // キューの先頭を見て, 出現時刻なら出現させることを繰り返す.
    while {
        if let Some(note) = notes.front() {
            // NOTE: 厳密にそのフレームで出現時間になっているかではなく, ソート済みを前提として時間が過ぎているかどうかのみで判定するようにした. 問題があれば直す.
            // (time_last..time_after_start).contains(&note.spawn_time)
            time_after_start > note.spawn_time
        } else {
            false
        }
    } {
        let note = notes.pop_front().unwrap();

        let note_bundle = match note.note_type {
            NoteType::Normal { key } => {
                let transform = Transform {
                    translation: Vec3::new(KeyLane::x_coord_from_num(key), NOTE_SPAWN_Y, 1.0),
                    ..Default::default()
                };
                let mesh = ColorMesh2dBundle {
                    mesh: game_assets.note.clone().into(),
                    material: game_assets.color_material_blue.clone(),
                    transform,
                    ..Default::default()
                };
                (note.clone(), mesh)
            }
            NoteType::BarLine => {
                let transform = Transform {
                    translation: Vec3::new(0.0, NOTE_SPAWN_Y, 0.5),
                    ..Default::default()
                };
                let mesh = ColorMesh2dBundle {
                    mesh: game_assets.bar_note.clone().into(),
                    material: game_assets.color_material_white_trans.clone(),
                    transform,
                    ..Default::default()
                };
                (note.clone(), mesh)
            }
            NoteType::AdLib { key } => {
                let transform = Transform {
                    translation: Vec3::new(KeyLane::x_coord_from_num(key), NOTE_SPAWN_Y, 1.0),
                    ..Default::default()
                };
                let mesh = ColorMesh2dBundle {
                    mesh: game_assets.note.clone().into(),
                    material: game_assets.color_material_trans.clone(),
                    // DEBUG: デバッグ時は色を変える
                    // material: game_assets.color_material_red.clone(),
                    transform,
                    ..Default::default()
                };
                (note.clone(), mesh)
            }
        };
        commands.spawn(note_bundle);
    }
}

fn move_notes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &NoteInfo, Option<&MissingNote>, Entity)>,
    speed: Res<NoteSpeed>,
    mut eval_ev_writer: EventWriter<NoteEvalEvent>,
) {
    for (mut transform, note, missing, ent) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * speed.0 * BASIC_NOTE_SPEED;
        let allow_distance = MISS_THR as f32 * BASIC_NOTE_SPEED * speed.0;
        let distance_after_target = transform.translation.y - (TARGET_Y - allow_distance);
        // ミス処理を行うノーツのタイプを選択する
        if matches!(note.note_type, NoteType::Normal { key: _ }) && distance_after_target < -0.02 {
            if missing.is_none() {
                // ミスが確定したときにコンポーネントを付与しつつイベント送信
                eval_ev_writer.send(NoteEvalEvent {
                    eval: CatchEval::Miss,
                    note: note.clone(),
                });
                commands.entity(ent).insert(MissingNote);
            }
            transform.rotate_axis(Vec3::Z, 0.1);
            transform.scale = (transform.scale
                - time.delta_seconds() * distance_after_target * 0.01)
                .clamp_length(0.0, 100.0);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn catch_notes(
    mut commands: Commands,
    note_q: Query<(&NoteInfo, Entity)>,
    mut lane_q: Query<&KeyLane>,
    key_input: Res<Input<KeyCode>>,
    mut catch_ev_writer: EventWriter<CatchNoteEvent>,
    mut eval_ev_writer: EventWriter<NoteEvalEvent>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
    bpm: Res<Bpm>,
    beat: Res<Beat>,
) {
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    let mut removed_ent = vec![];
    for lane in lane_q.iter_mut() {
        for (note, ent) in note_q.iter() {
            let note_target_time = note.target_time;
            // 現在時刻が許容範囲・鍵盤番号が一致・キーがちょうど押された・まだ消去されていないノートを取得処理
            let note_caught = match note.note_type {
                NoteType::Normal { key } => key == lane.0,
                NoteType::BarLine => false,
                NoteType::AdLib { key } => key == lane.0,
            };
            if (note_target_time - MISS_THR..=note_target_time + MISS_THR)
                .contains(&time_after_start)
                && note_caught
                && lane.key_just_pressed(&key_input)
                && !removed_ent.contains(&ent)
            {
                commands.entity(ent).despawn();
                removed_ent.push(ent);
                catch_ev_writer.send(CatchNoteEvent::new(note, time_after_start, **bpm, **beat));
                eval_ev_writer.send(NoteEvalEvent::new(note, time_after_start));
            }
        }
    }
}

/// 取れなかったときの処理
#[allow(clippy::too_many_arguments)]
fn drop_notes(mut commands: Commands, query: Query<(&Transform, &NoteInfo, Entity)>) {
    // let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    for (trans, _, ent) in query.iter() {
        let pos_y = trans.translation.y;
        if pos_y < 2.0 * TARGET_Y {
            commands.entity(ent).despawn();
        }
    }
}

const TIMESTEP: f64 = 1.0 / FRAMERATE;

pub struct NotePlugin;
impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(spawn_notes.label(TimerSystemLabel::StartAudio)),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(catch_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(drop_notes));
    }
}
