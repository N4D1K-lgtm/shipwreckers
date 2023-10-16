use super::components::Coordinates;
use super::components::{TileChunkComponent, TileComponent, TileType};
use super::resources::{NoiseResource, TilemapResource};
use bevy::log;
use bevy::prelude::*;

pub fn create_tilemap_chunks(
    mut commands: Commands,
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
    log::info!("Finished creating tilemap chunks");
}

pub fn print_tilemap_data(chunk_query: Query<&TileChunkComponent>) {
    let chunk_count = chunk_query.iter().count();
    println!("Total chunks found in the query: {}", chunk_count);
    for chunk in chunk_query.iter() {
        println!("Chunk starting at: {:?}", chunk.start_location);
        for tile in &chunk.tiles {
            println!(
                "Tile at global location: {:?}, chunk position: {:?}, type: {:?}",
                tile.world_location, tile.chunk_position, tile.tile_type
            );
        }
    }
}

// pub fn print_tilemap_graphic(tilemap: Option<Res<TilemapResource>>) {
//     if let Some(map) = &tilemap {
//         for row in &map.tiles {
//             let row_string: String = row
//                 .iter()
//                 .map(|tile| match tile.tile_type {
//                     TileType::Land => 'X',
//                     TileType::Water => ' ',
//                 })
//                 .collect();
//             println!("{}", row_string);
//         }
//     } else {
//         println!("TilemapResource does not exist.");
//     }
// }
