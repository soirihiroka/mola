use bevy::prelude::*;

use crate::shader_plugin::MaterialOverride;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_character);
    }
}

fn add_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/model1.glb#Scene0"));
    commands.spawn((SceneRoot(model), MaterialOverride));
}
