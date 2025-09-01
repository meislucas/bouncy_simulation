use bevy::prelude::*;

#[derive(Component)]
pub struct SpinningRing;

#[derive(Component)]
pub struct BouncyBall { pub color: Color }

#[derive(Component)]
pub struct TrailPoint { pub age: f32, pub max_age: f32 }

#[derive(Component)]
pub struct TrailSpawner { pub since_last: f32 }

#[derive(Component)]
pub struct GapRotation { pub angle: f32 }

#[derive(Component)]
pub struct RingCollider;

#[derive(Component)]
pub struct InitialSpeed(pub f32);

#[derive(Component)]
pub struct RingPhysics;

#[derive(Component)]
pub struct BallCounterText;
