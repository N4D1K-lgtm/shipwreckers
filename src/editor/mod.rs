mod resource_configs;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

use resource_configs::ResourcesConfigWindow;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_editor_window::<ResourcesConfigWindow>();
    }
}
