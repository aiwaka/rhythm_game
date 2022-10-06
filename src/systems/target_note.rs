use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::components::target_line::TargetLine;
use crate::game_constants::TARGET_POSITION;
use crate::resources::handles::GameAssetsHandles;
use crate::AppState;

fn setup_target_notes(mut commands: Commands, handles: Res<GameAssetsHandles>) {
    let transform = Transform {
        translation: Vec3::new(0.0, TARGET_POSITION, 2.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle::from(handles.judge_line.clone()),
            material: handles.color_material_white_trans.clone(),
            transform,
            ..Default::default()
        })
        .insert(TargetLine);
}

pub struct TargetNotePlugin;
impl Plugin for TargetNotePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_target_notes));
    }
}
