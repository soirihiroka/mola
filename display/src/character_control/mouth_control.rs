use std::f32::consts::PI;

use crate::api::face_api::CurrentFace;
use crate::api::hands_api::CurrentHands;
use crate::api::hands_api::HandLandmarkIndex;
use crate::character_control::character_controller::CharacterParts;
use crate::character_control::hands::*;
use crate::character_control::mouth::MouthOverlay;
use crate::ui::state::GuiState;
use crate::ui::ui_controller::MouthTextures;
use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
#[hot(hot_patch_signature = true)]
pub fn control_mouth(
    parts: Res<CharacterParts>,
    mut mut_transform_q: Query<&mut Transform>,
    gui_state: Res<GuiState>,
    mouth_textures: Res<MouthTextures>,
    mut query: Query<&mut ImageNode, With<MouthOverlay>>,
    g_trans_q: Query<&GlobalTransform>,
    child_of_q: Query<&ChildOf>,
    curr_face: Res<CurrentFace>,
) -> Result {
    let face = match curr_face.expression.as_ref() {
        Some(p) => p,
        None => return Ok(()),
    };

    let mut mouth = match query.single_mut() {
        Ok(m) => m,
        Err(_) => return Ok(()),
    };

    mouth.image = if face.jaw_open > 0.5  {
        mouth_textures.open_big.clone()
    } else if face.jaw_open > 0.2 {
        mouth_textures.open_small.clone()
    } else {
        mouth_textures.closed.clone()
    };

    Ok(())
}
