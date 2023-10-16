use bevy::{prelude::*, window::PrimaryWindow};

mod game;
use game::GamePlugin;

mod main_menu;
use main_menu::MainMenuPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
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
        .add_plugins((DefaultPlugins, MainMenuPlugin, GamePlugin))
        .add_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .run();
}
