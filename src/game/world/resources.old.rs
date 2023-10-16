use super::components::{Coordinates, Tile, TileType};
use bevy::prelude::Resource;
use noise::{NoiseFn, Perlin, Worley};
use rand::random;

#[derive(Resource, Default, Debug)]
pub struct TilemapResource {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl TilemapResource {
    /// Generates an empty map
    pub fn empty(width: usize, height: usize) -> Self {
        let tiles = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| Tile {
                        tile_type: TileType::Water,
                        location: Coordinates { x, y },
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        Self {
            height,
            width,
            tiles,
        }
    }

    pub fn generate_perlin_tilemap(
        width: usize,
        height: usize,
        scale: f64,
        threshold: f64,
    ) -> TilemapResource {
        let perlin = Perlin::new(random());

        let tiles = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let noise_val = perlin.get([x as f64 * scale, y as f64 * scale]);
                        let tile_type = if noise_val > threshold {
                            TileType::Land
                        } else {
                            TileType::Water
                        };

                        Tile {
                            tile_type,
                            location: Coordinates { x, y },
                        }
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        TilemapResource {
            width,
            height,
            tiles,
        }
    }

    pub fn generate_worley_tilemap(
        width: usize,
        height: usize,
        scale: f64,
        threshold: f64,
        seed: u32,
        frequency_value: f64,
    ) -> TilemapResource {
        let worley = Worley::new(seed).set_frequency(frequency_value);

        let tiles = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let noise_val = worley.get([x as f64 * scale, y as f64 * scale]);
                        dbg!(noise_val);
                        let tile_type = if noise_val > threshold {
                            TileType::Land
                        } else {
                            TileType::Water
                        };

                        Tile {
                            tile_type,
                            location: Coordinates { x, y },
                        }
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        TilemapResource {
            width,
            height,
            tiles,
        }
    }

    // Get a specific tile at the given coordinates.
    // Return None if coordinates are out of bounds.

    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }

    // Get the neighbors of a tile at the given coordinates.
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<&Tile> {
        let mut neighbors = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx != 0 || dy != 0 {
                    if let Some(neighbor) =
                        self.get_tile((x as isize + dx) as usize, (y as isize + dy) as usize)
                    {
                        neighbors.push(neighbor);
                    }
                }
            }
        }
        neighbors
    }
}
