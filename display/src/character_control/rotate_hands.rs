use std::f32::consts::PI;

use crate::api::hands_api::CurrentHands;
use crate::api::hands_api::HandLandmarkIndex;
use crate::character_control::character_controller::CharacterParts;
use crate::character_control::hands::*;
use crate::ui::state::GuiState;
use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

#[hot(hot_patch_signature = true)]
pub fn rotate_hands(
    parts: Res<CharacterParts>,
    mut mut_transform_q: Query<&mut Transform>,
    g_trans_q: Query<&GlobalTransform>,
    child_of_q: Query<&ChildOf>,
    gui_state: Res<GuiState>,
    current_hands: Res<CurrentHands>,
) -> Result {
    let left_hand_parts = &parts.left_hand;
    let right_hand_parts = &parts.right_hand;

    let rotate_g = |entity: &Option<Entity>,
                    rotation: Quat,
                    name: &str,
                    mut_transform_q: &mut Query<&mut Transform>|
     -> Result {
        let entity = entity.ok_or(format!("No unable to find {}", name))?;
        let mut transform = mut_transform_q.get_mut(entity)?;
        let parent = child_of_q.get(entity)?.parent();
        let parent_r = g_trans_q.get(parent)?.rotation();
        transform.rotation = parent_r.inverse() * rotation;
        Ok(())
    };

    use HandLandmarkIndex::*;

    // info!("Rotating hands");
    // info!("current_hands: {:?}", current_hands);

    let left_hand = match current_hands.left_hand.as_ref() {
        Some(p) => {
            // info!("Left hand landmarks: {:?}", p.get().landmarks);
            p},
        None => return Ok(()),
    };

    // info!("Rotating left hand");

    {
        let mcp: Option<Entity> = left_hand_parts.thumb.mcp;
        let mcp_r = compute_mcp_rotation_thumb(
            &left_hand.get().landmarks[ThumbMcp].position,
            &left_hand.get().landmarks[ThumbMcp].position,
            &left_hand.get().landmarks[ThumbIp].position,
            &left_hand.get().landmarks[IndexFingerMcp].position,
        );
        rotate_g(&mcp, mcp_r, "thumb_pip", &mut mut_transform_q)?;
    }

    {
        let ip = left_hand_parts.thumb.ip;
        let ip_r = compute_dip_rotation_thumb(
            &left_hand.get().landmarks[ThumbMcp].position,
            &left_hand.get().landmarks[ThumbIp].position,
            &left_hand.get().landmarks[ThumbTip].position,
            &left_hand.get().landmarks[IndexFingerMcp].position,
        );
        rotate_g(&ip, ip_r, "thumb_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = left_hand_parts.index.mcp;
        let mcp_r = if gui_state.rotate_index_cmp {
            compute_mcp_rotation(
                &left_hand.get().landmarks[IndexFingerMcp].position,
                &left_hand.get().landmarks[IndexFingerPip].position,
                &left_hand.get().landmarks[IndexFingerDip].position,
                &left_hand.get().landmarks[ThumbMcp].position,
            )
        } else {
            Quat::IDENTITY
        };
        rotate_g(&mcp, mcp_r, "index_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = left_hand_parts.index.pip;
        let pip_r = compute_pip_rotation(
            &left_hand.get().landmarks[IndexFingerMcp].position,
            &left_hand.get().landmarks[IndexFingerPip].position,
            &left_hand.get().landmarks[IndexFingerDip].position,
            &left_hand.get().landmarks[ThumbMcp].position,
        );
        rotate_g(&pip, pip_r, "index_pip", &mut mut_transform_q)?;
    }

    {
        let dip = left_hand_parts.index.dip;
        let dip_r = compute_dip_rotation(
            &left_hand.get().landmarks[IndexFingerMcp].position,
            &left_hand.get().landmarks[IndexFingerDip].position,
            &left_hand.get().landmarks[IndexFingerTip].position,
            &left_hand.get().landmarks[ThumbMcp].position,
        );
        rotate_g(&dip, dip_r, "index_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = left_hand_parts.middle.mcp;
        let mcp_r = compute_mcp_rotation(
            &left_hand.get().landmarks[MiddleFingerMcp].position,
            &left_hand.get().landmarks[MiddleFingerPip].position,
            &left_hand.get().landmarks[MiddleFingerDip].position,
            &left_hand.get().landmarks[IndexFingerMcp].position,
        );
        rotate_g(&mcp, mcp_r, "middle_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = left_hand_parts.middle.pip;
        let pip_r = compute_pip_rotation(
            &left_hand.get().landmarks[MiddleFingerMcp].position,
            &left_hand.get().landmarks[MiddleFingerPip].position,
            &left_hand.get().landmarks[MiddleFingerDip].position,
            &left_hand.get().landmarks[IndexFingerMcp].position,
        );
        rotate_g(&pip, pip_r, "middle_pip", &mut mut_transform_q)?;
    }

    {
        let dip = left_hand_parts.middle.dip;

        let dip_r = compute_dip_rotation(
            &left_hand.get().landmarks[MiddleFingerMcp].position,
            &left_hand.get().landmarks[MiddleFingerDip].position,
            &left_hand.get().landmarks[MiddleFingerTip].position,
            &left_hand.get().landmarks[IndexFingerMcp].position,
        );
        rotate_g(&dip, dip_r, "middle_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = left_hand_parts.ring.mcp;
        let mcp_r = compute_mcp_rotation(
            &left_hand.get().landmarks[RingFingerMcp].position,
            &left_hand.get().landmarks[RingFingerPip].position,
            &left_hand.get().landmarks[RingFingerDip].position,
            &left_hand.get().landmarks[MiddleFingerMcp].position,
        );
        rotate_g(&mcp, mcp_r, "ring_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = left_hand_parts.ring.pip;
        let pip_r = compute_pip_rotation(
            &left_hand.get().landmarks[RingFingerMcp].position,
            &left_hand.get().landmarks[RingFingerPip].position,
            &left_hand.get().landmarks[RingFingerDip].position,
            &left_hand.get().landmarks[MiddleFingerMcp].position,
        );
        rotate_g(&pip, pip_r, "ring_pip", &mut mut_transform_q)?;
    }

    {
        let dip = left_hand_parts.ring.dip;
        let dip_r = compute_dip_rotation(
            &left_hand.get().landmarks[RingFingerMcp].position,
            &left_hand.get().landmarks[RingFingerDip].position,
            &left_hand.get().landmarks[RingFingerTip].position,
            &left_hand.get().landmarks[MiddleFingerMcp].position,
        );
        rotate_g(&dip, dip_r, "ring_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = left_hand_parts.pinky.mcp;
        let mcp_r = compute_mcp_rotation(
            &left_hand.get().landmarks[PinkyMcp].position,
            &left_hand.get().landmarks[PinkyPip].position,
            &left_hand.get().landmarks[PinkyDip].position,
            &left_hand.get().landmarks[RingFingerMcp].position,
        );
        rotate_g(&mcp, mcp_r, "pinky_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = left_hand_parts.pinky.pip;
        let pip_r = compute_pip_rotation(
            &left_hand.get().landmarks[PinkyMcp].position,
            &left_hand.get().landmarks[PinkyPip].position,
            &left_hand.get().landmarks[PinkyDip].position,
            &left_hand.get().landmarks[RingFingerMcp].position,
        );
        rotate_g(&pip, pip_r, "pinky_pip", &mut mut_transform_q)?;
    }

    {
        let dip = left_hand_parts.pinky.dip;
        let dip_r = compute_dip_rotation(
            &left_hand.get().landmarks[PinkyMcp].position,
            &left_hand.get().landmarks[PinkyDip].position,
            &left_hand.get().landmarks[PinkyTip].position,
            &left_hand.get().landmarks[RingFingerMcp].position,
        );
        rotate_g(&dip, dip_r, "pinky_dip", &mut mut_transform_q)?;
    }

    {
        let left_palm = left_hand_parts.wrist;
        let l_palm_r = compute_left_palm_rotation(
            &left_hand.get().landmarks[Wrist].position,
            &left_hand.get().landmarks[MiddleFingerMcp].position,
            &left_hand.get().landmarks[ThumbCmc].position,
            &left_hand.get().landmarks[PinkyMcp].position,
        );
        rotate_g(&left_palm, l_palm_r, "left_palm", &mut mut_transform_q)?;
    }

    let right_hand = match current_hands.right_hand.as_ref() {
        Some(p) => p,
        None => return Ok(()),
    };

    {
        let mcp: Option<Entity> = right_hand_parts.thumb.mcp;
        let mcp_r = compute_mcp_rotation_thumb(
            &right_hand.get().landmarks[ThumbMcp].position,
            &right_hand.get().landmarks[ThumbMcp].position,
            &right_hand.get().landmarks[ThumbIp].position,
            &right_hand.get().landmarks[IndexFingerMcp].position,
        ) * Quat::from_rotation_x(-PI);
        rotate_g(&mcp, mcp_r, "thumb_mcp", &mut mut_transform_q)?;
    }

    {
        let ip = right_hand_parts.thumb.ip;
        let ip_r = compute_dip_rotation_thumb(
            &right_hand.get().landmarks[ThumbMcp].position,
            &right_hand.get().landmarks[ThumbIp].position,
            &right_hand.get().landmarks[ThumbTip].position,
            &right_hand.get().landmarks[IndexFingerMcp].position,
        ) * Quat::from_rotation_x(PI / 2.);
        rotate_g(&ip, ip_r, "thumb_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = right_hand_parts.index.mcp;
        let mcp_r = if gui_state.rotate_index_cmp {
            compute_mcp_rotation(
                &right_hand.get().landmarks[IndexFingerMcp].position,
                &right_hand.get().landmarks[IndexFingerPip].position,
                &right_hand.get().landmarks[IndexFingerDip].position,
                &right_hand.get().landmarks[ThumbMcp].position,
            ) * Quat::from_rotation_y(-PI / 2.)
        } else {
            Quat::IDENTITY
        };
        rotate_g(&mcp, mcp_r, "index_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = right_hand_parts.index.pip;
        let pip_r = compute_pip_rotation(
            &right_hand.get().landmarks[IndexFingerMcp].position,
            &right_hand.get().landmarks[IndexFingerPip].position,
            &right_hand.get().landmarks[IndexFingerDip].position,
            &right_hand.get().landmarks[ThumbMcp].position,
        );
        rotate_g(&pip, pip_r, "index_pip", &mut mut_transform_q)?;
    }

    {
        let dip = right_hand_parts.index.dip;
        let dip_r = compute_dip_rotation(
            &right_hand.get().landmarks[IndexFingerMcp].position,
            &right_hand.get().landmarks[IndexFingerDip].position,
            &right_hand.get().landmarks[IndexFingerTip].position,
            &right_hand.get().landmarks[ThumbMcp].position,
        );
        rotate_g(&dip, dip_r, "index_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = right_hand_parts.middle.mcp;
        let mcp_r = compute_mcp_rotation(
            &right_hand.get().landmarks[MiddleFingerMcp].position,
            &right_hand.get().landmarks[MiddleFingerPip].position,
            &right_hand.get().landmarks[MiddleFingerDip].position,
            &right_hand.get().landmarks[IndexFingerMcp].position,
        ) * Quat::from_rotation_y(-PI / 2.);
        rotate_g(&mcp, mcp_r, "middle_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = right_hand_parts.middle.pip;
        let pip_r = compute_pip_rotation(
            &right_hand.get().landmarks[MiddleFingerMcp].position,
            &right_hand.get().landmarks[MiddleFingerPip].position,
            &right_hand.get().landmarks[MiddleFingerDip].position,
            &right_hand.get().landmarks[IndexFingerMcp].position,
        );
        rotate_g(&pip, pip_r, "middle_pip", &mut mut_transform_q)?;
    }

    {
        let dip = right_hand_parts.middle.dip;

        let dip_r = compute_dip_rotation(
            &right_hand.get().landmarks[MiddleFingerMcp].position,
            &right_hand.get().landmarks[MiddleFingerDip].position,
            &right_hand.get().landmarks[MiddleFingerTip].position,
            &right_hand.get().landmarks[IndexFingerMcp].position,
        );
        rotate_g(&dip, dip_r, "middle_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = right_hand_parts.ring.mcp;
        let mcp_r = compute_mcp_rotation(
            &right_hand.get().landmarks[RingFingerMcp].position,
            &right_hand.get().landmarks[RingFingerPip].position,
            &right_hand.get().landmarks[RingFingerDip].position,
            &right_hand.get().landmarks[MiddleFingerMcp].position,
        ) * Quat::from_rotation_y(-PI / 2.);
        rotate_g(&mcp, mcp_r, "ring_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = right_hand_parts.ring.pip;
        let pip_r = compute_pip_rotation(
            &right_hand.get().landmarks[RingFingerMcp].position,
            &right_hand.get().landmarks[RingFingerPip].position,
            &right_hand.get().landmarks[RingFingerDip].position,
            &right_hand.get().landmarks[MiddleFingerMcp].position,
        ) * Quat::from_rotation_y(-PI / 2.);
        rotate_g(&pip, pip_r, "ring_pip", &mut mut_transform_q)?;
    }

    {
        let dip = right_hand_parts.ring.dip;
        let dip_r = compute_dip_rotation(
            &right_hand.get().landmarks[RingFingerMcp].position,
            &right_hand.get().landmarks[RingFingerDip].position,
            &right_hand.get().landmarks[RingFingerTip].position,
            &right_hand.get().landmarks[MiddleFingerMcp].position,
        ) * Quat::from_rotation_y(-PI / 2.);
        rotate_g(&dip, dip_r, "ring_dip", &mut mut_transform_q)?;
    }

    {
        let mcp = right_hand_parts.pinky.mcp;
        let mcp_r = compute_mcp_rotation(
            &right_hand.get().landmarks[PinkyMcp].position,
            &right_hand.get().landmarks[PinkyPip].position,
            &right_hand.get().landmarks[PinkyDip].position,
            &right_hand.get().landmarks[RingFingerMcp].position,
        ) * Quat::from_rotation_y(-PI / 2.);
        rotate_g(&mcp, mcp_r, "pinky_mcp", &mut mut_transform_q)?;
    }

    {
        let pip = right_hand_parts.pinky.pip;
        let pip_r = compute_pip_rotation(
            &right_hand.get().landmarks[PinkyMcp].position,
            &right_hand.get().landmarks[PinkyPip].position,
            &right_hand.get().landmarks[PinkyDip].position,
            &right_hand.get().landmarks[RingFingerMcp].position,
        ) * Quat::from_rotation_y(-PI / 2.);
        rotate_g(&pip, pip_r, "pinky_pip", &mut mut_transform_q)?;
    }

    {
        let dip = right_hand_parts.pinky.dip;
        let dip_r = compute_dip_rotation(
            &right_hand.get().landmarks[PinkyMcp].position,
            &right_hand.get().landmarks[PinkyDip].position,
            &right_hand.get().landmarks[PinkyTip].position,
            &right_hand.get().landmarks[RingFingerMcp].position,
        ) * Quat::from_rotation_y(-PI / 2.);
        rotate_g(&dip, dip_r, "pinky_dip", &mut mut_transform_q)?;
    }

    {
        let right_palm = right_hand_parts.wrist;
        let r_palm_r = compute_right_palm_rotation(
            &right_hand.get().landmarks[Wrist].position,
            &right_hand.get().landmarks[MiddleFingerMcp].position,
            &right_hand.get().landmarks[ThumbCmc].position,
            &right_hand.get().landmarks[PinkyMcp].position,
        );
        rotate_g(&right_palm, r_palm_r, "right_palm", &mut mut_transform_q)?;
    }
    Ok(())
}
