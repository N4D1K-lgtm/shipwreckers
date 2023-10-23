use crate::AppState;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};

use bevy_asset_loader::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 16, rows = 6))]
    #[asset(path = "Tilesheets/tilesheet1.png")]
    pub tilesheet1: Handle<TextureAtlas>,

    #[asset(path = "configs/noise/", collection(typed))]
    folder_config: Vec<Handle<NoiseConfig>>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<NoiseConfig>()
            .init_asset_loader::<CustomAssetLoader>()
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::Setup),
            )
            .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    }
}

#[derive(Debug, Deserialize, Serialize, TypeUuid, TypePath)]
#[uuid = "7b7c2c2e-8c7a-4a7e-9d0b-9b4f5b0f7e5f"]
pub struct NoiseConfig {
    name: String,
    seed: i32,
}

#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<NoiseConfig>(bytes)?;
            println!("{:?}", custom_asset);
            load_context.set_default_asset(LoadedAsset::new(NoiseConfig { ..custom_asset }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["conf.ron"]
    }
}

fn print_on_asset_changed(config_assets: Res<MyAssets>) {}
