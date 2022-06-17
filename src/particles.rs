use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
struct ParticleSize {
    start: f32,
    end: f32,
}

#[derive(Component, Clone, Copy)]
struct ParticleColor {
    start: Color,
    end: Color,
}

#[derive(Component, Clone, Copy)]
struct ParticleVelocity {
    start: Vec2,
    end: Vec2,
}

#[derive(Component)]
struct Particle {
    lifetime: Timer,
}

#[derive(Component)]
struct ParticleSpawnerTimer(Timer);

#[derive(Component)]
struct ParticleSpawner {
    rate: f32,
    amount_per_burst: usize,
    position_variance: f32,
    particle_lifetime: f32,
    particle_size: Option<ParticleSize>,
    particle_color: Option<ParticleColor>,
    particle_velocity: Option<ParticleVelocity>,
}

struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {}
}

// Interpolates two given values with a given multiplier
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

// Interpolates two given vectors with a given multiplier
fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}

// Interpolates two given colors with a given multiplier
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::rgba(
        lerp(a.r(), b.r(), t),
        lerp(a.g(), b.g(), t),
        lerp(a.b(), b.b(), t),
        lerp(a.a(), b.a(), t),
    )
}

fn spawn_particle() {}
