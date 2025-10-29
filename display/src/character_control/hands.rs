use bevy::prelude::*;
use std::f32::consts::PI;

use bevy_simple_subsecond_system::prelude::*;

#[hot]
pub fn compute_mcp_rotation(mcp: &Vec3, pip: &Vec3, dip: &Vec3, right_mcp: &Vec3) -> Quat {
    let forward = (pip - mcp).normalize();
    let right = (right_mcp - mcp).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();
    let rotation_matrix = Mat3::from_cols(right, up, forward);
    Quat::from_mat3(&rotation_matrix).normalize() * Quat::from_rotation_x(PI / 2.)
}

#[hot]
pub fn compute_pip_rotation(mcp: &Vec3, pip: &Vec3, dip: &Vec3, right_mcp: &Vec3) -> Quat {
    let forward = (dip - pip).normalize();
    let right = (right_mcp - mcp).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();
    let rotation_matrix = Mat3::from_cols(right, up, forward);
    Quat::from_mat3(&rotation_matrix).normalize() * Quat::from_rotation_x(PI / 2.)
}

#[hot]
pub fn compute_dip_rotation(mcp: &Vec3, dip: &Vec3, tip: &Vec3, right_mcp: &Vec3) -> Quat {
    let forward = (tip - dip).normalize();
    let right = (right_mcp - mcp).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();
    let rotation_matrix = Mat3::from_cols(right, up, forward);
    Quat::from_mat3(&rotation_matrix) * Quat::from_rotation_x(PI / 2.)
}

#[hot]
pub fn compute_mcp_rotation_thumb(mcp: &Vec3, pip: &Vec3, dip: &Vec3, left_mcp: &Vec3) -> Quat {
    let forward = (dip - pip).normalize();
    let right = (mcp - left_mcp).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();
    let rotation_matrix = Mat3::from_cols(right, up, forward);
    Quat::from_mat3(&rotation_matrix)
}

#[hot]
pub fn compute_dip_rotation_thumb(mcp: &Vec3, ip: &Vec3, tip: &Vec3, left_mcp: &Vec3) -> Quat {
    let forward = (tip - ip).normalize();
    let right = (mcp - left_mcp).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();
    let rotation_matrix = Mat3::from_cols(right, up, forward);
    Quat::from_mat3(&rotation_matrix)
}

#[hot]
pub fn compute_left_palm_rotation(wrist: &Vec3, middle: &Vec3, thumb: &Vec3, pinky: &Vec3) -> Quat {
    let forward = (middle - wrist).normalize();
    let right = (thumb - pinky).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();

    let rotation_matrix = Mat3::from_cols(right, up, forward);
    Quat::from_mat3(&rotation_matrix) * Quat::from_rotation_x(PI / 2.)
}

#[hot]
pub fn compute_right_palm_rotation(
    wrist: &Vec3,
    middle: &Vec3,
    thumb: &Vec3,
    pinky: &Vec3,
) -> Quat {
    let forward = (middle - wrist).normalize();
    let right = (pinky - thumb).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();

    let rotation_matrix = Mat3::from_cols(right, up, forward);
    Quat::from_mat3(&rotation_matrix) * Quat::from_rotation_x(PI / 2.)
}
