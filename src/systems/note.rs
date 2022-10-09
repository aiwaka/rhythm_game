use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::components::note::KeyLane;
use crate::components::note::Note;
use crate::components::timer::FrameCounter;
use crate::events::CatchNoteEvent;
use crate::game_constants::{ERROR_THRESHOLD, NOTE_BASE_SPEED, SPAWN_POSITION, TARGET_POSITION};
use crate::resources::handles::GameAssetsHandles;
use crate::resources::score::ScoreResource;
use crate::resources::song::{AudioStartTime, SongConfig, Speed};
use crate::AppState;

use super::system_labels::TimerSystemLabel;

fn set_lane(mut commands: Commands, handles: Res<GameAssetsHandles>) {
    for i in 0..4 {
        let x = KeyLane::x_coord_from_num(i);
        let transform = Transform {
            translation: Vec3::new(x, TARGET_POSITION + 250.0, 0.1),
            ..Default::default()
        };
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(handles.lane_background.clone()),
                material: handles.color_material_lane_background[i as usize].clone(),
                transform,
                ..Default::default()
            })
            .insert(KeyLane(i))
            .insert(FrameCounter::new_default(60));
    }
}

fn spawn_notes(
    mut commands: Commands,
    textures: Res<GameAssetsHandles>,
    mut song_config: ResMut<SongConfig>,
    start_time: Res<AudioStartTime>,
    time: Res<Time>,
) {
    // 現在スタートから何秒経ったかと前の処理が何秒だったかを取得する.
    let time_after_start = time.seconds_since_startup() - start_time.0;
    let time_last = time_after_start - time.delta_seconds_f64();

    // キューの先頭を見て, 出現時刻なら出現させることを繰り返す.
    while {
        if let Some(note) = song_config.notes.front() {
            time_last < note.spawn_time && note.spawn_time < time_after_start
        } else {
            false
        }
    } {
        let note = song_config.notes.pop_front().unwrap();
        let note_mesh = textures.note.clone();
        let color = textures.color_material_blue.clone();

        let transform = Transform {
            translation: Vec3::new(
                KeyLane::x_coord_from_num(note.key_column),
                SPAWN_POSITION,
                1.0,
            ),
            ..Default::default()
        };
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(note_mesh),
                material: color,
                transform,
                ..Default::default()
            })
            .insert(note);
    }
}

fn move_notes(time: Res<Time>, mut query: Query<(&mut Transform, &Note)>, speed: Res<Speed>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * speed.0 * NOTE_BASE_SPEED;
        let allow_distance = ERROR_THRESHOLD * NOTE_BASE_SPEED * speed.0;
        let distance_after_target = transform.translation.y - (TARGET_POSITION - allow_distance);
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
    query: Query<(&Transform, &Note, Entity)>,
    mut lane_q: Query<&KeyLane>,
    key_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
    mut ev_writer: EventWriter<CatchNoteEvent>,
    start_time: Res<AudioStartTime>,
    time: Res<Time>,
    song_info: Res<SongConfig>,
    speed: Res<Speed>,
) {
    let time_after_start = time.seconds_since_startup() - start_time.0;
    let mut removed_ent = vec![];
    for lane in lane_q.iter_mut() {
        for (trans, note, ent) in query.iter() {
            let pos_y = trans.translation.y;
            let allow_distance = ERROR_THRESHOLD * NOTE_BASE_SPEED * speed.0;
            if (TARGET_POSITION - allow_distance..=TARGET_POSITION + allow_distance)
                .contains(&pos_y)
                && note.key_column == lane.0
                && lane.key_just_pressed(&key_input)
                && !removed_ent.contains(&ent)
            {
                commands.entity(ent).despawn();
                removed_ent.push(ent);
                score.increase_correct(TARGET_POSITION - pos_y, allow_distance);
                ev_writer.send(CatchNoteEvent::new(
                    note,
                    time_after_start,
                    song_info.bpm,
                    song_info.beat_par_bar,
                ));
            }
        }
    }
}

fn despawn_notes(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Note>>,
    mut score: ResMut<ScoreResource>,
) {
    for (trans, ent) in query.iter() {
        let pos_y = trans.translation.y;
        if pos_y < 2.0 * TARGET_POSITION {
            commands.entity(ent).despawn();
            score.increase_fails();
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
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(despawn_notes));
    }
}
