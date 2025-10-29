use std::ops::{Add, Mul, Sub};
use std::time::Instant;

use crate::api::api_server::internal_error;
use crate::api::pose_api::LandmarkJson;
use crate::math::kalman_filter::VelocityKalman;
use crate::math::landmarks::{LandmarkIndex, Landmarks};
use crate::ui::state::GuiState;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use bevy::ecs::resource::Resource;
use bevy::log::{info, tracing};
use bevy_defer::{AsyncAccess, AsyncWorld};
use bevy_simple_subsecond_system::hot;
use serde;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter, Serialize, Deserialize)]
pub enum HandLandmarkIndex {
    Wrist = 0,
    ThumbCmc,
    ThumbMcp,
    ThumbIp,
    ThumbTip,
    IndexFingerMcp,
    IndexFingerPip,
    IndexFingerDip,
    IndexFingerTip,
    MiddleFingerMcp,
    MiddleFingerPip,
    MiddleFingerDip,
    MiddleFingerTip,
    RingFingerMcp,
    RingFingerPip,
    RingFingerDip,
    RingFingerTip,
    PinkyMcp,
    PinkyPip,
    PinkyDip,
    PinkyTip,
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandednessJson {
    #[serde(rename = "categoryName")]
    pub category_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "index")]
    pub index: u32,
    #[serde(rename = "score")]
    pub score: f32,
}
#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandLandmarkerResultJson {
    #[serde(rename = "handedness")]
    pub handedness: Vec<Vec<HandednessJson>>,
    #[serde(rename = "handednesses")]
    pub handednesses: Vec<Vec<HandednessJson>>,
    #[serde(rename = "landmarks")]
    pub landmarks: Vec<Vec<LandmarkJson>>,
    #[serde(rename = "worldLandmarks")]
    pub multi_hand_landmarks: Vec<Vec<LandmarkJson>>,
}
#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandLandmarkerResult {
    #[serde(rename = "handLandmarkerResult")]
    pub hand_landmarker_result: HandLandmarkerResultJson,
}

#[derive(Resource, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum HandednessName {
    Left,
    Right,
}
#[derive(Resource, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Handedness {
    pub name: HandednessName,
    pub score: f32,
}

pub async fn set_hands(Json(payload): Json<HandLandmarkerResult>) -> impl IntoResponse {
    set_hands_hot(payload)
}

pub type HandLandmarks = Landmarks<HandLandmarkIndex>;
impl LandmarkIndex for HandLandmarkIndex {
    fn as_index(self) -> usize {
        self as usize
    }
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandKeyPoints {
    pub landmarks: HandLandmarks,
    pub world_landmarks: HandLandmarks,
    pub handedness: Handedness,
}

impl Add for &HandKeyPoints {
    type Output = HandKeyPoints;

    fn add(self, rhs: Self) -> Self::Output {
        HandKeyPoints {
            landmarks: &self.landmarks + &rhs.landmarks,
            world_landmarks: &self.world_landmarks + &rhs.world_landmarks,
            handedness: self.handedness,
        }
    }
}

impl Sub for &HandKeyPoints {
    type Output = HandKeyPoints;

    fn sub(self, rhs: Self) -> Self::Output {
        HandKeyPoints {
            landmarks: &self.landmarks - &rhs.landmarks,
            world_landmarks: &self.world_landmarks - &rhs.world_landmarks,
            handedness: self.handedness,
        }
    }
}

impl Mul<f32> for &HandKeyPoints {
    type Output = HandKeyPoints;

