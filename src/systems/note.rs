use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::game_constants::{NOTE_BASE_SPEED, SPAWN_POSITION, TARGET_POSITION, THRESHOLD};
use crate::resources::handles::GameAssetsHandles;
use crate::resources::note::{AudioStartTime, Speed};
use crate::resources::score::ScoreResource;
use crate::resources::song::SongConfig;
use crate::AppState;
use crate::{components::note::Note, resources::note::SpawnTimer};

use super::system_labels::TimerSystemLabel;

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
            translation: Vec3::new(note.key_column.x_coord(), SPAWN_POSITION, 1.0),
            ..Default::default()
        };
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(note_mesh),
                material: color,
                transform,
                ..Default::default()
            })
            .insert(Note {
                key_column: note.key_column,
            });
    }
}

fn move_notes(time: Res<Time>, mut query: Query<(&mut Transform, &Note)>, speed: Res<Speed>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * speed.0 * NOTE_BASE_SPEED;
        // info!("y: {}", transform.translation.y);
        let distance_after_target = transform.translation.y - (TARGET_POSITION - THRESHOLD);
        if distance_after_target < -0.02 {
            transform.rotate_axis(Vec3::Z, 0.1);
            transform.scale = (transform.scale
                - time.delta_seconds() * distance_after_target * 0.01)
                .clamp_length(0.0, 100.0);
        }
    }
}

fn despawn_notes(
    mut commands: Commands,
    query: Query<(&Transform, &Note, Entity)>,
    key_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
) {
    for (trans, note, ent) in query.iter() {
        let pos = trans.translation.y;

        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && note.key_column.key_just_pressed(&key_input)
        {
            commands.entity(ent).despawn();
            score.increase_correct(TARGET_POSITION - pos);
        }

        if pos < 2.0 * TARGET_POSITION {
            commands.entity(ent).despawn();
            score.increase_fails();
        }
    }
}

pub struct NotePlugin;
impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)));
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_notes.label(TimerSystemLabel::StartAudio)),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(despawn_notes));
    }
}
