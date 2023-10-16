use bevy::prelude::*;
use voronoice::Point;

#[derive(Resource, Debug, Component, Clone)]
pub struct VoronoiResource {
    pub diagram: Option<voronoice::Voronoi>,
}

impl Default for VoronoiResource {
    fn default() -> Self {
        Self { diagram: None }
    }
}

#[derive(Resource)]
struct Island {
    epicenter: Point,
    grid_coordinate: (i32, i32),
}
