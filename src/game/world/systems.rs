use super::types::TileType;
use super::TilemapResource;

use bevy::log;
use bevy::prelude::*;

pub fn populate_tilemap(mut tilemap: ResMut<TilemapResource>) {
    *tilemap = TilemapResource::generate_worley_tilemap(50, 50, 0.1, 0.5, 20, 1.0);
    log::info!("Populated Tilemap");
}

pub fn print_tilemap_stats(tilemap: Res<TilemapResource>) {
    let mut land_count = 0;
    let mut water_count = 0;

    for row in &tilemap.tiles {
        for tile in row {
            match tile.tile_type {
                TileType::Land => land_count += 1,
                TileType::Water => water_count += 1,
            }
        }
    }

    log::info!(
        "TilemapResource:\n\
         \tWidth: {}\n\
         \tHeight: {}\n\
         \tTotal Tiles: {}\n\
         \tLand Tiles: {}\n\
         \tWater Tiles: {}",
        tilemap.width,
        tilemap.height,
        tilemap.height * tilemap.width,
        land_count,
        water_count
    );
}

pub fn print_tilemap_graphic(tilemap: Option<Res<TilemapResource>>) {
    if let Some(map) = &tilemap {
        for row in &map.tiles {
            let row_string: String = row
                .iter()
                .map(|tile| match tile.tile_type {
                    TileType::Land => 'X',
                    TileType::Water => ' ',
                })
                .collect();
            println!("{}", row_string);
        }
    } else {
        println!("TilemapResource does not exist.");
    }
}
