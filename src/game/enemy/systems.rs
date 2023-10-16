use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;

use super::NUM_ENEMIES;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUM_ENEMIES {
        let random_y = random::<f32>() * window.height();
        let random_x = random::<f32>() * window.width();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("Ships/ship-black.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}
