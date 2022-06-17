use crate::asteroids::{Asteroid, RADIUS};
use crate::player::Player;
use crate::{RESOLUTION, TILE_SIZE};
use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy::render::view::visibility;

pub const LIFETIME: f32 = 3.0;

pub struct ProjectilePlugin;

#[derive(Component)]
pub struct Projectile {
    lifetime: Timer,
}

#[derive(Component)]
pub struct ProjectileLifeTime(Timer);

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_projectile)
            .add_system(update_projectile)
            .add_system(check_if_out_of_bounds)
            .add_system(update_projectile_lifetime);
    }
}

fn check_if_out_of_bounds(mut projectile_query: Query<(&Projectile, &mut Transform)>) {
    for (_projectile, mut transform) in projectile_query.iter_mut() {
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
}

fn update_projectile_lifetime(
    mut proj_query: Query<(&mut Projectile, &mut Visibility, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut projectile, mut visibility, mut sprite) in proj_query.iter_mut() {
        projectile.lifetime.tick(time.delta());
        sprite.color = Color::rgba(
            sprite.color.r(),
            sprite.color.g(),
            sprite.color.b(),
            sprite.color.a() - 0.2 * TILE_SIZE,
        );
        if projectile.lifetime.finished() {
            visibility.is_visible = false
        }
    }
}

fn update_projectile(
    mut commands: Commands,
    mut proj_query: Query<(&Projectile, &mut Transform)>,
    time: Res<Time>,
) {
    for (projectile, mut transform) in proj_query.iter_mut() {
        let proj_vel = transform.rotation;

        transform.translation +=
            proj_vel * Vec3::new(2.0 * 15.0 * TILE_SIZE * time.delta_seconds(), 0.0, 0.0);
    }
}

fn spawn_projectile(
    mut commands: Commands,
    mut query: Query<(&Player, &Transform)>,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    let (_player, transform) = query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        let mut bullet = SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(0.6 * TILE_SIZE, 0.07 * TILE_SIZE)),
                ..default()
            },
            transform: *transform,
            ..default()
        };

        commands.spawn_bundle(bullet).insert(Projectile {
            lifetime: Timer::from_seconds(LIFETIME, false),
        });
    }
}
