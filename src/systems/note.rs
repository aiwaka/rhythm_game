use bevy::prelude::*;
use bevy::time::FixedTimestep;

use crate::components::note::{KeyLane, LongNote, LongNoteState, MissingNote, NoteInfo};
use crate::components::timer::FrameCounter;
use crate::constants::{BASIC_NOTE_SPEED, FRAMERATE, MISS_THR, TARGET_Y};
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

#[allow(clippy::too_many_arguments)]
fn spawn_notes(
    mut commands: Commands,
    game_assets: Option<Res<GameAssetsHandles>>,
    notes: Option<ResMut<SongNotes>>,
    start_time: Option<Res<SongStartTime>>,
    speed: Option<Res<NoteSpeed>>,
    bpm: Option<Res<Bpm>>,
    time: Option<Res<Time>>,
    mut color_material: ResMut<Assets<ColorMaterial>>,
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
    let speed = speed.unwrap();
    let bpm = bpm.unwrap();

    // 現在スタートから何秒経ったかと前の処理が何秒だったかを取得する.
    let time_after_start = start_time.time_after_start(&time);

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

        let note_mesh = game_assets.get_mesh_from_note_type(
            &mut color_material,
            &note.note_type,
            **speed,
            **bpm,
            false,
        );
        let is_long_note = matches!(
            note.note_type,
            NoteType::Long {
                key: _,
                length: _,
                id: _
            }
        );
        let note_bundle = (note, note_mesh);

        let ent = commands.spawn(note_bundle).id();
        if is_long_note {
            commands
                .entity(ent)
                .insert((LongNote::new(), FrameCounter::new()));
        }
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

/// ロングノーツの演出
fn long_note_operation(
    q: Query<(&Handle<ColorMaterial>, &LongNote)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (color_handle, note) in q.iter() {
        let color = &mut materials.get_mut(color_handle).unwrap().color;
        match note.state {
            LongNoteState::BeforeRetrieve => {
                *color = Color::rgba(1.0, 1.0, 1.0, 0.7);
            }
            LongNoteState::Hold | LongNoteState::End => {
                *color = Color::CYAN;
            }
            LongNoteState::Miss => {
                *color = Color::rgba(0.2, 0.0, 0.0, 0.7);
            }
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
    let time_after_start = start_time.time_after_start(&time);
    // despawnはクエリには影響しないため, 重複したキーで一つのノーツを複数回取れてしまう.
    // これを防ぐために取得したノーツをメモする.
    let mut removed_ent = vec![];
    for lane in lane_q.iter_mut() {
        for (note, ent) in note_q.iter() {
            let note_target_time = note.target_time;
            // 現在時刻が許容範囲・鍵盤番号が一致・キーがちょうど押された・まだ消去されていないノートを取得処理
            let note_caught = match note.note_type {
                NoteType::Normal { key } => key == lane.0,
                NoteType::BarLine => false,
                NoteType::AdLib { key } => key == lane.0,
                // ロングノーツはここでは扱わない
                NoteType::Long {
                    key: _,
                    length: _,
                    id: _,
                } => false,
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

/// ロングノーツの取得処理はこちら
#[allow(clippy::too_many_arguments)]
fn catch_long_notes(
    mut note_q: Query<(&NoteInfo, &mut LongNote, &mut FrameCounter)>,
    mut lane_q: Query<&KeyLane>,
    key_input: Res<Input<KeyCode>>,
    mut catch_ev_writer: EventWriter<CatchNoteEvent>,
    mut eval_ev_writer: EventWriter<NoteEvalEvent>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
    bpm: Res<Bpm>,
    beat: Res<Beat>,
) {
    let time_after_start = start_time.time_after_start(&time);
    for lane in lane_q.iter_mut() {
        for (note, mut long_note, mut counter) in note_q.iter_mut() {
            // ロングノーツでない場合飛ばす（クエリの制限により基本的にありえないはずだが）
            let NoteType::Long { key, length, id: _} = note.note_type else { continue };
            // キーとレーンが異なる場合は処理しない. また, 終了状態の場合も処理しない.
            if key != lane.0 {
                continue;
            }
            // ロングノーツの場合は始点の到着時刻
            let note_target_time = note.target_time;
            let note_end_time = note_target_time + (length / **bpm * 60.0) as f64;
            match long_note.state {
                LongNoteState::BeforeRetrieve => {
                    if (note_target_time - MISS_THR..=note_target_time + MISS_THR)
                        .contains(&time_after_start)
                        && lane.key_just_pressed(&key_input)
                    {
                        // 現在時刻が許容範囲・鍵盤番号が一致・キーがちょうど押されたら始点の取得処理
                        catch_ev_writer.send(CatchNoteEvent::new(
                            note,
                            time_after_start,
                            **bpm,
                            **beat,
                        ));
                        eval_ev_writer.send(NoteEvalEvent::new(note, time_after_start));
                        long_note.state = LongNoteState::Hold;
                    } else if time_after_start > note_target_time + MISS_THR {
                        long_note.state = LongNoteState::Miss;
                        counter.reset();
                    } else if time_after_start > note_target_time {
                        // ちょうど到達したときにカウンターをリセットする
                        counter.reset();
                    }
                }
                LongNoteState::Hold => {
                    if lane.key_pressed(&key_input)
                        && (note_target_time..=note_end_time).contains(&time_after_start)
                    {
                        if (counter.count() + 1) % 12 == 0 {
                            // 押しっぱなしでホールド中なら一定間隔で加点
                            // TODO: ノーツが判定可能かどうかで分岐し, ホールド中ならPerfect, そうでないならMissを送るように変更したい.
                            catch_ev_writer.send(CatchNoteEvent::new(
                                note,
                                time_after_start,
                                **bpm,
                                **beat,
                            ));
                            eval_ev_writer.send(NoteEvalEvent {
                                eval: CatchEval::Perfect,
                                note: note.clone(),
                            });
                        }
                    } else if lane.key_just_released(&key_input) {
                        // 離された場合は終点かどうかチェックして分岐
                        if (note_end_time - MISS_THR..=note_end_time + MISS_THR)
                            .contains(&time_after_start)
                        {
                            // NOTE: 終点でも許容範囲で離すことを要請している. 押しっぱなしでもいいようにする？
                            long_note.state = LongNoteState::End;
                        }
                    } else if time_after_start > note_target_time + MISS_THR {
                        long_note.state = LongNoteState::Miss;
                    }
                }
                LongNoteState::Miss => {
                    if (note_target_time..=note_end_time).contains(&time_after_start)
                        && (counter.count() + 1) % 12 == 0
                    {
                        eval_ev_writer.send(NoteEvalEvent {
                            eval: CatchEval::Miss,
                            note: note.clone(),
                        });
                    }
                }
                LongNoteState::End => {}
            }
        }
    }
}

/// 取れなかったときの処理
#[allow(clippy::too_many_arguments)]
fn drop_notes(mut commands: Commands, query: Query<(&Transform, &NoteInfo, Entity)>) {
    for (trans, _, ent) in query.iter() {
        let pos_y = trans.translation.y;
        if pos_y < -800.0 {
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
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(long_note_operation));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(catch_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(catch_long_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(drop_notes));
    }
}
