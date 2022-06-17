mod asteroids;
mod debug;
mod particles;
mod player;
mod projectile;

use crate::asteroids::AsteroidPlugin;
use crate::debug::DebugPlugin;
use crate::player::PlayerPlugin;
use crate::projectile::ProjectilePlugin;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;
pub const HEIGHT: f32 = 900.0;
const CLEAR: Color = Color::BLACK;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Asteroids".to_string(),
            resizable: false,
            ..default()
        })
        .insert_resource(WorldInspectorPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsteroidPlugin)
        .add_plugin(ProjectilePlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
