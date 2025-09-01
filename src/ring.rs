use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy_rapier2d::prelude::*;

use crate::components::{SpinningRing, GapRotation, RingPhysics};
use crate::constants::{CIRCLE_RADIUS, RING_THICKNESS, GAP_ANGLE};

pub fn shortest_angle(a: f32, b: f32) -> f32 { let mut d = a - b; while d > std::f32::consts::PI { d -= 2.0 * std::f32::consts::PI; } while d < -std::f32::consts::PI { d += 2.0 * std::f32::consts::PI; } d }

pub fn rebuild_ring_collider(
    gap_query: Query<&GapRotation>,
    mut commands: Commands,
    ring_query: Query<(Entity, &RingPhysics)>,
) {
    let Ok(gap) = gap_query.single() else { return; };
    let Ok((entity, _)) = ring_query.single() else { return; };

    let inner_radius = CIRCLE_RADIUS - RING_THICKNESS;
    let outer_radius = CIRCLE_RADIUS;
    let mid_radius = (inner_radius + outer_radius) * 0.5;
    let half_thickness = (outer_radius - inner_radius) * 0.5;
    let segments = 120;
    let segment_angle = 2.0 * std::f32::consts::PI / segments as f32;
    const GAP_BUFFER: f32 = 0.035;
    let gap_half_width = GAP_ANGLE * 0.5 + GAP_BUFFER;
    let gap_center = gap.angle;
    let mut parts: Vec<(Vec2, f32, Collider)> = Vec::new();

    for i in 0..segments {
        let a0 = i as f32 * segment_angle;
        let a1 = (i as f32 + 1.0) * segment_angle;
        let d0 = shortest_angle(a0, gap_center).abs();
        let d1 = shortest_angle(a1, gap_center).abs();
        if d0 <= gap_half_width || d1 <= gap_half_width { continue; }
        let p0 = Vec2::new(mid_radius * a0.cos(), mid_radius * a0.sin());
        let p1 = Vec2::new(mid_radius * a1.cos(), mid_radius * a1.sin());
        let seg = p1 - p0; let len = seg.length(); if len <= 0.0001 { continue; }
        let center = (p0 + p1) * 0.5; let angle = seg.y.atan2(seg.x); let half_height = len * 0.5;
        parts.push((center, angle, Collider::capsule_y(half_height, half_thickness)));
    }

    // gap edge guides
    let gap_edge1 = gap_center - GAP_ANGLE * 0.5;
    let gap_edge2 = gap_center + GAP_ANGLE * 0.5;
    let edge_thickness = 1.5; let guide_length = RING_THICKNESS * 0.8;
    let edge1_inner = Vec2::new(inner_radius * gap_edge1.cos(), inner_radius * gap_edge1.sin());
    let edge1_outer = Vec2::new((inner_radius + guide_length) * gap_edge1.cos(), (inner_radius + guide_length) * gap_edge1.sin());
    let edge1_center = (edge1_inner + edge1_outer) * 0.5; let edge1_angle = gap_edge1;
    parts.push((edge1_center, edge1_angle, Collider::capsule_y(guide_length * 0.5, edge_thickness * 0.5)));
    let edge2_inner = Vec2::new(inner_radius * gap_edge2.cos(), inner_radius * gap_edge2.sin());
    let edge2_outer = Vec2::new((inner_radius + guide_length) * gap_edge2.cos(), (inner_radius + guide_length) * gap_edge2.sin());
    let edge2_center = (edge2_inner + edge2_outer) * 0.5; let edge2_angle = gap_edge2;
    parts.push((edge2_center, edge2_angle, Collider::capsule_y(guide_length * 0.5, edge_thickness * 0.5)));

    commands.entity(entity).insert(Collider::compound(parts));
}

pub fn create_circular_ring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let inner_radius = CIRCLE_RADIUS - 20.0;
    let outer_radius = CIRCLE_RADIUS;
    let segments = 200;
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
        let gap_start = -GAP_ANGLE / 2.0;
        let gap_end = GAP_ANGLE / 2.0;
        let normalized_angle = if angle > std::f32::consts::PI { angle - 2.0 * std::f32::consts::PI } else { angle };
        if normalized_angle >= gap_start && normalized_angle <= gap_end { continue; }
        let cos_a = angle.cos(); let sin_a = angle.sin();
        positions.push([inner_radius * cos_a, inner_radius * sin_a, 0.0]);
        positions.push([outer_radius * cos_a, outer_radius * sin_a, 0.0]);
    }
    for i in 0..(positions.len() / 2 - 1) {
        let base = i * 2;
        indices.push(base as u32);
        indices.push((base + 1) as u32);
        indices.push((base + 3) as u32);
        indices.push(base as u32);
        indices.push((base + 3) as u32);
        indices.push((base + 2) as u32);
    }
    let mut ring_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    ring_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    ring_mesh.insert_indices(Indices::U32(indices));

    commands.spawn((
        Mesh2d(meshes.add(ring_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.9, 0.9)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpinningRing,
        GapRotation { angle: 0.0 },
    ));
}

pub fn rotate_gap(
    time: Res<Time>,
    mut gap_query: Query<&mut GapRotation>,
    mut ring_visual: Query<&mut Transform, With<SpinningRing>>,
) {
    if let Ok(mut gap_rotation) = gap_query.single_mut() {
        let rotation_speed = 0.5;
        let delta_rotation = time.delta_secs() * rotation_speed;
        gap_rotation.angle += delta_rotation;
        let pi = std::f32::consts::PI;
        if gap_rotation.angle > pi { gap_rotation.angle = ((gap_rotation.angle + pi) % (2.0 * pi)) - pi; }
        else if gap_rotation.angle < -pi { gap_rotation.angle = ((gap_rotation.angle - pi) % (2.0 * pi)) + pi; }
        if let Ok(mut t) = ring_visual.single_mut() { t.rotation *= Quat::from_rotation_z(delta_rotation); }
    }
}
