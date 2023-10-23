use bevy::prelude::*;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct TilemapResource {
    pub width: usize,
    pub height: usize,
}

impl Default for TilemapResource {
    fn default() -> Self {
        Self {
            width: 128,
            height: 128,
        }
    }
}
