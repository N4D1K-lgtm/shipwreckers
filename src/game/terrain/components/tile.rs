use bevy::prelude::*;

use super::coordinates::Coordinates;

// Represents individual tiles
#[derive(Debug, Component)]
pub struct TileComponent {
    pub tile_type: TileType,
    pub world_location: Coordinates, // Global position
    pub chunk_position: Coordinates, // Relative position within its chunk
}

// Represents chunks
#[derive(Debug, Default, Component)]
pub struct TileChunkComponent {
    pub start_location: Coordinates,
    pub tiles: Vec<TileComponent>,
}

// Marker component for interactive tiles
#[derive(Debug, Default, Component)]
pub struct InteractiveTileEntity {}

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Land,
    Water,
}
