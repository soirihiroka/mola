use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::character_control::character_controller::CharacterParts;

pub fn debug_named_entity(
    _trigger: Trigger<SceneInstanceReady>,
    query: Query<(Entity, &Name)>,
    world: &World,
) {
    for (entity, name) in &query {
        let name_str = name.as_str();
        match name_str {
            "GPencil" => {
                info!("Found GPencil");
                if let Ok(components_iterator) = world.inspect_entity(entity) {
                    // If successful, now we can loop over the components.
                    for component_info in components_iterator {
                        info!("  - Component: {:?}", component_info);
                    }
                } else {
                    // Optionally handle the case where the entity doesn't exist
                    // error!("Could not inspect entity {:?}, it may not exist.", entity);
                }
            }
            _ => {}
        }
    }
}

pub fn find_named_entity(
    _trigger: Trigger<SceneInstanceReady>,
    query: Query<(Entity, &Name)>,
    // archetypes: &Archetypes,
    // world: &World,
    mut parts: ResMut<CharacterParts>,
    // mut commands: Commands, // Add Commands to modify entities
) {
    for (entity, name) in &query {
        let name_str = name.as_str();
        match name_str {
            "Eye.L" => {
                info!("Found left eye {entity}");
                parts.left_eye = Some(entity);
            }
            "Eye.R" => {
                info!("Found right eye {entity}");
                parts.right_eye = Some(entity);
            }
            "Neck" => {
                parts.neck = Some(entity);
            }
            "GPencil" => {
                info!("Found GPencil");
                parts.g_pencil = Some(entity);
                // while let Ok(comp) = world.inspect_entity(entity) {
                //     for i in comp {
                //         println!("{:?}", i);
                //     }
                // }
            }
            "Mouth" => {
                info!("Found mouth {entity}");
                parts.mouth = Some(entity);
            }
            "Root" => {
                parts.root = Some(entity);
            }
            "UpperArm.L" => {
                parts.left_arm.upper = Some(entity);
            }
            "LowerArm.L" => {
                parts.left_arm.lower = Some(entity);
            }
            "LowerArmR.L" => {
                parts.left_arm.lower_r = Some(entity);
            }
            "UpperArm.R" => {
                parts.right_arm.upper = Some(entity);
            }
            "LowerArm.R" => {
                parts.right_arm.lower = Some(entity);
            }
            "LowerArmR.R" => {
                parts.right_arm.lower_r = Some(entity);
            }
            "Palm.L" => {
                parts.left_hand.wrist = Some(entity);
            }
            "ThumbMcp.L" => {
                parts.left_hand.thumb.mcp = Some(entity);
            }
            "ThumbIp.L" => {
                parts.left_hand.thumb.ip = Some(entity);
            }
            "IndexMcp.L" => {
                parts.left_hand.index.mcp = Some(entity);
            }
            "IndexPip.L" => {
                parts.left_hand.index.pip = Some(entity);
            }
            "IndexDip.L" => {
                parts.left_hand.index.dip = Some(entity);
            }
            "MiddleMcp.L" => {
                parts.left_hand.middle.mcp = Some(entity);
            }
            "MiddlePip.L" => {
                parts.left_hand.middle.pip = Some(entity);
            }
            "MiddleDip.L" => {
                parts.left_hand.middle.dip = Some(entity);
            }
            "RingMcp.L" => {
                parts.left_hand.ring.mcp = Some(entity);
            }
            "RingPip.L" => {
                parts.left_hand.ring.pip = Some(entity);
            }
            "RingDip.L" => {
                parts.left_hand.ring.dip = Some(entity);
            }
            "PinkyMcp.L" => {
                parts.left_hand.pinky.mcp = Some(entity);
            }
            "PinkyPip.L" => {
                parts.left_hand.pinky.pip = Some(entity);
            }
            "PinkyDip.L" => {
                parts.left_hand.pinky.dip = Some(entity);
            }
            "Palm.R" => {
                parts.right_hand.wrist = Some(entity);
            }
            "ThumbMcp.R" => {
                parts.right_hand.thumb.mcp = Some(entity);
            }
            "ThumbIp.R" => {
                parts.right_hand.thumb.ip = Some(entity);
            }
            "IndexMcp.R" => {
                parts.right_hand.index.mcp = Some(entity);
            }
            "IndexPip.R" => {
                parts.right_hand.index.pip = Some(entity);
            }
            "IndexDip.R" => {
                parts.right_hand.index.dip = Some(entity);
            }
            "MiddleMcp.R" => {
                parts.right_hand.middle.mcp = Some(entity);
            }
            "MiddlePip.R" => {
                parts.right_hand.middle.pip = Some(entity);
            }
            "MiddleDip.R" => {
                parts.right_hand.middle.dip = Some(entity);
            }
            "RingMcp.R" => {
                parts.right_hand.ring.mcp = Some(entity);
            }
            "RingPip.R" => {
                parts.right_hand.ring.pip = Some(entity);
            }
            "RingDip.R" => {
                parts.right_hand.ring.dip = Some(entity);
            }
            "PinkyMcp.R" => {
                parts.right_hand.pinky.mcp = Some(entity);
            }
            "PinkyPip.R" => {
                parts.right_hand.pinky.pip = Some(entity);
            }
            "PinkyDip.R" => {
                parts.right_hand.pinky.dip = Some(entity);
            }
            "UpperLeg.L" => {
                parts.left_leg.upper = Some(entity);
            }
            "LowerLeg.L" => {
                parts.left_leg.lower = Some(entity);
            }
            "UpperLeg.R" => {
                parts.right_leg.upper = Some(entity);
            }
            "LowerLeg.R" => {
                parts.right_leg.lower = Some(entity);
            }
            _ => {
                info!(
                    "Entity {:?} with name {} did not match any specific tags.",
                    entity, name_str
                );
            }
        }
    }
}
