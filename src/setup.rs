use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ring::create_circular_ring;
use crate::balls::{spawn_ball, SpawnOrigin};
use crate::ui::spawn_ui;
use crate::components::{RingPhysics, BouncyBall};

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    create_circular_ring(&mut commands, &mut meshes, &mut materials);
    commands.spawn((RigidBody::Fixed, Transform::default(), GlobalTransform::default(), RingPhysics));
    spawn_ball(&mut commands, &mut meshes, &mut materials, SpawnOrigin::RandomDisk { max_radius: 100.0 });
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    spawn_ui(&mut commands, font);
}

pub fn check_ball_escape(
    mut commands: Commands,
    ball_query: Query<(Entity, &Transform), With<BouncyBall>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.iter().next().expect("No window found");
    let (w, h) = (window.width(), window.height());
    let (left, right, bottom, top) = (-w/2.0, w/2.0, -h/2.0, h/2.0);
    let buffer = 20.0;
    for (entity, transform) in ball_query.iter() {
        let p = transform.translation.truncate();
        if p.x < left - buffer || p.x > right + buffer || p.y < bottom - buffer || p.y > top + buffer {
            commands.entity(entity).despawn();
            spawn_ball(&mut commands, &mut meshes, &mut materials, SpawnOrigin::NearCenter { max_offset: 20.0 });
            spawn_ball(&mut commands, &mut meshes, &mut materials, SpawnOrigin::NearCenter { max_offset: 20.0 });
        }
    }
}

pub fn handle_reset_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    ball_query: Query<Entity, With<BouncyBall>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for e in ball_query.iter() { commands.entity(e).despawn(); }
    spawn_ball(&mut commands, &mut meshes, &mut materials, SpawnOrigin::RandomDisk { max_radius: 100.0 });
    }
}
