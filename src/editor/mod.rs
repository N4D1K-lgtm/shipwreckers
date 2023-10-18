mod config;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

use config::ConfigEditorWindow;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_editor_window::<ConfigEditorWindow>();
    }
}
