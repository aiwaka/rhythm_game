mod receptor_info;
mod result;
mod song_select;

use bevy::prelude::*;

/// デバッグ用ユーティリティを外部に公開するためのモジュール
pub mod utilities {
    pub use super::receptor_info::boolean_string;
}

pub struct AppDebugPlugin;
impl Plugin for AppDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(self::receptor_info::ReceptorInfoPlugin);
        app.add_plugin(self::result::DebugResultPlugin);
        app.add_plugin(self::song_select::DebugSongSelectPlugin);
    }
}
