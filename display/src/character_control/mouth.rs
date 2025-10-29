use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::character_control::character_controller::CharacterParts;

#[derive(Component)]
pub struct MouthOverlay;
pub struct MouthControlPlugin;

impl Plugin for MouthControlPlugin {
    fn build(&self, app: &mut App) {
        // app.init_gizmo_group::<LandmarkGizmos>();
        app
            // .add_systems(Startup, spawn_mouth_overlay)
            // .add_systems(Startup, spawn_mouth_overlay)
            .add_systems(Update, mouth_billboard);
    }
}

#[hot(hot_patch_signature = true)]
fn mouth_billboard(
    parts: Res<CharacterParts>,
    q_cam: Query<(&Camera, &GlobalTransform)>,
    q_transforms: Query<&GlobalTransform>,
    mut q_overlay: Query<(&mut Node, &mut Transform), With<MouthOverlay>>,
) {
    let Ok((camera, cam_transform)) = q_cam.single() else {
        return;
    };
    let Some(mouth_entity) = parts.mouth else {
        return;
    };
    let Ok(mouth_tf) = q_transforms.get(mouth_entity) else {
        return;
    };

    if let Ok(screen_pos) = camera.world_to_viewport(cam_transform, mouth_tf.translation()) {
        let distance = cam_transform.translation().distance(mouth_tf.translation());

        // let base_size = 100.0;
        let base_width = 128.0;
        let base_height = 64.0;
        let reference_distance = 1.0;
        let scale = (reference_distance / distance).clamp(0.2, 4.0) * 2.0;

        for (mut style, mut transform) in &mut q_overlay {
            // style.left = Val::Px(screen_pos.x - (base_size * scale) / 2.0);
            // style.top = Val::Px(screen_pos.y - (base_size * 0.5 * scale) / 2.0);

            // style.width = Val::Px(base_size * scale);
            // style.height = Val::Px((base_size * 0.5) * scale);

            // style.width = Val::Px(base_width * scale);
            // style.height = Val::Px(base_height * scale);

            // style.left = Val::Px(screen_pos.x - (base_width * scale) / 2.0);
            // style.top = Val::Px(screen_pos.y - (base_height * scale) / 2.0);

            // Position
            style.width = Val::Px(base_width * scale);
            style.height = Val::Px(base_height * scale);
            style.left = Val::Px(screen_pos.x - (base_width * scale) / 2.0);
            style.top = Val::Px(screen_pos.y - (base_height * scale) / 2.0);

            let forward = mouth_tf.forward().truncate(); // drop Y -> Vec2 (XZ plane)
            if forward.length_squared() > 1e-6 {
                let forward = forward.normalize();

                // Compare to +Z axis (0,1 in Vec2)
                let angle = forward.y.atan2(forward.x); // X=right, Y=forward (on XZ plane)

                // Size & Position
                style.width = Val::Px(base_width * scale);
                style.height = Val::Px(base_height * scale);
                style.left = Val::Px(screen_pos.x - (base_width * scale) / 2.0);
                style.top = Val::Px(screen_pos.y - (base_height * scale) / 2.0);

                // Rotation (2D billboard)
                transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
            }
        }
    }
}
