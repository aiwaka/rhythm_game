use bevy::prelude::*;

mod io;
mod note;
mod ui;

pub struct ChartEditorPlugin;
impl Plugin for ChartEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ui::EditorUiPlugin);
        app.add_plugin(note::EditorNotePlugin);
    }
}
