use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::prelude::*;

pub const PLAYER_ROTATION_SPEED: f32 = 360.0; // Degrees per second
pub const ACCELERATION: f32 = 300.0;
pub const ROTATION_LERP_FACTOR: f32 = 0.1;

pub fn spawn_player(
    mut world: &World,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    player_handle: Res<LocalPlayerHandle>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("Ships/ship-white.png"),
            ..default()
        },
        Player {
            velocity: Vec3::ZERO,
            rotation_speed: 0.0,
            handle: player_handle.0,
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut acceleration = Vec3::ZERO;

        // Define the forward vector based on the ship's current rotation.
        let forward = transform.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));

        if keyboard_input.pressed(KeyCode::W) {
            acceleration -= forward;
        }
        if keyboard_input.pressed(KeyCode::S) {
            acceleration += forward;
        }

        // Determine target rotation speed based on A/D input
        let target_rotation_speed = if keyboard_input.pressed(KeyCode::A) {
            PLAYER_ROTATION_SPEED
        } else if keyboard_input.pressed(KeyCode::D) {
            -PLAYER_ROTATION_SPEED
        } else {
            0.0
        };

        // Gradually adjust the current rotation speed towards the target
        player.rotation_speed = player.rotation_speed * (1.0 - ROTATION_LERP_FACTOR)
            + target_rotation_speed * ROTATION_LERP_FACTOR;

        // Apply rotation
        transform.rotation *=
            Quat::from_rotation_z(player.rotation_speed.to_radians() * time.delta_seconds());

        // Apply acceleration to velocity
        player.velocity += acceleration * ACCELERATION * time.delta_seconds();

        // Update position based on velocity
        transform.translation += player.velocity * time.delta_seconds();

        // Apply some drag to reduce velocity over time
        player.velocity *= 0.98;
    }
}
