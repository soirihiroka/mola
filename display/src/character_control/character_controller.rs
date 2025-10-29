use crate::character_control::find_entity::{debug_named_entity, find_named_entity};
use crate::character_control::mouth_control::control_mouth;
use crate::character_control::move_eyes::move_eyes;
use crate::character_control::pose::*;
use crate::character_control::rotate_body::rotate_body;
use crate::character_control::rotate_hands::rotate_hands;
use crate::ui::state::GuiState;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

use crate::api::pose_api::CurrentPose;
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(CharacterParts::default())
            .add_systems(
                Update,
                (move_character, rotate_body, control_mouth,rotate_hands, move_eyes),
            )
            .add_observer(find_named_entity)
            .add_observer(debug_named_entity);
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct CharacterParts {
    pub root: Option<Entity>,
    pub neck: Option<Entity>,

    pub left_eye: Option<Entity>,
    pub right_eye: Option<Entity>,

    pub g_pencil: Option<Entity>,

    pub mouth: Option<Entity>,

    pub left_arm: ArmParts,
    pub right_arm: ArmParts,

    pub left_leg: LimbParts,
    pub right_leg: LimbParts,

    pub left_hand: HandParts,
    pub right_hand: HandParts,
}

#[derive(Resource, Default, Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct LimbParts {
    pub upper: Option<Entity>,
    pub lower: Option<Entity>,
}

#[derive(Resource, Default, Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct ArmParts {
    pub upper: Option<Entity>,
    pub lower: Option<Entity>,
    pub lower_r: Option<Entity>,
}

#[derive(Resource, Default, Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct HandParts {
    pub wrist: Option<Entity>,
    pub thumb: ThumbParts,
    pub index: FingerParts,
    pub middle: FingerParts,
    pub ring: FingerParts,
    pub pinky: FingerParts,
}
#[derive(Resource, Default, Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct FingerParts {
    pub mcp: Option<Entity>,
    pub pip: Option<Entity>,
    pub dip: Option<Entity>,
}

#[derive(Resource, Default, Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct ThumbParts {
    pub mcp: Option<Entity>,
    pub ip: Option<Entity>,
}

#[hot]
fn move_character(
    parts: Res<CharacterParts>,
    mut mut_transform_q: Query<&mut Transform>,
    gui_state: Res<GuiState>,
    current_pose: Res<CurrentPose>,
    // mut gizmos: Gizmos,
) -> Result {
    let pose = match current_pose.0.as_ref() {
        Some(p) => p,
        None => return Ok(()),
    };
    let root = parts.root.ok_or("No Root")?;
    let root_position_target = if gui_state.move_root {
        ((pose.get().landmarks[PoseLandmarkIndex::LeftHip].position
            + pose.get().landmarks[PoseLandmarkIndex::RightHip].position)
            / 2.
            + Vec3::new(0., 2., 0.))
            * gui_state.move_scale
    } else {
        Vec3::new(0., 0., 0.)
    };

    let mut root_transform = mut_transform_q.get_mut(root)?;

    root_transform.translation = root_position_target;
    Ok(())
}
