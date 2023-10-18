use bevy::prelude::*;
use bevy_editor_pls::editor_window::{EditorWindow, EditorWindowContext};
use bevy_editor_pls::egui;
use bevy_inspector_egui::bevy_inspector;

use crate::config::{NoiseConfig, TileConfig};
use crate::prelude::NoiseGeneratorType;

pub struct ConfigEditorWindow;

pub struct WindowState {
    old_seed: u32,
    frequency: f32,
    noise_texture: Option<egui::TextureHandle>,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            old_seed: 0,
            frequency: 0.0,
            noise_texture: None,
        }
    }
}

impl EditorWindow for ConfigEditorWindow {
    type State = WindowState;
    const NAME: &'static str = "Game Configuration";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let tile_config = world.get_resource::<TileConfig>().unwrap();

        world.resource_scope(|world, mut tile_config: Mut<TileConfig>| {
            let ctx = ui.ctx().clone();
            let Some(state) = cx.state_mut::<ConfigEditorWindow>() else {
                return;
            };
            // UI logic for tile_config
            ui.group(|ui| {
                ui.label("Tile Configuration");
                ui.horizontal(|ui| {
                    ui.label("Tile Size:");
                    // ui.drag_value(&mut tile_config.tile_size.y);
                });
                // ... Add other tile configurations here
            });
        });

        world.resource_scope(|world, mut noise_config: Mut<NoiseConfig>| {
            let ctx = ui.ctx().clone();
            let Some(state) = cx.state_mut::<ConfigEditorWindow>() else {
                return;
            };

            // UI logic for noise_config
            ui.group(|ui| {
                ui.indent("Noise Config", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Seed:");
                        if bevy_inspector::ui_for_value(&mut state.old_seed, ui, world) {
                            match &mut noise_config.generator_type {
                                NoiseGeneratorType::Perlin(perlin_wrapper) => {
                                    perlin_wrapper.seed = state.old_seed;
                                }
                                NoiseGeneratorType::Worley(worley_wrapper) => {
                                    worley_wrapper.seed = state.old_seed;
                                }
                            }
                        }
                    });
                })
                // ... Add other noise configurations here
            });
        });
    }
}
