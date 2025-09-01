use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::{BouncyBall, InitialSpeed, TrailSpawner};
use crate::constants::{BALL_RADIUS, MIN_SPEED_FACTOR};

pub fn get_random_neon_color() -> Color {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let colors = [
        Color::srgb(1.0, 0.0, 1.0),
        Color::srgb(0.0, 1.0, 1.0),
        Color::srgb(1.0, 1.0, 0.0),
        Color::srgb(1.0, 0.5, 0.0),
        Color::srgb(0.5, 1.0, 0.0),
        Color::srgb(1.0, 0.0, 0.5),
        Color::srgb(0.0, 0.5, 1.0),
        Color::srgb(0.5, 0.0, 1.0),
    ];
    if count < 8 { colors[count as usize] } else { let mut rng = rand::rng(); colors[rng.random_range(0..8)] }
}

pub fn spawn_ball_with_physics(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..2.0 * std::f32::consts::PI);
    let radius = rng.random_range(0.0..100.0);
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    let velocity_x = rng.random_range(-400.0..400.0);
    let velocity_y = rng.random_range(-400.0..400.0);
    let ball_color = get_random_neon_color();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from(ball_color))),
        Transform::from_xyz(x, y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity { linvel: Vec2::new(velocity_x, velocity_y), angvel: 0.0 },
        Restitution { coefficient: 1.0, combine_rule: CoefficientCombineRule::Max },
        Friction::coefficient(0.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        GravityScale(0.0),
        Ccd::enabled(),
        Sleeping::disabled(),
        InitialSpeed(Vec2::new(velocity_x, velocity_y).length()),
        BouncyBall { color: ball_color },
        TrailSpawner { since_last: 0.0 },
    ));
}

pub fn spawn_ball_at_center_with_physics(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let offset_x = rng.random_range(-20.0..20.0);
    let offset_y = rng.random_range(-20.0..20.0);
    let velocity_x = rng.random_range(-400.0..400.0);
    let velocity_y = rng.random_range(-400.0..400.0);
    let ball_color = get_random_neon_color();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from(ball_color))),
        Transform::from_xyz(offset_x, offset_y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity { linvel: Vec2::new(velocity_x, velocity_y), angvel: 0.0 },
        Restitution { coefficient: 1.0, combine_rule: CoefficientCombineRule::Max },
        Friction::coefficient(0.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        GravityScale(0.0),
        Ccd::enabled(),
        Sleeping::disabled(),
        InitialSpeed(Vec2::new(velocity_x, velocity_y).length()),
        BouncyBall { color: ball_color },
        TrailSpawner { since_last: 0.0 },
    ));
}

pub fn enforce_min_speed(mut query: Query<(&mut Velocity, &InitialSpeed)>) {
    for (mut vel, init) in query.iter_mut() {
        let speed = vel.linvel.length();
        let min_speed = init.0 * MIN_SPEED_FACTOR;
        if speed < min_speed && speed > 0.0001 { vel.linvel = vel.linvel.normalize() * min_speed; }
    }
}
