use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};

use super::nodes::{NodeData, NodeType};
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Debug, Deserialize, Serialize, TypeUuid, TypePath)]
#[uuid = "7b7c2c2e-8c7a-4a7e-9d0b-9b4f5b0f7e5f"]
pub struct NoiseConfig {
    #[serde(default)]
    name: String,
    #[serde(flatten)]
    nodes: NodeData,
}

#[derive(Default)]
pub struct NoiseConfigAssetLoader;

impl AssetLoader for NoiseConfigAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let config_asset: NoiseConfig = serde_json::de::from_slice(bytes)?;

            let serialized_asset = serde_json::to_string_pretty(&config_asset).unwrap();

            print!(
                "Deserialized: \n{:#?}, Serialized: \n{}",
                config_asset, serialized_asset
            );

            // Set the loaded asset
            load_context.set_default_asset(LoadedAsset::new(config_asset));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["noise.json"]
    }
}
