mod constants;
mod components;
mod ring;
mod balls;
mod trails;
mod ui;
mod setup;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use ring::{rotate_gap, rebuild_ring_collider};
use trails::{spawn_trails, update_trails, cleanup_old_trails};
use setup::{setup_system, check_ball_escape, handle_reset_input};
use ui::update_ball_counter;
use balls::enforce_min_speed;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default()) //uncomment this line to see colliders 
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                rotate_gap,
                rebuild_ring_collider,
                spawn_trails,
                update_trails,
                cleanup_old_trails,
                check_ball_escape,
                enforce_min_speed,
                handle_reset_input,
                update_ball_counter,
            ),
        )
        .run();
}
