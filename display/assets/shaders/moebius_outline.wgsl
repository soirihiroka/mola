// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import "shaders/convolution_filter.wgsl"::{create_filter, apply_filter_on_depth_buffer, apply_filter }

struct MoebiusOutlineSettings {
    inline_color: vec4<f32>,
    inline_threshold: f32,
    @align(16)
    outline_color: vec4<f32>,
    outline_threshold: f32,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: MoebiusOutlineSettings;
@group(0) @binding(3) var depth_prepass_texture: texture_depth_2d;
@group(0) @binding(4) var normal_prepass_texture: texture_2d<f32>;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let sobel_x = create_filter(
         1.0,  2.0, 1.0,
         0.0,  0.0, 0.0,
        -1.0, -2.0, -1.0,
    );

    let sobel_y = create_filter(
        1.0, 0.0, -1.0,
        2.0, 0.0, -2.0,
        1.0, 0.0, -1.0,
    );

    // render outline
    if settings.outline_threshold > 0.0 {
        let depth_x = vec3f(apply_filter_on_depth_buffer(in.position.xy, depth_prepass_texture, sobel_x));
        let depth_y = vec3f(apply_filter_on_depth_buffer(in.position.xy, depth_prepass_texture, sobel_y));

        let edge = sqrt(dot(depth_x, depth_x) + dot(depth_y, depth_y));

        if edge >= settings.outline_threshold {
            return settings.outline_color;
        }
    }

    // render inline
    if settings.inline_threshold > 0.0 {
        let normal_x = apply_filter(in.position.xy, normal_prepass_texture, sobel_x);
        let normal_y = apply_filter(in.position.xy, normal_prepass_texture, sobel_y);

        let edge = sqrt(dot(normal_x, normal_x) + dot(normal_y, normal_y));

        // IDK WTF is going on with wgpu or rust
        if edge >= 1.3 {
            return settings.inline_color;
        }
    }

    // render orginal color
    return textureSample(screen_texture, texture_sampler, in.uv).rgba;
}