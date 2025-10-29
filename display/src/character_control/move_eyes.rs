use std::f32::consts::PI;

use crate::api::face_api::CurrentFace;
use crate::api::hands_api::CurrentHands;
use crate::api::hands_api::HandLandmarkIndex;
use crate::character_control::character_controller::CharacterParts;
use crate::character_control::hands::*;
use crate::ui::state::GuiState;
use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

#[hot(hot_patch_signature = true)]
pub fn move_eyes(
    parts: Res<CharacterParts>,
    mut mut_transform_q: Query<&mut Transform>,
    gui_state: Res<GuiState>,
    g_trans_q: Query<&GlobalTransform>,
    child_of_q: Query<&ChildOf>,
    curr_face: Res<CurrentFace>,
) -> Result {
    let face = match curr_face.expression.as_ref() {
        Some(p) => p,
        None => return Ok(()),
    };

    let left_eye = parts.left_eye.ok_or("No Left Eye")?;

    let right_eye = parts.right_eye.ok_or("No Right Eye")?;

    // print!("Moving eyes to ({}, {})\n", face.look_x, face.look_y);

    {
        let mut t_left_eye = mut_transform_q.get_mut(left_eye)?;
        t_left_eye.translation = Vec3::from((
            0.2,
            face.look_y * 0.1 * gui_state.move_eyes_scale + 0.31,
            -face.look_x * 0.1 * gui_state.move_eyes_scale - 0.15,
        ))
    }

    {
        let mut t_right_eye = mut_transform_q.get_mut(right_eye)?;
        t_right_eye.translation = Vec3::from((
            0.2,
            face.look_y * 0.1 * gui_state.move_eyes_scale + 0.31,
            -face.look_x * 0.1 * gui_state.move_eyes_scale + 0.15,
        ))
    }

    // {
    //     let mut t_right_eye = mut_transform_q.get_mut(right_eye)?;
    //     t_right_eye.translation = Vec3::from((
    //         face.look_x * gui_state.move_eyes_scale,
    //         -face.look_y * gui_state.move_eyes_scale,
    //         0.0,
    //     ))
    // }
    Ok(())
}
