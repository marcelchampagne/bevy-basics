use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Component, Debug)]
pub struct MainLight;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(0.0, 20.0, 12.5)
                .looking_at(Vec3::new(0.0, 0.0, 12.5), -Vec3::Y),
            ..Default::default()
        },
        RenderLayers::all(),
    ));
}
