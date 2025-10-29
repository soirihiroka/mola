mod api;
mod camera_controller;
mod gizmos_plugin;
mod ui;
use std::{env, f32::consts::PI, path::PathBuf};

use api::api_server::MocapApiPlugin;
mod character_control;
mod math;
mod model_plugin;
use crate::{
    material::moebius_material::MoebiusMaterialPlugin,
    material::post_processing_moebius::MoebiusPostProcessPlugin,
    model_plugin::ModelPlugin,
    // shader_plugin::ShaderPlugin,
};
use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    ecs::system::Commands,
    pbr::PointLight,
    prelude::*,
    scene::SceneInstanceReady,
    transform::components::Transform,
    utils::default,
};
use bevy_simple_subsecond_system::prelude::*;
use camera_controller::CameraControllerPlugin;
use character_control::character_controller::CharacterControllerPlugin;
use character_control::mouth::MouthControlPlugin;
use gizmos_plugin::GizmosPlugin;
use ui::ui_controller::GuiControllerPlugin;
mod material;
// mod post_processing_plugin;
mod shader_plugin;
// mod post_processing_moebius;
// mod moebius_material;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
fn main() {
    if cfg!(debug_assertions) {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        env::set_current_dir(PathBuf::from(manifest_dir)).unwrap();
        println!("CARGO_MANIFEST_DIR: {manifest_dir}");
    }

    App::new()
        .add_plugins((
            DefaultPlugins,
            // WorldInspectorPlugin::new(),
            SimpleSubsecondPlugin::default(),
            ModelPlugin,
            MocapApiPlugin,
            GuiControllerPlugin,
            CameraControllerPlugin,
            CharacterControllerPlugin,
            MoebiusMaterialPlugin,
            MoebiusPostProcessPlugin,
            GizmosPlugin,
            MouthControlPlugin,
        ))
        .add_event::<SceneInstanceReady>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_lights)
        // .add_observer(observer)
        .run();
}
#[derive(Component)]
struct NeckTag;

#[derive(Component)]
struct MovedScene;

// #[hot]
fn setup(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            // intensity: 9000.,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(10.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-PI * 3. / 4.),
            ..default()
        },
    ));

    commands.insert_resource(AmbientLight {
        // color: ORANGE_RED.into(),
        brightness: 2000.0,
        ..default()
    });
}

/// This system queries for all PointLight components and updates their properties each frame.
#[hot]
fn update_lights(
    mut query: Query<&mut PointLight>,
    mut ambient_light: ResMut<AmbientLight>,
    mut directional_light: Query<&mut DirectionalLight>,
    // time: Res<Time>, // The Time resource provides timing information
) {
    ambient_light.brightness = 1500.;
    // Iterate over each mutable PointLight component found in the scene
    for mut light in query.iter_mut() {
        // Use a sine wave based on the elapsed time to create a pulsating effect.
        // The intensity will oscillate between 50,000 and 150,000.
        light.intensity = 5000.
        // light.color = Color::hsl(hue, 1.0, 0.5);
    }

    for mut light in directional_light.iter_mut() {
        light.illuminance = 5000.;
        // light.direction =
    }
}
