#[derive(Component)]
pub struct MouthOverlay;

#[hot(hot_patch_signature = true)]
fn mouth_billboard(
    parts: Res<CharacterParts>,
    q_cam: Query<(&Camera, &GlobalTransform)>,
    q_transforms: Query<&GlobalTransform>,
    mut q_overlay: Query<&mut Node, With<MouthOverlay>>,
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

        let base_size = 40.0;
        let reference_distance = 5.0;
        let scale = (reference_distance / distance).clamp(0.5, 2.0);

        for mut style in &mut q_overlay {
            style.left = Val::Px(screen_pos.x - (base_size * scale) / 2.0);
            style.top = Val::Px(screen_pos.y - (base_size * 0.5 * scale) / 2.0);

            style.width = Val::Px(base_size * scale);
            style.height = Val::Px((base_size * 0.5) * scale);
        }
    }
}


pub struct MouthControlPlugin;

impl Plugin for MouthControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<LandmarkGizmos>();
        app.add_systems(Update, draw_gizmos)
            // .add_systems(Startup, spawn_mouth_overlay)
            .add_systems(Update, mouth_billboard);
    }
}
