use crate::api::pose_api::CurrentPose;
use crate::character_control::pose::*;
use crate::ui::state::GuiState;
use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
use paste::paste;
use std::f32::consts::PI;

#[hot(hot_patch_signature = true)]
pub fn rotate_body(
    parts: Res<super::character_controller::CharacterParts>,
    mut mut_transform_q: Query<&mut Transform>,
    g_trans_q: Query<&GlobalTransform>,
    child_of_q: Query<&ChildOf>,
    gui_state: Res<GuiState>,
    current_pose: Res<CurrentPose>,
) -> Result {
    let pose = match current_pose.0.as_ref() {
        Some(p) => p,
        None => return Ok(()),
    };
    let landmarks = &pose.get().world_landmarks;
    let root = parts.root;
    let neck = parts.neck;

    let left_upper_arm = parts.left_arm.upper;
    let right_upper_arm = parts.right_arm.upper;

    let left_lower_arm = parts.left_arm.lower;
    let right_lower_arm = parts.right_arm.lower;

    let left_lower_arm_r = parts.left_arm.lower_r;
    let right_lower_arm_r = parts.right_arm.lower_r;

    let left_upper_leg = parts.left_leg.upper;
    let left_lower_leg = parts.left_leg.lower;

    let right_upper_leg = parts.right_leg.upper;
    let right_lower_leg = parts.right_leg.lower;

    let mut rotate_g = |entity: &Option<Entity>, rotation: Quat, name: &str| -> Result {
        let entity = entity.ok_or(format!("No unable to find {}", name))?;
        let mut transform = mut_transform_q.get_mut(entity)?;
        let parent = child_of_q.get(entity)?.parent();
        let parent_r = g_trans_q.get(parent)?.rotation();
        transform.rotation = parent_r.inverse() * rotation;
        Ok(())
    };

    // Macro definition using the `paste` crate
    macro_rules! rotate_part {
        ($part:ident, $default_rotation:expr) => {
            // `paste!` concatenates identifiers at compile time.
            // [< ... >] is the syntax used by `paste` to merge items.
            paste! {
                {
                    let rotation = if gui_state.[<rotate_ $part>] {
                        // Creates the function call, e.g., `compute_left_upper_armrust-analyzer-diagnostics-view:/diagnostic%20message%20[3]?3#file:///mnt/d/repos/mocap-render/src/character_control/character_controller.rs_rotation(landmarks)`
                        [<compute_ $part _rotation>](landmarks)
                    } else {
                        $default_rotation
                    };
                    // `stringify!` converts the identifier to a string literal, e.g., "left_upper_arm"
                    rotate_g(&$part, rotation, stringify!($part))?;
                }
            }
        };
    }

    rotate_part!(root, Quat::from_rotation_y(PI));

    rotate_part!(neck, Quat::IDENTITY * Quat::from_rotation_y(PI / 2.));

    rotate_part!(
        left_upper_arm,
        Quat::from_rotation_y(PI / 2.) * Quat::from_rotation_x(-PI / 2.)
    );
    rotate_part!(
        right_upper_arm,
        Quat::from_rotation_y(-PI / 2.) * Quat::from_rotation_x(-PI / 2.)
    );

    rotate_part!(
        left_lower_arm,
        Quat::from_rotation_y(PI / 2.) * Quat::from_rotation_x(-PI / 2.)
    );
    rotate_part!(
        right_lower_arm,
        Quat::from_rotation_y(-PI / 2.) * Quat::from_rotation_x(-PI / 2.)
    );

    rotate_part!(
        left_upper_leg,
        Quat::from_rotation_y(PI / 2.) * Quat::from_rotation_x(PI)
    );

    rotate_part!(
        left_lower_leg,
        Quat::from_rotation_y(PI / 2.) * Quat::from_rotation_x(PI)
    );

    rotate_part!(
        right_upper_leg,
        Quat::from_rotation_y(-PI / 2.) * Quat::from_rotation_x(PI)
    );

    rotate_part!(
        right_lower_leg,
        Quat::from_rotation_y(-PI / 2.) * Quat::from_rotation_x(PI)
    );

    rotate_part!(left_lower_arm_r, Quat::from_rotation_y(PI / 2.));

    rotate_part!(right_lower_arm_r, Quat::from_rotation_y(-PI / 2.));

    Ok(())
}
