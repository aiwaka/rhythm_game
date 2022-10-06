use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::components::note::{SongConfig, Speed};
use crate::game_constants::{SPAWN_POSITION, TARGET_POSITION, THRESHOLD};
use crate::resources::handles::GameAssetsHandles;
use crate::resources::score::ScoreResource;
use crate::AppState;
use crate::{components::note::Note, resources::note::SpawnTimer};

fn spawn_notes(
    mut commands: Commands,
    textures: Res<GameAssetsHandles>,
    mut song_config: ResMut<SongConfig>,
    time: Res<Time>,
) {
    // 現在スタートから何秒経ったかと前の処理が何秒だったかを取得する. 3秒遅らせることで開始準備カウントとなる.
    let secs = time.seconds_since_startup() - 3.0;
    let secs_last = secs - time.delta_seconds_f64();

    let mut remove_counter = 0;
    for note in song_config.notes.iter() {
        if secs_last < note.spawn_time && note.spawn_time < secs {
            remove_counter += 1;

            let note_mesh = textures.note.clone();
            let color = match note.speed {
                Speed::Slow => textures.color_material_red.clone(),
                Speed::Medium => textures.color_material_blue.clone(),
                Speed::Fast => textures.color_material_green.clone(),
            };

            info!("spawn at {}", note.key_column.0);
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
                    speed: note.speed,
                    key_column: note.key_column,
                });
        } else {
            break;
        }
    }
    for _ in 0..remove_counter {
        song_config.notes.remove(0);
    }
}

fn move_notes(time: Res<Time>, mut query: Query<(&mut Transform, &Note)>) {
    for (mut transform, note) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * note.speed.value();
        // info!("y: {}", transform.translation.y);
        let distance_after_target = transform.translation.y - (TARGET_POSITION + THRESHOLD);
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
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_notes));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(despawn_notes));
    }
}
