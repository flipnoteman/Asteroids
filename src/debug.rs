use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};

use crate::player::Player;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.insert_resource(WorldInspectorParams {
                enabled: false,
                highlight_changes: true,
                ..default()
            })
            .add_plugin(WorldInspectorPlugin::new())
            .add_system(toggle_debugger);
            app.register_inspectable::<Player>();
        }
    }
}

fn toggle_debugger(mut window_params: ResMut<WorldInspectorParams>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled;
    }
}
