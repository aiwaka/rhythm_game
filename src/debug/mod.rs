mod receptor_info;
mod result;

use bevy::prelude::*;

pub struct AppDebugPlugin;
impl Plugin for AppDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(self::receptor_info::ReceptorInfoPlugin);
        app.add_plugin(self::result::DebugResultPlugin);
    }
}
