mod receptor_info;

use bevy::prelude::*;

pub struct AppDebugPlugin;
impl Plugin for AppDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(self::receptor_info::ReceptorInfoPlugin);
    }
}
