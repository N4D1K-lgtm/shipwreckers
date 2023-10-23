use std::time::Duration;

use bevy::{asset::ChangeWatcher, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_editor_pls::prelude::EditorPlugin as BevyEditorPlugin;
mod assets;
mod camera;
mod editor;
mod game;
mod main_menu;

mod prelude;

use assets::AssetsPlugin;
use camera::GameCameraPlugin;
use editor::EditorPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

#[reflect_trait]
pub trait ConfigResource {}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    AssetLoading,
    Setup,
    Menu,
    Game,
    Exit,
}
fn main() {
    App::new()
        .add_state::<AppState>()
        // plugins from bevy or external crates
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shipwreckers".to_owned(),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
            BevyEditorPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        //My game specific plugins
        .add_plugins((
            EditorPlugin,
            GameCameraPlugin,
            AssetsPlugin,
            GamePlugin,
            MainMenuPlugin,
        ))
        .run();
}
