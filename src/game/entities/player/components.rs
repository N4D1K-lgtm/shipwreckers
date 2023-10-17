use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub rotation_speed: f32, // Current rotation speed
}
