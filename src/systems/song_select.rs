use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    resources::{
        game_scene::NextAppState,
        handles::SongSelectAssetHandles,
        song::{SelectedSong, Speed},
    },
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

struct AlreadyExistEntities(Vec<Entity>);

fn setup_song_select_scene(
    mut commands: Commands,
    already_exist_q: Query<Entity>,
    handles: Res<SongSelectAssetHandles>,
) {
    // シーン遷移時点で存在しているエンティティをすべて保存
    commands.insert_resource(AlreadyExistEntities(already_exist_q.iter().collect_vec()));
    // 背景を出現
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
            ..Default::default()
        },
        texture: handles.background.clone(),
        ..Default::default()
    });
}

fn test_to_nextsong(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::Z) {
        commands.insert_resource(SelectedSong {
            name: "test".to_string(),
            filename: "test.toml".to_string(),
        });
        commands.insert_resource(Speed(1.5));
        commands.insert_resource(NextAppState(AppState::Game));
        state.set(AppState::Loading).unwrap();
    }
}

fn despawn_song_select_scene(
    mut commands: Commands,
    already_exist: Res<AlreadyExistEntities>,
    entity_q: Query<Entity>,
) {
    for ent in entity_q.iter() {
        // もとからあったものではないエンティティをすべて削除する
        if !already_exist.0.contains(&ent) {
            commands.entity(ent).despawn();
        }
    }
}

pub struct SongSelectStatePlugin;
impl Plugin for SongSelectStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::SongSelect).with_system(setup_song_select_scene),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::SongSelect).with_system(test_to_nextsong),
        );
        app.add_system_set(
            SystemSet::on_exit(AppState::SongSelect).with_system(despawn_song_select_scene),
        );
    }
}
