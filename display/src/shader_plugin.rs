use crate::material::moebius_material::{MoebiusMaterial, MoebiusMaterialAssets};
use bevy::pbr::OpaqueRendererMethod;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::{
    app::Plugin,
    asset::Assets,
    ecs::{
        component::Component,
        hierarchy::Children,
        observer::Trigger,
        system::{Commands, Query, Res, ResMut},
    },
    pbr::{ExtendedMaterial, MaterialPlugin, MeshMaterial3d, StandardMaterial},
    scene::SceneInstanceReady,
};
use bevy_simple_subsecond_system::hot;
pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, MoebiusMaterial>,
        >::default())
            .add_observer(change_material);
    }
}

#[derive(Component)]
pub struct MaterialOverride;

// #[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
// struct ToonMaterialExtension {
//     // We need to ensure that the bindings of the base material and the extension do not conflict,
//     // so we start from binding slot 100, leaving slots 0-99 for the base material.
//     #[uniform(100)]
//     data: ToonMaterialUniformData,
// }

// // 1. Create a new struct that derives `ShaderType`.
// // This struct's memory layout will match the `ToonMaterial` struct in WGSL.
// #[derive(ShaderType, Debug, Clone, Default, Reflect)]
// struct ToonMaterialUniformData {
//     quantize_steps: f32,
//     outline_color: Vec4,
//     outline_width: f32,
//     // The `ShaderType` derive macro automatically handles std140 padding and alignment.
// }

// // Define associated constants in an `impl` block for the struct
// impl ToonMaterialExtension {
//     const SHADER_ASSET_PATH: &'static str = "shaders/toon.wgsl";
// }

// impl MaterialExtension for ToonMaterialExtension {
//     fn fragment_shader() -> ShaderRef {
//         Self::SHADER_ASSET_PATH.into()
//     }
//     fn deferred_fragment_shader() -> ShaderRef {
//         Self::SHADER_ASSET_PATH.into()
//     }
// }

#[hot]
fn change_material(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    material_override: Query<&MaterialOverride>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MoebiusMaterial>>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &Aabb)>,
    assets: Res<MoebiusMaterialAssets>,
) {
    // Get the `MaterialOverride` of the entity, if it does not have a color override, skip
    let Ok(_) = material_override.get(trigger.target()) else {
        return;
    };

    // let Ok((_, aabb)) = query.get(trigger.target()) else {
    //     info!("No aabb");
    //     return;
    // };

    info!("Changing materials!!");

    // Iterate over all children recursively
    for descendants in children.iter_descendants(trigger.target()) {
        // Get the material of the descendant
        if let Some(material) = mesh_materials
            .get(descendants)
            .ok()
            .and_then(|id| asset_materials.get_mut(id.id()))
        {
            commands
                .entity(descendants)
                .insert(MeshMaterial3d(materials.add(ExtendedMaterial {
                    base: StandardMaterial {
                        base_color: material.base_color.into(),
                        // can be used in forward or deferred mode
                        opaque_render_method: OpaqueRendererMethod::Auto,
                        // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                        // in forward mode, the output can also be modified after lighting is applied.
                        // see the fragment shader `extended_material.wgsl` for more info.
                        // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                        // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                        ..Default::default()
                    },
                    extension: MoebiusMaterial {
                        shadow_texture: assets.shadow_texture.clone(),
                        model_size: Vec3::new(5., 5., 5.),
                    },
                })));
        }
    }
}
