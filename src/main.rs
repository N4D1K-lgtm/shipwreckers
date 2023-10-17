use bevy::{asset::LoadState, prelude::*, window::PrimaryWindow};

use bevy_asset_loader::prelude::*;
use bevy_editor_pls::prelude::*;

mod game;
use game::GamePlugin;

mod main_menu;
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

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "Tilesheets/tilesheet1.png")]
    pub player: Handle<Image>,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.1),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MainMenuPlugin,
            GamePlugin,
            EditorPlugin::default(),
        ))
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::Setup),
        )
        .add_collection_to_loading_state::<_, GameAssets>(AppState::AssetLoading)
        .add_systems(Startup, spawn_camera)
        .run();
}
