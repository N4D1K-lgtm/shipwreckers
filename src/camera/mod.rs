use bevy::{prelude::*, render::camera::ScalingMode};

pub mod movement;

use crate::AppState;

use movement::camera_follow;

#[derive(Component)]
pub struct MainGameCamera;

pub fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn((camera_bundle, MainGameCamera {}));
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            movement::camera_follow.run_if(in_state(AppState::Game)),
        );
    }
}
