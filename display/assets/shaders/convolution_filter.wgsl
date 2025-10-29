fn create_filter_from_scalar(a: f32) -> mat3x3<f32> {
    return mat3x3(
        a, a, a,
        a, a, a,
        a, a, a,
    );
}

//    Example filters
//
//    let identity = create_filter(
//        0.0, 0.0, 0.0,
//        0.0, 1.0, 0.0,
//        0.0, 0.0, 0.0,
//    );
//
//    let sharpen = create_filter(
//        0.0, -1.0, 0.0,
//        -1.0, 5.0, -1.0,
//        0.0, -1.0, 0.0,
//    );
//
//    let mean_blur = create_filter_from_scalar(1.0/9.0);
//
//    let leplecian = create_filter(
//        0.0, 1.0, 0.0,
//        1.0, -4.0, 1.0,
//        0.0, 1.0, 0.0,
//    );
//
//    let gauss = create_filter(
//        1.0/16.0, 2.0/16.0, 1.0/16.0,
//        2.0/16.0, 4.0/16.0, 2.0/16.0,
//        1.0/16.0, 2.0/16.0, 1.0/16.0,
//    );
//
//    let sobel_x = create_filter(
//         1.0,  2.0, 1.0,
//         0.0,  0.0, 0.0,
//        -1.0, -2.0, -1.0,
//    );
//
//    let sobel_y = create_filter(
//        1.0, 0.0, -1.0,
//        2.0, 0.0, -2.0,
//        1.0, 0.0, -1.0,
//    );
fn create_filter(a: f32, b:f32, c:f32, d:f32, e:f32, f:f32, g:f32, h:f32, i:f32) -> mat3x3<f32> {
    return mat3x3(
        a, d, g,
        b, e, h,
        c, f, i,
    );
}

fn apply_filter_on_depth_buffer(id: vec2<f32>, texture: texture_depth_2d, filter_mat: mat3x3<f32>) -> f32 {
    let depth = abs(
          filter_mat[0][0] * textureLoad(texture, vec2<i32>(id + vec2<f32>(-1.0,  1.0)), 0)
        + filter_mat[1][0] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0,  1.0)), 0)
        + filter_mat[2][0] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 1.0,  1.0)), 0)
        + filter_mat[0][1] * textureLoad(texture, vec2<i32>(id + vec2<f32>(-1.0,  0.0)), 0)
        + filter_mat[1][1] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0,  0.0)), 0)
        + filter_mat[2][1] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0, -1.0)), 0)
        + filter_mat[0][2] * textureLoad(texture, vec2<i32>(id + vec2<f32>(-1.0, -1.0)), 0)
        + filter_mat[1][2] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0, -1.0)), 0)
        + filter_mat[2][2] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 1.0, -1.0)), 0)
    );

    return depth;
}

fn apply_filter(id: vec2<f32>, texture: texture_2d<f32>, filter_mat: mat3x3<f32>) -> vec3<f32> {
    let depth = abs(
          filter_mat[0][0] * textureLoad(texture, vec2<i32>(id + vec2<f32>(-1.0,  1.0)), 0).rgb
        + filter_mat[1][0] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0,  1.0)), 0).rgb
        + filter_mat[2][0] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 1.0,  1.0)), 0).rgb
        + filter_mat[0][1] * textureLoad(texture, vec2<i32>(id + vec2<f32>(-1.0,  0.0)), 0).rgb
        + filter_mat[1][1] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0,  0.0)), 0).rgb
        + filter_mat[2][1] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0, -1.0)), 0).rgb
        + filter_mat[0][2] * textureLoad(texture, vec2<i32>(id + vec2<f32>(-1.0, -1.0)), 0).rgb
        + filter_mat[1][2] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 0.0, -1.0)), 0).rgb
        + filter_mat[2][2] * textureLoad(texture, vec2<i32>(id + vec2<f32>( 1.0, -1.0)), 0).rgb
    );

    return depth;
}

fn apply_filter_on_texture_with_sampler(id: vec2<f32>, texture: texture_2d<f32>, texture_sampler: sampler, resolution: vec2<f32>, filter_mat: mat3x3<f32>) -> vec3<f32> {
    let color = abs(
          filter_mat[0][0] * textureSample(texture, texture_sampler, id + (vec2<f32>(-1.0,  1.0) / resolution)).rgb
        + filter_mat[1][0] * textureSample(texture, texture_sampler, id + (vec2<f32>( 0.0,  1.0) / resolution)).rgb
        + filter_mat[2][0] * textureSample(texture, texture_sampler, id + (vec2<f32>( 1.0,  1.0) / resolution)).rgb
        + filter_mat[0][1] * textureSample(texture, texture_sampler, id + (vec2<f32>(-1.0,  0.0) / resolution)).rgb
        + filter_mat[1][1] * textureSample(texture, texture_sampler, id + (vec2<f32>( 0.0,  0.0) / resolution)).rgb
        + filter_mat[2][1] * textureSample(texture, texture_sampler, id + (vec2<f32>( 0.0, -1.0) / resolution)).rgb
        + filter_mat[0][2] * textureSample(texture, texture_sampler, id + (vec2<f32>(-1.0, -1.0) / resolution)).rgb
        + filter_mat[1][2] * textureSample(texture, texture_sampler, id + (vec2<f32>( 0.0, -1.0) / resolution)).rgb
        + filter_mat[2][2] * textureSample(texture, texture_sampler, id + (vec2<f32>( 1.0, -1.0) / resolution)).rgb
    );

    return vec3<f32>(color);
}
