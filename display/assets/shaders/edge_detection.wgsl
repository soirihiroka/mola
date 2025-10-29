#import "shaders/convolution_filter.wgsl"::create_filter
#import "shaders/convolution_filter.wgsl"::apply_filter_on_depth_buffer
#import "shaders/convolution_filter.wgsl"::apply_filter

struct DetectEdgeSettings {
    depth_threshold: f32,
    normal_threshold: f32,

    #ifdef SIXTEEN_BYTE_ALIGNMENT
        // WebGL2 structs must be 16 byte aligned.
        _webgl2_padding: vec3<f32>
    #endif
}

fn detect_edge(id: vec2<f32>, deep_buffer: texture_depth_2d, normal_map: texture_2d<f32>, settings: DetectEdgeSettings) -> f32 {
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

    var depth = 0.0;
    var normal = 0.0;

    if settings.depth_threshold > 0.0 {
        let depth_x = vec3f(apply_filter_on_depth_buffer(id, deep_buffer, sobel_x));
        let depth_y = vec3f(apply_filter_on_depth_buffer(id, deep_buffer, sobel_y));

        depth = sqrt(dot(depth_x, depth_x) + dot(depth_y, depth_y));
    }

    if depth < settings.depth_threshold {
        depth = 0.0;
    }

    if settings.normal_threshold > 0.0 {
        let normal_x = apply_filter(id, normal_map, sobel_x);
        let normal_y = apply_filter(id, normal_map, sobel_y);

        normal = sqrt(dot(normal_x, normal_x) + dot(normal_y, normal_y));
    }

    if normal < settings.normal_threshold {
        normal = 0.0;
    }

    return max(depth, normal);
}