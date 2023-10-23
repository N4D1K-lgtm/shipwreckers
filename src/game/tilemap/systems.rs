use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use super::super::components::ChunkMarker;
use super::super::resources::ChunkManager;

use crate::assets::MyAssets;
use crate::camera::MainGameCamera;

use crate::config::{chunk::ChunkConfig, tilemap::TileConfig};

pub fn spawn_chunk(
    commands: &mut Commands,
    my_assets: &Res<MyAssets>,
    texture_atlases: &Res<Assets<TextureAtlas>>,
    chunk_pos: IVec2,
    tile_config: &TileConfig,
) {
    let _chunk_size = tile_config.chunk_size;
    let _tile_size = tile_config.tile_size;

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(_chunk_size.into());
    // Spawn the elements of the tilemap.
    for x in 0.._chunk_size.x {
        for y in 0.._chunk_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * _chunk_size.x as f32 * _tile_size.x,
        chunk_pos.y as f32 * _chunk_size.y as f32 * _tile_size.y,
        0.0,
    ));

    // Use the loaded texture atlas from MyAssets
    let atlas = texture_atlases
        .get(&my_assets.tilesheet1)
        .expect("Failed to find our atlas");

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size: _tile_size.into(),
            size: _chunk_size.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(atlas.texture.clone()),
            tile_size: _tile_size,
            transform,
            ..Default::default()
        },
        ChunkMarker {},
        Name::new("Chunk"),
    ));
}

pub fn camera_pos_to_chunk_pos(camera_pos: &Vec2, tile_config: &TileConfig) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(
        tile_config.chunk_size.x as i32,
        tile_config.chunk_size.y as i32,
    );
    let tile_size: IVec2 = IVec2::new(
        tile_config.chunk_size.x as i32,
        tile_config.chunk_size.y as i32,
    );
    camera_pos / (chunk_size * tile_size)
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    camera_query: Query<&Transform, (With<Camera>, With<MainGameCamera>)>,
    mut chunk_manager: ResMut<ChunkManager>,
    tile_config: Res<TileConfig>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy(), &tile_config);
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(
                        &mut commands,
                        &my_assets,
                        &texture_atlases,
                        IVec2::new(x, y),
                        &tile_config,
                    );
                }
            }
        }
    }
}

pub fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, (With<Camera>, With<MainGameCamera>)>,
    chunks_query: Query<(Entity, &Transform), With<ChunkMarker>>,
    mut chunk_manager: ResMut<ChunkManager>,
    tile_config: Res<TileConfig>,
    chunk_config: Res<ChunkConfig>,
) {
    let despawn_distance = chunk_config.despawn_distance;
    let chunk_size = tile_config.chunk_size;
    let tile_size = tile_config.tile_size;
    for camera_transform in camera_query.iter() {
        let camera_chunk_pos =
            camera_pos_to_chunk_pos(&camera_transform.translation.xy(), &tile_config);

        // Define bounds based on DESPAWN_DISTANCE and camera_chunk_pos
        let despawn_bounds = (
            camera_chunk_pos - IVec2::new(despawn_distance as i32, despawn_distance as i32),
            camera_chunk_pos + IVec2::new(despawn_distance as i32, despawn_distance as i32),
        );

        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let chunk_tile_pos = IVec2::new(
                (chunk_pos.x / (chunk_size.x as f32 * tile_size.x)).floor() as i32,
                (chunk_pos.y / (chunk_size.y as f32 * tile_size.y)).floor() as i32,
            );

            // Check if chunk_tile_pos is outside the bounds
            if chunk_tile_pos.x < despawn_bounds.0.x
                || chunk_tile_pos.x > despawn_bounds.1.x
                || chunk_tile_pos.y < despawn_bounds.0.y
                || chunk_tile_pos.y > despawn_bounds.1.y
            {
                chunk_manager.spawned_chunks.remove(&chunk_tile_pos);

                if commands.get_entity(entity).is_some() {
                    // Check if entity exists
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
