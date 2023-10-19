use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub handle: usize,
    pub rotation_speed: f32, // Current rotation speed
}
