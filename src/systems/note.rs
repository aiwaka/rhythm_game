use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use itertools::Itertools;

use crate::components::note::KeyLane;
use crate::components::note::NoteInfo;
use crate::components::timer::FrameCounter;
use crate::components::ui::GameSceneObject;
use crate::constants::BASIC_NOTE_SPEED;
use crate::constants::MISS_THR;
use crate::constants::NOTE_SPAWN_Y;
use crate::constants::TARGET_Y;
use crate::events::CatchNoteEvent;
use crate::resources::config::Beat;
use crate::resources::config::Bpm;
use crate::resources::config::NoteSpeed;
use crate::resources::game_state::ExistingEntities;
use crate::resources::handles::GameAssetsHandles;
use crate::resources::note::NoteType;
use crate::resources::score::CatchEval;
use crate::resources::score::ScoreResource;
use crate::resources::song::SongNotes;
use crate::resources::song::SongStartTime;
use crate::AppState;

use super::system_labels::TimerSystemLabel;

fn set_lane(
    mut commands: Commands,
    handles: Res<GameAssetsHandles>,
    already_exist_q: Query<Entity>,
) {
    // シーン遷移時点で存在しているエンティティをすべて保存
    commands.insert_resource(ExistingEntities(already_exist_q.iter().collect_vec()));
    for i in 0..4 {
        let x = KeyLane::x_coord_from_num(i);
        let transform = Transform {
            translation: Vec3::new(x, TARGET_Y + 250.0, 0.1),
            ..Default::default()
        };
        commands
            .spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(handles.lane_background.clone()),
                material: handles.color_material_lane_background[i as usize].clone(),
                transform,
                ..Default::default()
            })
            .insert(KeyLane(i))
            .insert(FrameCounter::new_default(60))
            .insert(GameSceneObject);
    }
}

fn spawn_notes(
    mut commands: Commands,
    textures: Res<GameAssetsHandles>,
    mut notes: ResMut<SongNotes>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
) {
    // 現在スタートから何秒経ったかと前の処理が何秒だったかを取得する.
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    let time_last = time_after_start - time.delta_seconds_f64();

    // キューの先頭を見て, 出現時刻なら出現させることを繰り返す.
    while {
        if let Some(note) = notes.front() {
            time_last < note.spawn_time && note.spawn_time < time_after_start
        } else {
            false
        }
    } {
        let note = notes.pop_front().unwrap();
        let note_mesh = textures.note.clone();
        let color = textures.color_material_blue.clone();

        let note_bundle = match note.note_type {
            NoteType::Normal { key } => {
                let transform = Transform {
                    translation: Vec3::new(KeyLane::x_coord_from_num(key), NOTE_SPAWN_Y, 1.0),
                    ..Default::default()
                };
                let mesh = ColorMesh2dBundle {
                    mesh: Mesh2dHandle::from(note_mesh),
                    material: color,
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
    time: Res<Time>,
    mut query: Query<(&mut Transform, &NoteInfo)>,
    speed: Res<NoteSpeed>,
) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * speed.0 * BASIC_NOTE_SPEED;
        let allow_distance = MISS_THR as f32 * BASIC_NOTE_SPEED * speed.0;
        let distance_after_target = transform.translation.y - (TARGET_Y - allow_distance);
        if distance_after_target < -0.02 {
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
    mut score: ResMut<ScoreResource>,
    mut ev_writer: EventWriter<CatchNoteEvent>,
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
            };
            if (note_target_time - MISS_THR..=note_target_time + MISS_THR)
                .contains(&time_after_start)
                && note_caught
                && lane.key_just_pressed(&key_input)
                && !removed_ent.contains(&ent)
            {
                commands.entity(ent).despawn();
                removed_ent.push(ent);
                let score_eval = CatchEval::new(note.target_time, time_after_start);
                score.update_score(&score_eval);
                ev_writer.send(CatchNoteEvent::new(note, time_after_start, bpm.0, beat.0));
            }
        }
    }
}

/// 取れなかったときの処理
#[allow(clippy::too_many_arguments)]
fn drop_notes(
    mut commands: Commands,
    query: Query<(&Transform, &NoteInfo, Entity)>,
    mut score: ResMut<ScoreResource>,
    mut ev_writer: EventWriter<CatchNoteEvent>,
    start_time: Res<SongStartTime>,
    time: Res<Time>,
    bpm: Res<Bpm>,
    beat: Res<Beat>,
) {
    let time_after_start = time.elapsed_seconds_f64() - start_time.0;
    for (trans, note, ent) in query.iter() {
        let pos_y = trans.translation.y;
        if pos_y < 2.0 * TARGET_Y {
            commands.entity(ent).despawn();
            let eval = CatchEval::Miss;
            score.update_score(&eval);
            ev_writer.send(CatchNoteEvent::new(note, time_after_start, **bpm, **beat));
        }
    }
}

pub struct NotePlugin;
impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(set_lane));
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_notes.label(TimerSystemLabel::StartAudio)),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(catch_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(drop_notes));
    }
}
