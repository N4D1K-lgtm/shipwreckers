use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};

use serde::{Deserialize, Serialize};

use super::noise_types::{
    default_frequency, default_lacunarity, default_octaves, default_persistence, NoiseType,
    ReturnType, Seed,
};

#[derive(Debug, Deserialize, Serialize, TypeUuid, TypePath)]
#[uuid = "7b7c2c2e-8c7a-4a7e-9d0b-9b4f5b0f7e5f"]
pub struct NoiseConfig {
    name: String,
    noise_type: NoiseType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoiseConfigDescriptor {
    name: String,
    variant: String,
    seed: Option<Seed>,
    return_type: Option<ReturnType>,
    lacunarity: Option<f64>,
    octaves: Option<usize>,
    frequency: Option<f64>,
    persistence: Option<f64>,
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
            // Deserialize the bytes into a NoiseConfigDescriptor
            let descriptor: NoiseConfigDescriptor = ron::de::from_bytes(bytes)?;

            // Convert the descriptor into a NoiseConfig
            let custom_asset: NoiseConfig = descriptor.into();

            bevy::log::info!("Loaded NoiseConfig: {:#?}", custom_asset); // Pretty print using {:#?}

            // Set the loaded asset
            load_context.set_default_asset(LoadedAsset::new(custom_asset));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["noise.ron"]
    }
}

impl From<NoiseConfigDescriptor> for NoiseConfig {
    fn from(repr: NoiseConfigDescriptor) -> Self {
        let noise_type = match repr.variant.as_str() {
            "Perlin" => NoiseType::Perlin(repr.seed.unwrap_or_default()),
            "OpenSimplex" => NoiseType::OpenSimplex(repr.seed.unwrap_or_default()),
            "SuperSimplex" => NoiseType::SuperSimplex(repr.seed.unwrap_or_default()),
            "Value" => NoiseType::Value(repr.seed.unwrap_or_default()),
            "Worley" => NoiseType::Worley {
                return_type: repr.return_type.unwrap_or_default(),
                frequency: repr.frequency.unwrap_or(default_frequency()),
            },
            "Fbm" => NoiseType::Fbm {
                octaves: repr.octaves.unwrap_or(default_octaves()),
                frequency: repr.frequency.unwrap_or(default_frequency()),
                lacunarity: repr.lacunarity.unwrap_or(default_lacunarity()),
                persistence: repr.persistence.unwrap_or(default_persistence()),
            },
            "Billow" => NoiseType::Billow {
                octaves: repr.octaves.unwrap_or(default_octaves()),
                frequency: repr.frequency.unwrap_or(default_frequency()),
                lacunarity: repr.lacunarity.unwrap_or(default_lacunarity()),
                persistence: repr.persistence.unwrap_or(default_persistence()),
            },
            "HybridMulti" => NoiseType::HybridMulti {
                octaves: repr.octaves.unwrap_or(default_octaves()),
                frequency: repr.frequency.unwrap_or(default_frequency()),
                lacunarity: repr.lacunarity.unwrap_or(default_lacunarity()),
                persistence: repr.persistence.unwrap_or(default_persistence()),
            },
            _ => panic!("Unknown variant"), // or handle this case more gracefully
        };

        NoiseConfig {
            name: repr.name,
            noise_type,
        }
    }
}
