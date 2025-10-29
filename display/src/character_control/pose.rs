use crate::math::landmarks::{LandmarkIndex, Landmarks, LandmarksError};
use bevy::math::{Mat3, Quat, Vec3};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::ops::{Add, Mul, Sub};
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter, FromRepr)]
#[repr(usize)]
pub enum PoseLandmarkIndex {
    Nose = 0,
    LeftEyeInner,
    LeftEye,
    LeftEyeOuter,
    RightEyeInner,
    RightEye,
    RightEyeOuter,
    LeftEar,
    RightEar,
    MouthLeft,
    MouthRight,
    LeftShoulder,
    RightShoulder,
    LeftElbow,
    RightElbow,
    LeftWrist,
    RightWrist,
    LeftPinky,
    RightPinky,
    LeftIndex,
    RightIndex,
    LeftThumb,
    RightThumb,
    LeftHip,
    RightHip,
    LeftKnee,
    RightKnee,
    LeftAnkle,
    RightAnkle,
    LeftHeel,
    RightHeel,
    LeftFootIndex,
    RightFootIndex,
}

impl LandmarkIndex for PoseLandmarkIndex {
    fn as_index(self) -> usize {
        self as usize
    }
}

use PoseLandmarkIndex::*;

use bevy_simple_subsecond_system::prelude::*;

use crate::api::pose_api::PoseDataJson;

type PoseLandmarks = Landmarks<PoseLandmarkIndex>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoseData {
    pub world_landmarks: PoseLandmarks,
    pub landmarks: PoseLandmarks,
}

impl Add for &PoseData {
    type Output = PoseData;

    fn add(self, rhs: Self) -> Self::Output {
        PoseData {
            world_landmarks: &self.world_landmarks + &rhs.world_landmarks,
            landmarks: &self.landmarks + &rhs.landmarks,
        }
    }
}

impl Sub for &PoseData {
    type Output = PoseData;

    fn sub(self, rhs: Self) -> Self::Output {
        PoseData {
            world_landmarks: &self.world_landmarks - &rhs.world_landmarks,
            landmarks: &self.landmarks - &rhs.landmarks,
        }
    }
}

impl Mul<f32> for &PoseData {
    type Output = PoseData;

    fn mul(self, rhs: f32) -> Self::Output {
        PoseData {
            world_landmarks: &self.world_landmarks * rhs,
            landmarks: &self.landmarks * rhs,
        }
    }
}

impl TryFrom<PoseDataJson> for PoseData {
    type Error = LandmarksError;
    fn try_from(value: PoseDataJson) -> Result<Self, Self::Error> {
        let landmarks_vec_ref = value.pose_landmarker_result.landmarks.get(0);
        let landmarks_vec = match landmarks_vec_ref {
            Some(v) => v.clone(),
            None => return Err(LandmarksError::NoKeyPoints),
        };
        let landmarks: PoseLandmarks = match (landmarks_vec).try_into() {
            Ok(v) => v,
            Err(err) => return Err(err), // Corrected return for error
        };
        let world_landmarks_vec_ref = value.pose_landmarker_result.world_landmarks.get(0);
        let world_landmarks_vec = match world_landmarks_vec_ref {
            Some(v) => v.clone(),
            None => return Err(LandmarksError::NoKeyPoints),
        };
        let world_landmarks: PoseLandmarks = match world_landmarks_vec.try_into() {
            Ok(v) => v,
            Err(err) => return Err(err),
        };
        Ok(Self {
            world_landmarks,
            landmarks,
        })
    }
}
#[hot]
pub fn compute_neck_rotation(pose: &PoseLandmarks) -> Quat {
    let nose = pose[Nose].position;
    let left_ear = pose[LeftEar].position;
    let right_ear = pose[RightEar].position;

    // Midpoint between ears = approximate center of head
    let head_center = (left_ear + right_ear) * 0.5;

    let forward = (nose - head_center).normalize();
    let right = (right_ear - left_ear).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix) * Quat::from_rotation_y(-PI / 2. - 0.3) * Quat::from_rotation_x(PI - 0.35) 
}
#[hot]
pub fn compute_root_rotation(pose: &PoseLandmarks) -> Quat {
    let ls = pose[LeftShoulder].position;
    let rs = pose[RightShoulder].position;
    let lh = pose[LeftHip].position;
    let rh = pose[RightHip].position;

    let shoulder_center = (ls + rs) * 0.5;
    let hip_center = (lh + rh) * 0.5;

    let up = (shoulder_center - hip_center).normalize();
    let right = (lh - rh).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix)
}
#[hot]
pub fn compute_left_upper_arm_rotation(pose: &PoseLandmarks) -> Quat {
    let left_shoulder = pose[LeftShoulder].position;
    let left_elbow = pose[LeftElbow].position;
    let left_hip = pose[LeftHip].position;

    let up = (left_elbow - left_shoulder).normalize();
    let forward = (left_shoulder - left_hip).normalize();
    let right = up.cross(forward).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix)
}
#[hot]
pub fn compute_left_lower_arm_rotation(pose: &PoseLandmarks) -> Quat {
    let left_shoulder = pose[LeftShoulder].position;
    let left_elbow = pose[LeftElbow].position;
    let left_wrist = pose[LeftWrist].position;

    let up = (left_wrist - left_elbow).normalize();
    let forward = (left_elbow - left_shoulder).normalize();
    let right: Vec3 = up.cross(forward).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix)
}
#[hot]
pub fn compute_right_upper_arm_rotation(pose: &PoseLandmarks) -> Quat {
    let right_shoulder = pose[RightShoulder].position;
    let right_elbow = pose[RightElbow].position;
    let right_hip = pose[RightHip].position;

    let up = (right_elbow - right_shoulder).normalize();
    let forward = (right_shoulder - right_hip).normalize();
    let right = up.cross(forward).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix)
}
#[hot]
pub fn compute_right_lower_arm_rotation(pose: &PoseLandmarks) -> Quat {
    let right_shoulder = pose[RightShoulder].position;
    let right_elbow = pose[RightElbow].position;
    let right_wrist = pose[RightWrist].position;

    let up = (right_wrist - right_elbow).normalize();
    let forward = (right_elbow - right_shoulder).normalize();
    let right = up.cross(forward).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix)
}

