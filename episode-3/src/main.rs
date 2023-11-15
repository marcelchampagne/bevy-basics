mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use bevy::prelude::*;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
