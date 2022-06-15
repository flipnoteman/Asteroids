use std::cmp::min;
use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget::Window;
use bevy::render::render_resource::Texture;
use bevy::transform;
use bevy::window::WindowId;
use bevy_inspector_egui::Inspectable;
use crate::{RESOLUTION, TILE_SIZE};
use crate::asteroids::{Asteroid, RADIUS};

const PLAYER_VELOCITY_BOUND: f32 = 1.0;
const MAX_SPEED: f32 = 15.0;

pub struct PlayerPlugin;


#[derive(Component, Inspectable, Clone, Copy)]
pub struct Player {
    speed: f32,
    velocity: Vec3
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(check_if_out_of_bounds.before(move_player))
            .add_system(check_collision_with_asteroid.before(check_if_out_of_bounds));
    }
}

fn check_collision_with_asteroid(asteroid_query: Query<(&Asteroid, &Transform)>, player_query: Query<(&Player, &Transform)>){
    let (_player, transform) = player_query.single();
    let mut distance = 150000.0f32;


    for (_asteroid, ast_trans) in asteroid_query.iter() {
        let x = transform.translation.x - ast_trans.translation.x;
        let y = transform.translation.y - ast_trans.translation.y;
        let x_square = x.powf(2.0);
        let y_square = y.powf(2.0);
        distance = (x_square.abs() + y_square.abs()).sqrt();
        println!("{}", distance);
        if distance < RADIUS {
            println!("True");
        }
    }
    println!("False");
}


fn check_if_out_of_bounds(mut player_query: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = player_query.single_mut();

    if transform.translation.x > 1.0 * RESOLUTION {
        transform.translation.x = -0.99 * RESOLUTION;
    }
    if transform.translation.x < -1.0 * RESOLUTION {
        transform.translation.x = 0.99 * RESOLUTION;
    }
    if transform.translation.y > 1.0 {
        transform.translation.y = -0.99;
    }
    if transform.translation.y < -1.0 {
        transform.translation.y = 0.99;
    }
}

fn move_player (
    input: Res<Input<KeyCode>>, mut query: Query<(&mut Player, &mut Transform)>, time: Res<Time>
) {
    let (mut player, mut transform) = query.single_mut();

    if input.pressed(KeyCode::Right) {
        transform.rotation *= (Quat::from_rotation_z(2.0 * MAX_SPEED * TILE_SIZE * -time.delta_seconds()));
    }
    if input.pressed(KeyCode::Left) {
        transform.rotation *= (Quat::from_rotation_z(2.0 * MAX_SPEED * TILE_SIZE * time.delta_seconds()));
    }
    if input.pressed(KeyCode::Up) {
        player.velocity += transform.rotation * Vec3::new(0.5, 0.0, 0.0);
        player.velocity = Vec3::clamp(
            player.velocity,
            Vec3::new(-MAX_SPEED, -MAX_SPEED, 0.0),
            Vec3::new(MAX_SPEED, MAX_SPEED, 0.0)
        );
    }
    transform.translation += player.velocity * TILE_SIZE * time.delta_seconds();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("ships/ship_2.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0 * TILE_SIZE, 1.0 * TILE_SIZE)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..default()
        },
        ..default() })
        .insert(Name::new("Player"))
        .insert(Player { speed: 0.5, velocity: Vec3::new(1.0, 1.0, 0.0) });
}