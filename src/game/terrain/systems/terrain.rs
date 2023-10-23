use crate::prelude::*;

use bevy::log;
use bevy::prelude::*;

pub fn create_tilemap_chunks(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    tilemap: Res<TilemapResource>,
    noise: Res<NoiseResource>,
) {
    let chunk_size = 16; // or whatever you prefer

    for y in (0..tilemap.height).step_by(chunk_size) {
        for x in (0..tilemap.width).step_by(chunk_size) {
            let mut tiles = Vec::with_capacity(chunk_size * chunk_size);

            for cy in 0..chunk_size {
                for cx in 0..chunk_size {
                    let global_x = x + cx;
                    let global_y = y + cy;
                    let noise_val = noise.get_noise_value(global_x as f64, global_y as f64);

                    let tile_type = if noise_val > noise.threshold {
                        TileType::Land
                    } else {
                        TileType::Water
                    };

                    tiles.push(TileComponent {
                        tile_type,
                        world_location: Coordinates {
                            x: global_x,
                            y: global_y,
                        },
                        chunk_position: Coordinates { x: cx, y: cy },
                    });
                }
            }

            // Spawn the chunk entity with the tiles Vec populated
            commands.spawn(TileChunkComponent {
                start_location: Coordinates { x, y },
                tiles,
            });
        }
    }
    next_state.set(AppState::Game);
    log::info!("Finished creating tilemap chunks");
}

pub fn log_tilemap_info(chunk_query: Query<&TileChunkComponent>) {
    let chunk_count = chunk_query.iter().count();
    let mut land_count = 0;
    let mut water_count = 0;

    // Gather general stats about the tilemap
    for chunk in chunk_query.iter() {
        for tile in &chunk.tiles {
            match tile.tile_type {
                TileType::Land => land_count += 1,
                TileType::Water => water_count += 1,
            }
        }
    }

    // Print header
    log::info!("========== WORLD INFO ==========");
    log::info!("Total chunks: {}", chunk_count);

    // Using separators for better readability
    log::info!("--------------------------");

    // Tile stats
    log::info!("Total Land Tiles: {}", land_count);
    log::info!("Total Water Tiles: {}", water_count);

    // Calculate percentages
    let total_tiles = land_count + water_count;
    if total_tiles > 0 {
        let land_percentage = (land_count as f64 / total_tiles as f64) * 100.0;
        let water_percentage = 100.0 - land_percentage;
        log::info!("Land Percentage: {:.2}%", land_percentage);
        log::info!("Water Percentage: {:.2}%", water_percentage);
    }

    // Closing separator
    log::info!("===============================");
}
