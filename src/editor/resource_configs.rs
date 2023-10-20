use bevy::prelude::*;

use crate::config::ReflectConfigResource;

use bevy_editor_pls::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::bevy_inspector::by_type_id;

use bevy_inspector_egui::egui;
pub struct ResourcesConfigWindow;

impl EditorWindow for ResourcesConfigWindow {
    type State = ();

    const NAME: &'static str = "Game Config Resources";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        let type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        let mut resources: Vec<_> = type_registry
            .iter()
            .filter(|registration| registration.data::<ReflectConfigResource>().is_some())
            .map(|registration| (registration.short_name().to_owned(), registration.type_id()))
            .collect();
        resources.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));
        for (name, type_id) in resources {
            ui.collapsing(&name, |ui| {
                by_type_id::ui_for_resource(world, type_id, ui, &name, &type_registry);
            });
        }
    }
}
