use bevy::prelude::*;
use bevy_editor_pls::editor_window::{EditorWindow, EditorWindowContext};
use bevy_editor_pls::egui;
use bevy_inspector_egui::bevy_inspector;
use std::collections::HashMap;

use crate::config::{NoiseConfig, TileConfig};
use crate::prelude::NoiseGeneratorType;

pub struct ConfigEditorWindow;

pub struct WindowState {
    states: HashMap<String, Box<dyn bevy::reflect::Reflect>>,
    noise_texture: Option<egui::TextureHandle>,
}

impl Default for WindowState {
    fn default() -> Self {
        let mut states = HashMap::new();

        states.insert(
            "NoiseConfig".to_string(),
            Box::<NoiseConfig>::default() as Box<dyn Reflect>,
        );

        states.insert(
            "TileConfig".to_string(),
            Box::<TileConfig>::default() as Box<dyn Reflect>,
        );

        // ... add other configs as needed ...

        Self {
            states,
            noise_texture: None,
        }
    }
}

impl EditorWindow for ConfigEditorWindow {
    type State = WindowState;
    const NAME: &'static str = "Game Configuration";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let Some(state) = cx.state_mut::<ConfigEditorWindow>() else {
            return;
        };

        // Tile Configuration UI
        ui.collapsing("Tile Configuration", |ui| {
            if let Some(tile_config_state) = state.states.get_mut("TileConfig") {
                if let Some(tile_config) = tile_config_state.downcast_mut::<TileConfig>() {
                    ui.vertical(|ui| {
                        ui.label("Tile Size:");
                        bevy_inspector::ui_for_value(&mut tile_config.tile_size, ui, world);
                    });
                    if ui.button("Submit Tile Config").clicked() {
                        world.resource_scope(|_world, mut tile_config_res: Mut<TileConfig>| {
                            tile_config_res.tile_size = tile_config.tile_size;
                        });
                    }
                }
            }
        });

        // Noise Configuration UI
        ui.collapsing("Noise Configuration", |ui| {
            if let Some(noise_config_state) = state.states.get_mut("NoiseConfig") {
                if let Some(noise_config) = noise_config_state.downcast_mut::<NoiseConfig>() {
                    ui.vertical(|ui| {
                        ui.label("Seed:");
                        match &mut noise_config.generator_type {
                            NoiseGeneratorType::Perlin(perlin_wrapper) => {
                                bevy_inspector::ui_for_value(&mut perlin_wrapper.seed, ui, world);
                            }
                            NoiseGeneratorType::Worley(worley_wrapper) => {
                                bevy_inspector::ui_for_value(&mut worley_wrapper.seed, ui, world);
                            }
                        }
                    });
                    if ui.button("Submit Noise Config").clicked() {
                        world.resource_scope(|_world, mut noise_config_res: Mut<NoiseConfig>| {
                            noise_config_res.generator_type = noise_config.generator_type.clone();
                        });
                    }
                }
            }
        });
    }
}