#[hot]
pub fn compute_left_upper_leg_rotation(pose: &PoseLandmarks) -> Quat {
    let right_hip = pose[RightHip].position;
    let left_hip = pose[LeftHip].position;
    let left_knee = pose[LeftKnee].position;

    let up = (left_knee - left_hip).normalize();
    let right = (left_hip - right_hip).normalize();
    let forward = right.cross(up).normalize();
    let right = up.cross(forward).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix) * Quat::from_rotation_y(PI)
}

#[hot]
pub fn compute_left_lower_leg_rotation(pose: &PoseLandmarks) -> Quat {
    let left_hip = pose[LeftHip].position;
    let left_knee = pose[LeftKnee].position;
    let left_ankle = pose[LeftAnkle].position;

    let up = (left_ankle - left_knee).normalize();
    let forward = (left_knee - left_hip).normalize();
    let right = up.cross(forward).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix) * Quat::from_rotation_y(-PI / 2.)
}

#[hot]
pub fn compute_right_upper_leg_rotation(pose: &PoseLandmarks) -> Quat {
    let left_hip = pose[LeftHip].position;
    let right_hip = pose[RightHip].position;
    let right_knee = pose[RightKnee].position;

    let up = (right_knee - right_hip).normalize();
    let right = (right_hip - left_hip).normalize();
    let forward = right.cross(up).normalize();
    let right = up.cross(forward).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix) * Quat::from_rotation_y(PI)
}

#[hot]
pub fn compute_right_lower_leg_rotation(pose: &PoseLandmarks) -> Quat {
    let right_hip = pose[RightHip].position;
    let right_knee = pose[RightKnee].position;
    let right_ankle = pose[RightAnkle].position;

    let up = (right_ankle - right_knee).normalize();
    let forward = (right_knee - right_hip).normalize();
    let right = up.cross(forward).normalize();
    let forward = right.cross(up).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix) * Quat::from_rotation_y(PI / 2.)
}

#[hot]
pub fn compute_left_lower_arm_r_rotation(pose: &PoseLandmarks) -> Quat {
    let left_elbow = pose[LeftElbow].position;
    let left_wrist = pose[LeftWrist].position;

    let left_index = pose[LeftIndex].position;
    let left_thumb = pose[LeftThumb].position;

    let forward = (left_wrist - left_elbow).normalize();
    let right = (left_index - left_thumb).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix) * Quat::from_rotation_x(PI / 2.) * Quat::from_rotation_y(PI)
}

#[hot]
pub fn compute_right_lower_arm_r_rotation(pose: &PoseLandmarks) -> Quat {
    let right_elbow = pose[RightElbow].position;
    let right_wrist = pose[RightWrist].position;

    let right_index = pose[RightIndex].position;
    let right_thumb = pose[RightThumb].position;

    let forward = (right_wrist - right_elbow).normalize();
    let right = (right_index - right_thumb).normalize();
    let up = forward.cross(right).normalize();
    let right = up.cross(forward).normalize();

    let rot_matrix = Mat3::from_cols(right, up, forward);

    Quat::from_mat3(&rot_matrix) * Quat::from_rotation_x(PI / 2.)
}
