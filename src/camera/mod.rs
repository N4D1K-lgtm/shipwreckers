use bevy::{prelude::*, window::PrimaryWindow};
pub mod movement;
use crate::AppState;

#[derive(Component)]
pub struct MainGameCamera;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.1),
            ..default()
        },
        MainGameCamera {},
    ));
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, movement::movement.run_if(in_state(AppState::Game)));
    }
}
