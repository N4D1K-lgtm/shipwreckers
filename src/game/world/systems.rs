use bevy::prelude::*;
use rand::Rng;
use voronoice::{BoundingBox, Point, VoronoiBuilder};

use super::resources::*;

const MAP_WIDTH: f64 = 32.0; // in tiles
const MAP_HEIGHT: f64 = 64.0; // in tiles
const TILE_SIZE: f64 = 64.0; // in pixels

fn get_grid_coordinate(site: Point) -> (i32, i32) {
    let x = site.x / TILE_SIZE;
    let y = site.y / TILE_SIZE;
    (x as i32, y as i32)
}

pub fn generate_voronoi(mut commands: Commands) {
    // 1. Randomize Epicenter Locations
    let mut rng = rand::thread_rng();
    let num_epicenters = 10; // Adjust as needed
    let mut sites = Vec::new();
    for _ in 0..num_epicenters {
        sites.push(Point {
            x: rng.gen_range(0.0..MAP_WIDTH * TILE_SIZE),
            y: rng.gen_range(0.0..MAP_HEIGHT * TILE_SIZE),
        });
    }

    println!("Sites: {:?}", sites);

    // 2. Generate Voronoi Diagram for these Random Points
    let diagram_option = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(BoundingBox::new(
            Point { x: 0.0, y: 0.0 },
            MAP_WIDTH * TILE_SIZE,
            MAP_HEIGHT * TILE_SIZE,
        ))
        .build();

    match diagram_option {
        Some(diagram) => {
            commands.insert_resource(VoronoiResource {
                diagram: Some(diagram),
            });
        }
        None => {
            eprintln!("Failed to build the Voronoi diagram");
        }
    }
}

pub fn generate_islands(command: Commands, voronoi: Res<VoronoiResource>) -> Vec<Island> {
    let mut islands = Vec::new();

    if let Some(diagram) = &voronoi.diagram {
        let sites = diagram.sites();

        for cell in diagram.iter_cells() {
            let site_index = cell.site();
            let site_point = &sites[site_index];
            let grid_coord = get_grid_coordinate(*site_point);
        }
    }
}

pub fn print_islands(islands: &Vec<Island>) {
    for island in islands {
        println!(
            "Epicenter: {:?}, Grid Coordinate: {:?}",
            island.epicenter, island.grid_coordinate
        );
    }
}
