use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 16, rows = 6))]
    #[asset(path = "Tilesheets/tilesheet1.png")]
    pub tilesheet1: Handle<TextureAtlas>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::Setup),
        )
        .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    }
}
