use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
// use bevy::render::texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::render::primitives::Aabb;

use crate::material::pure_color_material::PureColorMaterial;
pub struct MoebiusMaterialPlugin;

impl Plugin for MoebiusMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, MoebiusMaterial>>::default(),
            MaterialPlugin::<PureColorMaterial>::default(),
        ));
        // MaterialPlugin::<PureColorMaterial>::default();
        app.register_type::<ForceMoebiusMaterial>();
        app.add_systems(PreStartup, load_moebius_material_assets);
        app.add_observer(on_force_moebius_material_spawn);
        // app.add_systems(PostStartup, tag_moebius_entities);
        app.add_observer(on_standard_material_added);
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct MoebiusMaterialAssets {
    pub shadow_texture: Handle<Image>,
}

fn load_moebius_material_assets(asset_server: ResMut<AssetServer>, mut commands: Commands) {
    commands.insert_resource(MoebiusMaterialAssets {
        shadow_texture: asset_server.load_with_settings("shadows.png", |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        }),
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MoebiusMaterial {
    #[texture(100)]
    #[sampler(101)]
    pub shadow_texture: Handle<Image>,
    #[uniform(102)]
    pub model_size: Vec3,
}

impl MaterialExtension for MoebiusMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/moebius_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/moebius_material.wgsl".into()
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct ForceMoebiusMaterial;

fn on_force_moebius_material_spawn(
    trigger: Trigger<OnInsert, ForceMoebiusMaterial>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, Option<&Aabb>), With<ForceMoebiusMaterial>>,
    mut mobius_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MoebiusMaterial>>>,
    assets: Res<MoebiusMaterialAssets>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    info!("on_force_moebius_material_spawn");

    let Ok((material_handle, aabb)) = query.get(trigger.target()) else {
        return;
    };

    let Some(material) = pbr_materials.get(material_handle) else {
        info!("Material handle not resolved yet");
        return;
    };

    let model_size = aabb.map(|a| a.half_extents.into()).unwrap_or(Vec3::ONE); // fallback if Aabb isn’t ready

    let custom = mobius_materials.add(ExtendedMaterial {
        base: material.clone(),
        extension: MoebiusMaterial {
            shadow_texture: assets.shadow_texture.clone(),
            model_size,
        },
    });

    commands
        .entity(trigger.target())
        .insert(MeshMaterial3d(custom))
        .remove::<MeshMaterial3d<StandardMaterial>>();

    info!("Material is forced");
}

fn on_standard_material_added(
    trigger: Trigger<OnAdd, MeshMaterial3d<StandardMaterial>>,
    mut commands: Commands,
    query: Query<(Entity, &MeshMaterial3d<StandardMaterial>)>,
    materials: Res<Assets<StandardMaterial>>,
    mut pure_materials: ResMut<Assets<PureColorMaterial>>,
) {
    let Ok((entity, material_component)) = query.get(trigger.target()) else {
        return;
    };

    let handle = &material_component.0;
    if let Some(material) = materials.get(handle) {
        let color = material.base_color;

        if color == Color::WHITE || color == Color::BLACK {
            // Replace with pure color material
            let linear = color.to_linear().to_f32_array();
            let pure = pure_materials.add(PureColorMaterial {
                color: Vec4::from_array(linear),
            });

            commands
                .entity(entity)
                .insert(MeshMaterial3d(pure))
                .remove::<MeshMaterial3d<StandardMaterial>>();

            // if material.base_color == Color::WHITE {
            //     commands.entity(entity).insert(ForceMoebiusMaterial);
            //     info!("Tagged entity {:?} with ForceMoebiusMaterial", entity);
            // } else {
            //     info!("Entity {:?} has a non-white material", entity);
            // }
            info!(
                "Entity {:?} replaced with PureColorMaterial {:?}",
                entity, color
            );
        } else {
            // Otherwise, just mark for MoebiusMaterial replacement
            commands.entity(entity).insert(ForceMoebiusMaterial);
            info!("Entity {:?} marked for MoebiusMaterial", entity);
        }
    } else {
        info!(
            "Entity {:?} has a material handle not yet resolved in Assets",
            entity
        );
    }
}
