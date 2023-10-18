use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    simulation_state: Res<State<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match simulation_state.get() {
            SimulationState::Running => {
                next_simulation_state.set(SimulationState::Paused);
                println!("Game Paused");
            }
            SimulationState::Paused => {
                next_simulation_state.set(SimulationState::Running);
                println!("Game Resumed");
            }
        }
    }
}
