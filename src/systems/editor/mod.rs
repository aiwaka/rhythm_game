use bevy::prelude::*;

mod io;
mod ui;

pub struct ChartEditorPlugin;
impl Plugin for ChartEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ui::EditorUiPlugin);
    }
}
