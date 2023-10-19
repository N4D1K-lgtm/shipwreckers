use bevy::prelude::*;
use bevy_editor_pls::prelude::EditorPlugin as BevyEditorPlugin;

mod assets;
mod camera;
mod config;
mod editor;
mod game;
mod main_menu;

mod prelude;

use assets::AssetsPlugin;
use camera::GameCameraPlugin;
use config::ConfigPlugin;
use editor::EditorPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

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
                .set(ImagePlugin::default_nearest()),
            BevyEditorPlugin::default(),
        ))
        //My game specific plugins
        .add_plugins((
            EditorPlugin,
            GameCameraPlugin,
            AssetsPlugin,
            GamePlugin,
            MainMenuPlugin,
            ConfigPlugin,
        ))
        .run();
}
