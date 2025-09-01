use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::{BouncyBall, InitialSpeed, TrailSpawner};
use crate::constants::{BALL_RADIUS, MIN_SPEED_FACTOR};

/// How to choose initial position for a new ball
pub enum SpawnOrigin {
    RandomDisk { max_radius: f32 },
    NearCenter { max_offset: f32 },
}

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

pub fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    origin: SpawnOrigin,
) {
    let mut rng = rand::rng();
    let (x, y) = match origin {
        SpawnOrigin::RandomDisk { max_radius } => {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let r = rng.random_range(0.0..max_radius);
            (r * angle.cos(), r * angle.sin())
        }
        SpawnOrigin::NearCenter { max_offset } => {
            (rng.random_range(-max_offset..max_offset), rng.random_range(-max_offset..max_offset))
        }
    };
    let vx = rng.random_range(-400.0..400.0);
    let vy = rng.random_range(-400.0..400.0);
    let color = get_random_neon_color();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from(color))),
        Transform::from_xyz(x, y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity { linvel: Vec2::new(vx, vy), angvel: 0.0 },
        Restitution { coefficient: 1.0, combine_rule: CoefficientCombineRule::Max },
        Friction::coefficient(0.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        GravityScale(0.0),
        Ccd::enabled(),
        Sleeping::disabled(),
        InitialSpeed(Vec2::new(vx, vy).length()),
        BouncyBall { color },
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
