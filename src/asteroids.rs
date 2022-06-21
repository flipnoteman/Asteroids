use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::{thread_rng, Rng};
use bevy_rapier2d::prelude::*;

use crate::projectile::Projectile;
use crate::{RESOLUTION, TILE_SIZE};

pub const RADIUS: f32 = 4.0 * TILE_SIZE;
const ASTEROID_MAX_SPEED: f32 = 6.0;

enum SpawnSide {
    Top,
    Right,
    Left,
    Bottom,
}

pub struct AsteroidPlugin;

#[derive(Component, Inspectable)]
pub struct Asteroid {
    velocity: Vec3,
}

#[derive(Component)]
pub struct AsteroidTimer(Timer);

#[derive(Component)]
struct AsteroidSpawner {
    rate: f32,
    amount_per_burst: u8,
    timer: Option<AsteroidTimer>,
}

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_asteroid)
            .add_system(check_if_out_of_bounds.before(update_asteroids))
            .add_system(update_asteroids);
    }
}

fn update_asteroids(mut ast_query: Query<(&Asteroid, &mut Transform)>, time: Res<Time>) {
    for (asteroid, mut transform) in ast_query.iter_mut() {
        transform.translation += asteroid.velocity * time.delta_seconds() * TILE_SIZE;
    }
}

fn gen_side() -> SpawnSide {
    let mut rng = thread_rng();
    match rng.gen_range(0..4) {
        0 => SpawnSide::Top,
        1 => SpawnSide::Bottom,
        2 => SpawnSide::Right,
        3 => SpawnSide::Left,
        _ => SpawnSide::Top,
    }
}

fn outside_bounds_vector(side: SpawnSide) -> Vec3 {
    let top_min = 1.0;
    let top_max = 1.0 + RADIUS;
    let bottom_min = -1.0 - RADIUS;
    let bottom_max = -1.0;
    let right_min = 1.0 * RESOLUTION;
    let right_max = (1.0 + RADIUS) * RESOLUTION;
    let left_min = (-1.0 - RADIUS) * RESOLUTION;
    let left_max = -1.0 - RESOLUTION;

    let result = match side {
        SpawnSide::Top => Vec3::new(gen_val(left_min, right_max), gen_val(top_min, top_max), 0.0),
        SpawnSide::Bottom => Vec3::new(
            gen_val(left_min, right_max),
            gen_val(bottom_min, bottom_max),
            0.0,
        ),
        SpawnSide::Right => Vec3::new(
            gen_val(right_min, right_max),
            gen_val(bottom_min, top_max),
            0.0,
        ),
        SpawnSide::Left => Vec3::new(
            gen_val(left_min, left_max),
            gen_val(bottom_min, top_max),
            0.0,
        ),
    };
    result
}

fn gen_val(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

fn check_if_out_of_bounds(mut player_query: Query<(&Asteroid, &mut Transform)>) {
    let (player, mut transform) = player_query.single_mut();

    if transform.translation.x > (1.0 + RADIUS * 0.5) * RESOLUTION {
        transform.translation.x = (-0.99 - RADIUS * 0.5) * RESOLUTION;
    }
    if transform.translation.x < (-1.0 - RADIUS * 0.5) * RESOLUTION {
        transform.translation.x = (0.99 + RADIUS * 0.5) * RESOLUTION;
    }
    if transform.translation.y > (1.0 + RADIUS * 0.5) * RESOLUTION {
        transform.translation.y = (-0.99 - RADIUS * 0.5) * RESOLUTION;
    }
    if transform.translation.y < (-1.0 - RADIUS * 0.5) * RESOLUTION {
        transform.translation.y = (0.99 + RADIUS * 0.5) * RESOLUTION;
    }
}

fn spawn_asteroid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();
    let asteroid_speed: f32 = rng.gen_range(-ASTEROID_MAX_SPEED..ASTEROID_MAX_SPEED);
    let random_location = outside_bounds_vector(gen_side());
    let random_asteroid: u16 = rng.gen_range(1..=3);
    let start_asteroid = match random_asteroid {
        1 => "asteroids/larg_asteroid_1.png",
        2 => "asteroids/larg_asteroid_2.png",
        3 => "asteroids/larg_asteroid_3.png",
        _ => "asteroids/larg_asteroid_1.png",
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(RADIUS, RADIUS)),
                ..default()
            },
            texture: asset_server.load(start_asteroid),
            transform: Transform::from_xyz(random_location.x, random_location.y, 0.0),
            ..default()
        })
        .insert(Asteroid {
            velocity: Vec3::new(
                gen_val(-ASTEROID_MAX_SPEED, ASTEROID_MAX_SPEED),
                gen_val(-ASTEROID_MAX_SPEED, ASTEROID_MAX_SPEED),
                0.0,
            ),
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2::new(1.0 * RESOLUTION * TILE_SIZE, 2.0 * RESOLUTION * TILE_SIZE),
            angvel: 0.2
        })
        .insert(GravityScale(0.1))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}
