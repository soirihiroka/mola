use bevy::asset::Asset;
use bevy::math::Vec4;
use bevy::pbr::Material;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
// #[bind_group(1)]
pub struct PureColorMaterial {
    #[uniform(103)]
    pub color: Vec4,
}

impl Material for PureColorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/pure_color.wgsl".into()
    }
}
