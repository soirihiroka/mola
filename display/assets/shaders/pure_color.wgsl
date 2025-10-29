#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings::view,
    mesh_bindings::world_from_local,
    utils::coords_to_viewport_uv,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif


struct PureColorMaterialUniform {
    color: vec4<f32>,
};

@group(2) @binding(103)
var<uniform> material: PureColorMaterialUniform;

@fragment
fn fragment() ->FragmentOutput {
return FragmentOutput(material.color);
}
