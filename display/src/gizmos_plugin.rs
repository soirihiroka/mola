use bevy::{
    app::{App, Plugin, Update},
    color::LinearRgba,
    ecs::system::Res,
    gizmos::{AppGizmoBuilder, gizmos::Gizmos},
    math::{Quat, UVec2, Vec2},
    prelude::*,
};

use crate::{
    api::hands_api::{CurrentHands, HandLandmarks},
    character_control::character_controller::CharacterParts,
    ui::state::GuiState,
};
use bevy_simple_subsecond_system::prelude::*;

use crate::api::pose_api::CurrentPose;
use bevy::color::palettes::css::*;
use std::f32::consts::PI;

pub struct GizmosPlugin;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct LandmarkGizmos;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<LandmarkGizmos>();
        app.add_systems(Update, draw_gizmos);
    }
}


#[hot(hot_patch_signature = true)]
fn draw_gizmos(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<LandmarkGizmos>,
    current_pose: Res<CurrentPose>,
    current_hands: Res<CurrentHands>,
    gui_state: Res<GuiState>,
) {
    if gui_state.show_grid {
        gizmos.grid(
            Quat::from_rotation_x(PI / 2.),
            UVec2::splat(40),
            Vec2::new(1., 1.),
            LinearRgba::gray(0.20),
        );
    }

    use PoseLandmarkIndex::*;

    use crate::{api::hands_api::HandLandmarkIndex, character_control::pose::PoseLandmarkIndex};

    if gui_state.show_pose_world_landmarks && current_pose.0.is_some() {
        let pose_value = current_pose.0.as_ref().unwrap();
        let landmarks = &pose_value.get().world_landmarks.data;

        let mut points = vec![];

        // Extract and draw spheres at each landmark
        for point in landmarks.iter() {
            let visibility = point.visibility;
            let color = if visibility > 0.75 {
                GREEN
            } else if visibility > 0.3 {
                YELLOW
            } else {
                RED
            };

            let pos = Vec3::new(point.position.x + 5., point.position.y, point.position.z);
            my_gizmos.sphere(pos, 0.015, color);
            points.push((pos, color));
        }
        let connections = [
            (Nose, LeftEyeInner),
            (LeftEyeInner, LeftEye),
            (LeftEye, LeftEyeOuter),
            (LeftEyeOuter, LeftEar),
            (Nose, RightEyeInner),
            (RightEyeInner, RightEye),
            (RightEye, RightEyeOuter),
            (RightEyeOuter, RightEar),
            (MouthLeft, MouthRight),
            (LeftShoulder, RightShoulder),
            (LeftShoulder, LeftElbow),
            (LeftElbow, LeftWrist),
            (LeftWrist, LeftPinky),
            (LeftWrist, LeftIndex),
            (LeftWrist, LeftThumb),
            (LeftPinky, LeftIndex),
            (RightShoulder, RightElbow),
            (RightElbow, RightWrist),
            (RightWrist, RightPinky),
            (RightWrist, RightIndex),
            (RightWrist, RightThumb),
            (RightPinky, RightIndex),
            (LeftShoulder, LeftHip),
            (RightShoulder, RightHip),
            (LeftHip, RightHip),
            (LeftHip, LeftKnee),
            (RightHip, RightKnee),
            (LeftKnee, LeftAnkle),
            (LeftAnkle, LeftHeel),
            (LeftHeel, LeftFootIndex),
            (RightKnee, RightAnkle),
            (RightAnkle, RightHeel),
            (RightHeel, RightFootIndex),
        ];

        for &(start, end) in &connections {
            let s = start as usize;
            let e = end as usize;
            if s < points.len() && e < points.len() {
                let (p1, _) = points[s];
                let (p2, _) = points[e];
                my_gizmos.line(p1, p2, GRAY);
            }
        }
    }

    if gui_state.show_pose_landmarks && current_pose.0.is_some() {
        let pose_value = current_pose.0.as_ref().unwrap();
        let mut points = vec![];

        // Extract and draw spheres at each landmark
        for point in pose_value.get().landmarks.data.iter() {
            let visibility = point.visibility;
            let color = if visibility > 0.75 {
                GREEN
            } else if visibility > 0.3 {
                YELLOW
            } else {
                RED
            };

            let pos = Vec3::new(point.position.x + 5., point.position.y, point.position.z);
            my_gizmos.sphere(pos, 0.015, color);
            points.push((pos, color));
        }
        let connections = [
            (Nose, LeftEyeInner),
            (LeftEyeInner, LeftEye),
            (LeftEye, LeftEyeOuter),
            (LeftEyeOuter, LeftEar),
            (Nose, RightEyeInner),
            (RightEyeInner, RightEye),
            (RightEye, RightEyeOuter),
            (RightEyeOuter, RightEar),
            (MouthLeft, MouthRight),
            (LeftShoulder, RightShoulder),
            (LeftShoulder, LeftElbow),
            (LeftElbow, LeftWrist),
            (LeftWrist, LeftPinky),
            (LeftWrist, LeftIndex),
            (LeftWrist, LeftThumb),
            (LeftPinky, LeftIndex),
            (RightShoulder, RightElbow),
            (RightElbow, RightWrist),
            (RightWrist, RightPinky),
            (RightWrist, RightIndex),
            (RightWrist, RightThumb),
            (RightPinky, RightIndex),
            (LeftShoulder, LeftHip),
            (RightShoulder, RightHip),
            (LeftHip, RightHip),
            (LeftHip, LeftKnee),
            (RightHip, RightKnee),
            (LeftKnee, LeftAnkle),
            (LeftAnkle, LeftHeel),
            (LeftHeel, LeftFootIndex),
            (RightKnee, RightAnkle),
            (RightAnkle, RightHeel),
            (RightHeel, RightFootIndex),
        ];

        for &(start, end) in &connections {
            let s = start as usize;
            let e = end as usize;
            if s < points.len() && e < points.len() {
                let (p1, _) = points[s];
                let (p2, _) = points[e];
                my_gizmos.line(p1, p2, GRAY);
            }
        }
    }

    use HandLandmarkIndex::*;

    // Define the connections between hand landmarks
    let hand_connections = [
        // Thumb
        (Wrist, ThumbCmc),
        (ThumbCmc, ThumbMcp),
        (ThumbMcp, ThumbIp),
        (ThumbIp, ThumbTip),
        // Index Finger
        (Wrist, IndexFingerMcp),
        (IndexFingerMcp, IndexFingerPip),
        (IndexFingerPip, IndexFingerDip),
        (IndexFingerDip, IndexFingerTip),
        // Middle Finger
        (Wrist, MiddleFingerMcp),
        (MiddleFingerMcp, MiddleFingerPip),
        (MiddleFingerPip, MiddleFingerDip),
        (MiddleFingerDip, MiddleFingerTip),
        // Ring Finger
        (Wrist, RingFingerMcp),
        (RingFingerMcp, RingFingerPip),
        (RingFingerPip, RingFingerDip),
        (RingFingerDip, RingFingerTip),
        // Pinky
        (Wrist, PinkyMcp),
        (PinkyMcp, PinkyPip),
        (PinkyPip, PinkyDip),
        (PinkyDip, PinkyTip),
        // Palm
        (IndexFingerMcp, MiddleFingerMcp),
        (MiddleFingerMcp, RingFingerMcp),
        (RingFingerMcp, PinkyMcp),
    ];

    // A reusable closure to draw the landmarks and connections for a single hand
    let mut draw_hand = |hand_landmarks: &HandLandmarks, color: Srgba| {
        let mut points = vec![];
        for point in hand_landmarks.data.iter() {
            // Apply the same offset as pose landmarks for consistent placement
            let pos = Vec3::new(point.position.x + 0.3, point.position.y, point.position.z) * 5.;
            my_gizmos.sphere(pos, 0.01, color); // Use a smaller radius for hand joints
            points.push(pos);
        }

        for &(start, end) in &hand_connections {
            let s = start as usize;
            let e = end as usize;
            // Ensure the landmarks exist before trying to draw a line
            if s < points.len() && e < points.len() {
                my_gizmos.line(points[s], points[e], GRAY);
            }
        }
    };

    // Draw the left hand if its data is available
    if let Some(hand) = &current_hands.left_hand {
        if gui_state.show_hand_landmarks {
            draw_hand(&hand.get().landmarks, GREEN);
        }
        if gui_state.show_hand_world_landmarks {
            draw_hand(&hand.get().world_landmarks, GREEN);
        }
    }

    // Draw the right hand if its data is available
    if let Some(hand) = &current_hands.right_hand {
        if gui_state.show_hand_landmarks {
            draw_hand(&hand.get().landmarks, BLUE);
        }
        if gui_state.show_hand_world_landmarks {
            draw_hand(&hand.get().world_landmarks, BLUE);
        }
    }
    // }
}
