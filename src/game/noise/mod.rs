use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppState;

mod asset_loader;
mod constant_types;
mod nodes;

use asset_loader::{NoiseConfig, NoiseConfigAssetLoader};

#[derive(AssetCollection, Resource)]
pub struct NoiseConfigs {
    #[asset(path = "configs/noise/", collection(typed))]
    folder_config: Vec<Handle<NoiseConfig>>,
}

pub struct NoisePlugin;

impl Plugin for NoisePlugin {
    fn name(&self) -> &str {
        "NoiseConfig"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<NoiseConfig>()
            .init_asset_loader::<NoiseConfigAssetLoader>()
            .add_collection_to_loading_state::<_, NoiseConfigs>(AppState::AssetLoading);
    }
}
