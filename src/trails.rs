use bevy::prelude::*;

use crate::components::{TrailPoint, TrailSpawner, BouncyBall};

pub fn spawn_trails(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q: Query<(&Transform, &BouncyBall, &mut TrailSpawner)>,
) {
    let interval = 0.02;
    for (transform, ball, mut spawner) in q.iter_mut() {
        spawner.since_last += time.delta_secs();
        if spawner.since_last < interval { continue; }
        spawner.since_last = 0.0;
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(3.5))),
            MeshMaterial2d(materials.add(ColorMaterial::from(ball.color.with_alpha(0.85)))),
            Transform::from_xyz(transform.translation.x, transform.translation.y, 0.5),
            TrailPoint { age: 0.0, max_age: 1.6 },
        ));
    }
}

pub fn update_trails(
    time: Res<Time>,
    mut q: Query<(&mut TrailPoint, &mut Transform, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut trail, mut transform, material_handle) in q.iter_mut() {
        trail.age += time.delta_secs();
        let t = (trail.age / trail.max_age).clamp(0.0, 1.0);
        let alpha = (1.0 - t).powf(1.8);
        let scale = 1.0 - 0.75 * t;
        transform.scale = Vec3::splat(scale);
        if let Some(mat) = materials.get_mut(&material_handle.0) {
            let c = mat.color.to_srgba();
            mat.color = Color::srgba(c.red, c.green, c.blue, alpha as f32);
        }
    }
}

pub fn cleanup_old_trails(mut commands: Commands, q: Query<(Entity, &TrailPoint)>) {
    for (e, trail) in q.iter() { if trail.age >= trail.max_age { commands.entity(e).despawn(); } }
}
