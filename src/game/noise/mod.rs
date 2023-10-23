use bevy::prelude::*;

pub struct NoiseConfigPlugin;

impl Plugin for NoiseConfigPlugin {
    fn name(&self) -> &str {
        "NoiseConfig"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app;
    }
}
