use std::time::Instant;

use crate::math::kalman_filter::VelocityKalman;
use crate::{character_control::pose::PoseData, ui::state::GuiState};
use axum::{Json, http::StatusCode, response::IntoResponse};
use bevy::{ecs::resource::Resource, log::tracing};
use bevy_defer::AsyncAccess;
use bevy_defer::AsyncWorld;
use bevy_simple_subsecond_system::hot;
use serde::{Deserialize, Serialize};

use super::api_server::internal_error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LandmarkJson {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub visibility: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoseLandmarkerResultJson {
    pub landmarks: Vec<Vec<LandmarkJson>>,
    #[serde(rename = "worldLandmarks")]
    pub world_landmarks: Vec<Vec<LandmarkJson>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoseDataJson {
    #[serde(rename = "poseLandmarkerResult")]
    pub pose_landmarker_result: PoseLandmarkerResultJson,
}

impl Default for PoseDataJson {
    fn default() -> Self {
        Self {
            pose_landmarker_result: PoseLandmarkerResultJson {
                landmarks: Vec::new(),
                world_landmarks: Vec::new(),
            },
        }
    }
}

// Define a Bevy Resource to hold the current PoseData
#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)] // Derive necessary traits
pub struct CurrentPose(pub Option<VelocityKalman<PoseData>>);

pub async fn get_pose() -> impl IntoResponse {
    get_pose_hot()
}

#[hot]
pub fn get_pose_hot() -> impl IntoResponse {
    match AsyncWorld
        .resource::<CurrentPose>()
        .get(|pose: &_| pose.0.clone())
    {
        Ok(pose) => (StatusCode::OK, Json(pose)).into_response(),
        Err(err) => {
            let message = format!("Failed to retrieve CurrentPose: {}", err);
            internal_error(&message)
        }
    }
}

pub async fn set_pose(Json(payload): Json<PoseDataJson>) -> impl IntoResponse {
    set_pose_hot(payload)
}

#[derive(Resource, Default, Clone, Debug)]
pub struct LastPoseUpdateTime(pub Option<Instant>);

#[hot]
pub fn set_pose_hot(payload: PoseDataJson) -> impl IntoResponse {
    match AsyncWorld
        .resource::<GuiState>()
        .get(|state| state.update_pose_data)
    {
        Ok(v) => {
            if !v {
                return StatusCode::OK.into_response();
            }
        }
        Err(err) => {
            let message = format!("Error accessing CurrentPose: {}", err);
            tracing::error!(message);
            return internal_error(&message);
        }
    };

    // First, convert PoseDataJson to PoseData
    let pose_data: PoseData = match payload.try_into() {
        Ok(data) => data,
        Err(err) => {
            let message = format!("Error converting PoseDataJson to PoseData: {}", err);
            tracing::error!(message);
            return internal_error(&message); // Or a more specific client error like BadRequest
        }
    };

    let now = Instant::now();

    // Retrieve and update the last update time
    let dt = match AsyncWorld.resource::<LastPoseUpdateTime>().get_mut(
        |last_update_time: &mut LastPoseUpdateTime| {
            let dt = last_update_time
                .0
                .map(|last| now.duration_since(last).as_secs_f32())
                .unwrap_or(0.0);
            last_update_time.0 = Some(now);
            dt
        },
    ) {
        Ok(dt) => dt,
        Err(err) => {
            let message = format!("Error accessing LastPoseUpdateTime: {}", err);
            tracing::error!(message);
            return internal_error(&message);
        }
    };

    match AsyncWorld
        .resource::<CurrentPose>()
        .get_mut(|current_pose: &mut CurrentPose| {
            // Check if a VelocityKalman<PoseData> already exists
            if let Some(kalman_pose) = &mut current_pose.0 {
                // If it exists, update it.
                // You'll need a 'dt' (delta time) for the update.
                // This 'dt' usually comes from how often this endpoint is hit,
                // or from a timestamp within your PoseDataJson.
                // For simplicity, let's assume a fixed dt for now, or you'll need to pass it.
                // For a real-world scenario, you'd calculate dt based on previous timestamp.
                // let dt = 1.0 / 60.0; // Example: assuming 60 FPS update rate
                kalman_pose.update(&pose_data, dt);
            } else {
                // If it doesn't exist, create a new one.
                current_pose.0 = Some(VelocityKalman::new(pose_data));
            }
        }) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(err) => {
            let message = format!("Error accessing CurrentPose: {}", err);
            tracing::error!(message);
            internal_error(&message)
        }
    }
}
