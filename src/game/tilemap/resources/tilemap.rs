use bevy::prelude::Resource;

#[derive(Resource, Debug)]
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
