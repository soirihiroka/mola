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

@group(2) @binding(100)
var line_shadow_texture: texture_2d<f32>;
@group(2) @binding(101)
var shadow_texture_sampler: sampler;
@group(2) @binding(102)
var<uniform> model_size: vec3<f32>;

fn grayscale(gamma: vec4<f32>) -> f32 {
    return (gamma.r + gamma.g + gamma.b) / 3.0;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    //pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting

    let scale_factor = 1.0 / length(model_size * 4);

    let light = apply_pbr_lighting(pbr_input);
    let light_power = pbr_input.material.base_color - light;
    let shadow_power = 1.0 - grayscale(light_power);

    let viewport_uv = coords_to_viewport_uv(in.position.xy, view.viewport) * 16;
    let lines = textureSample(line_shadow_texture, shadow_texture_sampler, viewport_uv).rgba;
    //let lines = textureSample(line_shadow_texture, shadow_texture_sampler, vec2f(in.uv / scale_factor)).rgba;

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    let post_processing_light = pbr_input.material.base_color - main_pass_post_lighting_processing(pbr_input, light);
    out.color = pbr_input.material.base_color;

    // Strong Shadow
    if shadow_power < 0.5 && (lines.r == 1.0 || lines.g == 1.0 || lines.b == 1.0) {
        out.color -= vec4(1.00, 1.00, 1.00, 0.0);
        out.color -= post_processing_light * 0.1;
    }

    // Middle Shadow
    else if shadow_power < 0.8 && (lines.r == 1.0 || lines.b == 1.0) {
        out.color -= vec4(1.00, 1.00, 1.00, 0.0);
        out.color -= post_processing_light * 0.2;
    }

    // Light Shadow
    else if shadow_power < 1.1 && (lines.b == 1.0) {
        out.color -= vec4(1.00, 1.00, 1.00, 0.0);
    }


#endif

    return out;
}