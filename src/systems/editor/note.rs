use bevy::prelude::*;
use bevy::time::FixedTimestep;

use crate::components::note::{KeyLane, NoteInfo};
use crate::constants::{
    BASIC_NOTE_SPEED, FRAMERATE, MISS_THR, NOTE_SPAWN_Y, SCREEN_HEIGHT, TARGET_Y,
};
use crate::events::EditNoteEvent;
use crate::resources::editor::{EditNote, EditorNotesQueue};
use crate::resources::note::NoteType;
use crate::resources::{
    config::{Beat, Bpm, NoteSpeed},
    handles::GameAssetsHandles,
    song::{SongNotes, SongStartTime},
};
use crate::AppState;

use crate::systems::system_labels::TimerSystemLabel;

/// エディット時は下から出現するため出現位置を調整したものを用意する
const EDIT_NOTE_SPAWN_Y: f32 = (NOTE_SPAWN_Y - TARGET_Y) * -1.0 + TARGET_Y;

fn setup_queue(mut commands: Commands) {
    // エディットキューを用意する
    commands.insert_resource(EditorNotesQueue::default());
}

/// エディット中でもすでに存在しているものは使う.
/// BPM変更等も反映できるようにする.
fn spawn_notes(
    mut commands: Commands,
    game_assets: Option<Res<GameAssetsHandles>>,
    notes: Option<ResMut<SongNotes>>,
    start_time: Option<Res<SongStartTime>>,
    time: Option<Res<Time>>,
    state: Res<State<AppState>>,
) {
    // FixedTimeStepを利用するためステート依存を外しているため特殊な引数となっている.
    if !matches!(state.current(), &AppState::Editor) {
        return;
    }
    let game_assets = game_assets.unwrap();
    let mut notes = notes.unwrap();
    let start_time = start_time.unwrap();
    let time = time.unwrap();

    // 現在スタートから何秒経ったかと前の処理が何秒だったかを取得する.
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;

    while {
        if let Some(note) = notes.front() {
            time_after_start > note.spawn_time
        } else {
            false
        }
    } {
        let note = notes.pop_front().unwrap();

        let note_bundle = match note.note_type {
            NoteType::Normal { key } => {
                let transform = Transform {
                    translation: Vec3::new(KeyLane::x_coord_from_num(key), EDIT_NOTE_SPAWN_Y, 1.0),
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
                    translation: Vec3::new(0.0, EDIT_NOTE_SPAWN_Y, 0.5),
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
                    translation: Vec3::new(KeyLane::x_coord_from_num(key), EDIT_NOTE_SPAWN_Y, 1.0),
                    ..Default::default()
                };
                let mesh = ColorMesh2dBundle {
                    mesh: game_assets.note.clone().into(),
                    material: game_assets.color_material_red.clone(),
                    transform,
                    ..Default::default()
                };
                (note.clone(), mesh)
            }
        };
        commands.spawn(note_bundle);
    }
}

/// 上がっていくように
fn move_notes(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &NoteInfo)>,
    speed: Res<NoteSpeed>,
) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.y += time.delta_seconds() * speed.0 * BASIC_NOTE_SPEED;
    }
}

/// エディター本体.
/// 鍵盤対応のキーを押したらノーツが出現し, ノーツ情報キューに溜め込まれる.
#[allow(clippy::too_many_arguments)]
fn input_notes(
    mut lane_q: Query<&KeyLane>,
    key_input: Res<Input<KeyCode>>,
    start_time: Res<SongStartTime>,
    mut queue: ResMut<EditorNotesQueue>,
    time: Res<Time>,
    mut ev_writer: EventWriter<EditNoteEvent>,
) {
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    for lane in lane_q.iter_mut() {
        if lane.key_just_pressed(&key_input) {
            let note = EditNote {
                key: lane.0,
                time_after_start,
            };
            ev_writer.send(note.clone().into());
            queue.push_back(note);
        }
    }
}

/// エディットノートを出現
fn spawn_edit_note(
    mut commands: Commands,
    mut ev_reader: EventReader<EditNoteEvent>,
    game_assets: Res<GameAssetsHandles>,
) {
    for ev in ev_reader.iter() {
        let key = ev.key;
        let transform = Transform {
            translation: Vec3::new(KeyLane::x_coord_from_num(key), TARGET_Y, 1.0),
            ..Default::default()
        };
        let mesh = ColorMesh2dBundle {
            mesh: game_assets.note.clone().into(),
            material: game_assets.color_material_blue.clone(),
            transform,
            ..Default::default()
        };
        let note_info = NoteInfo {
            note_type: NoteType::Normal { key },
            bar: 0,
            beat: 0.0,
            spawn_time: 0.0,
            target_time: 0.0,
        };
        commands.spawn((note_info, mesh));
    }
}

/// BPM変更等, ノーツの種類によっては処理するためのもの
fn execute_notes(
    note_q: Query<(&NoteInfo, Entity)>,
    mut lane_q: Query<&KeyLane>,
    start_time: Res<SongStartTime>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    _bpm: Res<Bpm>,
    _beat: Res<Beat>,
) {
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    for lane in lane_q.iter_mut() {
        for (note, ent) in note_q.iter() {
            let note_target_time = note.target_time;
            // 現在時刻が許容範囲・鍵盤番号が一致・キーがちょうど押された・まだ消去されていないノートを取得処理
            if (note_target_time - MISS_THR..=note_target_time + MISS_THR)
                .contains(&time_after_start)
                && lane.key_just_pressed(&key_input)
            {
                // execute something
                // match note.note_type {
                //     NoteType::Normal { key: _ } => false,
                //     NoteType::BarLine => false,
                //     NoteType::AdLib { key: _ } => false,
                // }
            }
        }
    }
}

/// 画面外にでたノーツを消去する
#[allow(clippy::too_many_arguments)]
fn drop_notes(mut commands: Commands, query: Query<(&Transform, &NoteInfo, Entity)>) {
    // let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    for (trans, _, ent) in query.iter() {
        let pos_y = trans.translation.y;
        if pos_y > SCREEN_HEIGHT / 2.0 {
            commands.entity(ent).despawn();
        }
    }
}

const TIMESTEP: f64 = 1.0 / FRAMERATE;

pub(super) struct EditorNotePlugin;
impl Plugin for EditorNotePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup_queue));
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(spawn_notes.label(TimerSystemLabel::StartAudio)),
        );
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(move_notes));
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(input_notes));
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(spawn_edit_note));
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(execute_notes));
        app.add_system_set(SystemSet::on_update(AppState::Editor).with_system(drop_notes));
    }
}
