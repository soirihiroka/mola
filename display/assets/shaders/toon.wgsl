#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#import bevy_pbr::prepass_utils

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

// The uniform struct now includes outline properties.
// `color_steps` is a float for easier math.
struct ToonMaterial {
    quantize_steps: f32,
    outline_color: vec4<f32>,
    outline_width: f32,
};

@group(2) @binding(100)
var<uniform> toon_material: ToonMaterial;

@fragment
fn fragment(
    @builtin(sample_index) sample_index: u32,
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Example tweak
    pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);

#else
    var out: FragmentOutput;

    // Toon outline pass
    if !is_front {
        out.color = toon_material.outline_color;

        // Optional discard if outline width is 0
        if toon_material.outline_width <= 0.0 {
            discard;
        }

        return out;
    }

    // Apply PBR lighting
    out.color = apply_pbr_lighting(pbr_input);

    // Quantize the color (toon shading)
    out.color = vec4<f32>(vec4<u32>(out.color * (toon_material.quantize_steps))) / (toon_material.quantize_steps);

    // Post-processing
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // Optional final tweak
    out.color *= 2.0;

    let depth = bevy_pbr::prepass_utils::prepass_depth(in.position, sample_index);


#endif

    return out;
}
