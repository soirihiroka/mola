#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct SketchJitterSettings {
    strength: f32,
    speed: f32,
};

@group(0) @binding(0) var screen_tex: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: SketchJitterSettings;

fn hash(p: vec2<f32>) -> f32 {
    // simple hash noise
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;

    // animate over time using settings.speed
    let time = settings.speed;
    let noise_x = hash(uv * 100.0 + vec2<f32>(time, 0.0));
    let noise_y = hash(uv * 100.0 + vec2<f32>(0.0, time));

    let jitter = (vec2<f32>(noise_x, noise_y) - 0.5) * settings.strength;
    uv = uv + jitter;

    return textureSample(screen_tex, screen_sampler, uv);
}
