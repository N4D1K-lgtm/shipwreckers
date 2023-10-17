use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

mod assets;
mod camera;
mod game;
mod main_menu;
mod prelude;

use assets::AssetsPlugin;
use camera::GameCameraPlugin;
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
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Shipwreckers"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            MainMenuPlugin,
            GamePlugin,
            EditorPlugin::default(),
            GameCameraPlugin,
            AssetsPlugin,
        ))
        .run();
}