    fn mul(self, rhs: f32) -> Self::Output {
        HandKeyPoints {
            landmarks: &self.landmarks * rhs,
            world_landmarks: &self.world_landmarks * rhs,
            handedness: self.handedness,
        }
    }
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CurrentHands {
    pub left_hand: Option<VelocityKalman<HandKeyPoints>>,
    pub right_hand: Option<VelocityKalman<HandKeyPoints>>,
}

impl From<HandLandmarkerResultJson> for CurrentHands {
    fn from(value: HandLandmarkerResultJson) -> Self {
        let left_hand = match value.landmarks.get(0) {
            Some(v) => {
                info!("Detected left hand");
                let handedness = Handedness {
                    name: HandednessName::Left,
                    score: value
                        .handedness
                        .get(0)
                        .and_then(|v| v.get(0))
                        .map_or(0.0, |v| v.score), // Using map_or is slightly cleaner
                };
                let world_landmarks: HandLandmarks = value
                    .multi_hand_landmarks
                    .get(0)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let landmarks = v.try_into().unwrap();
                // info!("Detected set left hand");
                Some(VelocityKalman::new(HandKeyPoints {
                    landmarks,
                    world_landmarks,
                    handedness,
                }).set_measurement_noise(100.0).clone())
            }
            None => None,
        };
        // You will likely need to do the same for the right hand
        let right_hand = match value.landmarks.get(1) {
            // Assuming index 1 for the right hand
            Some(v) => {
                let handedness = Handedness {
                    name: HandednessName::Right,
                    score: value
                        .handedness
                        .get(1)
                        .and_then(|v| v.get(0))
                        .map_or(0.0, |v| v.score),
                };
                let world_landmarks: HandLandmarks = value
                    .multi_hand_landmarks
                    .get(1)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let landmarks = v.try_into().unwrap();
                Some(VelocityKalman::new(HandKeyPoints {
                    landmarks,
                    world_landmarks,
                    handedness,
                }).set_measurement_noise(10.0).clone())
            }
            None => None,
        };

        return CurrentHands {
            left_hand,
            right_hand,
        };
    }
}

#[derive(Resource, Default, Clone, Debug)]
pub struct LastHandsUpdateTime(pub Option<Instant>);

#[hot]
fn set_hands_hot(payload: HandLandmarkerResult) -> impl IntoResponse {
    match AsyncWorld
        .resource::<GuiState>()
        .get(|state| state.update_hands_data)
    {
        Ok(v) => {
            if !v {
                // info!("Not updating pose data");
                return StatusCode::OK.into_response();
            }
        }
        Err(err) => {
            let message = format!("Error accessing CurrentPose: {}", err);
            tracing::error!(message);
            return internal_error(&message);
        }
    };

    let now = Instant::now();

    // Retrieve and update the last update time
    let dt = match AsyncWorld.resource::<LastHandsUpdateTime>().get_mut(
        |last_update_time: &mut LastHandsUpdateTime| {
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
            let message = format!("Error accessing LastHandsUpdateTime: {}", err);
            tracing::error!(message);
            return internal_error(&message);
        }
    };

    match AsyncWorld
        .resource::<CurrentHands>()
        .get_mut(|hands: &mut CurrentHands| {
            let new_hands: CurrentHands = payload.hand_landmarker_result.into();

            if let Some(new_left) = new_hands.left_hand {
                if let Some(existing_left) = hands.left_hand.as_mut() {
                    existing_left.update(new_left.get(), dt);
                } else {
                    hands.left_hand = Some(new_left); // fixed
                }
            }
            if let Some(new_right) = new_hands.right_hand {
                if let Some(existing_right) = hands.right_hand.as_mut() {
                    existing_right.update(new_right.get(), dt);
                } else {
                    hands.right_hand = Some(new_right); // fixed
                }
            }
        }) {
        Ok(_) => {}
        Err(err) => {
            let message = format!("Error accessing CurrentPose: {}", err);
            tracing::error!(message);
            return internal_error(&message).into_response();
        }
    }

    return StatusCode::OK.into_response();
}

pub async fn get_hands() -> impl IntoResponse {
    get_hands_hot()
}

#[hot]
async fn get_hands_hot() -> impl IntoResponse {
    match AsyncWorld
        .resource::<CurrentHands>()
        .get(|hands: &_| hands.clone())
    {
        Ok(hands) => (StatusCode::OK, Json(hands)).into_response(),
        Err(err) => {
            let message = format!("Failed to retrieve CurrentPose: {}", err);
            internal_error(&message)
        }
    }
}
