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
use bevy::core_pipeline::experimental;
use bevy::ecs::resource::Resource;
use bevy::log::tracing;
use bevy_defer::{AsyncAccess, AsyncWorld};
use bevy_simple_subsecond_system::hot;
use serde;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

/// Represents a predicted category for a blendshape.
#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]

pub struct FaceCategoryJson {
    #[serde(rename = "index")]
    pub index: u32,
    #[serde(rename = "score")]
    pub score: f32,
    #[serde(rename = "categoryName")]
    pub category_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

/// Contains the classification results for face blendshapes.
#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FaceBlendshapesJson {
    #[serde(rename = "categories")]
    pub categories: Vec<FaceCategoryJson>,
    #[serde(rename = "headIndex")]
    pub head_index: i32,
    #[serde(rename = "headName")]
    pub head_name: String,
}

/// The top-level structure for the entire FaceLandmarker result JSON object.
#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FaceLandmarkerResultJson {
    #[serde(rename = "faceLandmarks")]
    pub face_landmarks: Vec<Vec<LandmarkJson>>,
    #[serde(rename = "faceBlendshapes")]
    pub face_blendshapes: Vec<FaceBlendshapesJson>,
    #[serde(rename = "facialTransformationMatrixes")]
    pub facial_transformation_matrixes: Vec<Vec<f32>>,
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FaceLandmarkerResult {
    #[serde(rename = "faceLandmarkerResult")]
    pub face_landmarker_result: FaceLandmarkerResultJson,
}

#[derive(Resource, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FaceExpression {
    pub neutral: f32,
    pub brow_down_left: f32,
    pub brow_down_right: f32,
    pub brow_inner_up: f32,
    pub brow_outer_up_left: f32,
    pub brow_outer_up_right: f32,
    pub cheek_puff: f32,
    pub cheek_squint_left: f32,
    pub cheek_squint_right: f32,
    pub eye_blink_left: f32,
    pub eye_blink_right: f32,
    pub eye_look_down_left: f32,
    pub eye_look_down_right: f32,
    pub eye_look_in_left: f32,
    pub eye_look_in_right: f32,
    pub eye_look_out_left: f32,
    pub eye_look_out_right: f32,
    pub eye_look_up_left: f32,
    pub eye_look_up_right: f32,
    pub eye_squint_left: f32,
    pub eye_squint_right: f32,
    pub eye_wide_left: f32,
    pub eye_wide_right: f32,
    pub jaw_forward: f32,
    pub jaw_left: f32,
    pub jaw_open: f32,
    pub jaw_right: f32,
    pub mouth_close: f32,
    pub mouth_dimple_left: f32,
    pub mouth_dimple_right: f32,
    pub mouth_frown_left: f32,
    pub mouth_frown_right: f32,
    pub mouth_funnel: f32,
    pub mouth_left: f32,
    pub mouth_lower_down_left: f32,
    pub mouth_lower_down_right: f32,
    pub mouth_press_left: f32,
    pub mouth_press_right: f32,
    pub mouth_pucker: f32,
    pub mouth_right: f32,
    pub mouth_roll_lower: f32,
    pub mouth_roll_upper: f32,
    pub mouth_shrug_lower: f32,
    pub mouth_shrug_upper: f32,
    pub mouth_smile_left: f32,
    pub mouth_smile_right: f32,
    pub mouth_stretch_left: f32,
    pub mouth_stretch_right: f32,
    pub mouth_upper_up_left: f32,
    pub mouth_upper_up_right: f32,
    pub nose_sneer_left: f32,
    pub nose_sneer_right: f32,

    pub look_x: f32,
    pub look_y: f32,
}

impl From<&[FaceCategoryJson]> for FaceExpression {
    /// Converts a slice of `BlendshapeCategory` into a `FaceExpression` struct.
    fn from(categories: &[FaceCategoryJson]) -> Self {
        let mut expression = FaceExpression::default();

        for category in categories {
            match category.category_name.as_str() {
                "_neutral" => expression.neutral = category.score,
                "browDownLeft" => expression.brow_down_left = category.score,
                "browDownRight" => expression.brow_down_right = category.score,
                "browInnerUp" => expression.brow_inner_up = category.score,
                "browOuterUpLeft" => expression.brow_outer_up_left = category.score,
                "browOuterUpRight" => expression.brow_outer_up_right = category.score,
                "cheekPuff" => expression.cheek_puff = category.score,
                "cheekSquintLeft" => expression.cheek_squint_left = category.score,
                "cheekSquintRight" => expression.cheek_squint_right = category.score,
                "eyeBlinkLeft" => expression.eye_blink_left = category.score,
                "eyeBlinkRight" => expression.eye_blink_right = category.score,
                "eyeLookDownLeft" => expression.eye_look_down_left = category.score,
                "eyeLookDownRight" => expression.eye_look_down_right = category.score,
                "eyeLookInLeft" => expression.eye_look_in_left = category.score,
                "eyeLookInRight" => expression.eye_look_in_right = category.score,
                "eyeLookOutLeft" => expression.eye_look_out_left = category.score,
                "eyeLookOutRight" => expression.eye_look_out_right = category.score,
                "eyeLookUpLeft" => expression.eye_look_up_left = category.score,
                "eyeLookUpRight" => expression.eye_look_up_right = category.score,
                "eyeSquintLeft" => expression.eye_squint_left = category.score,
                "eyeSquintRight" => expression.eye_squint_right = category.score,
                "eyeWideLeft" => expression.eye_wide_left = category.score,
                "eyeWideRight" => expression.eye_wide_right = category.score,
                "jawForward" => expression.jaw_forward = category.score,
                "jawLeft" => expression.jaw_left = category.score,
                "jawOpen" => expression.jaw_open = category.score,
                "jawRight" => expression.jaw_right = category.score,
                "mouthClose" => expression.mouth_close = category.score,
                "mouthDimpleLeft" => expression.mouth_dimple_left = category.score,
                "mouthDimpleRight" => expression.mouth_dimple_right = category.score,
                "mouthFrownLeft" => expression.mouth_frown_left = category.score,
                "mouthFrownRight" => expression.mouth_frown_right = category.score,
                "mouthFunnel" => expression.mouth_funnel = category.score,
                "mouthLeft" => expression.mouth_left = category.score,
                "mouthLowerDownLeft" => expression.mouth_lower_down_left = category.score,
                "mouthLowerDownRight" => expression.mouth_lower_down_right = category.score,
                "mouthPressLeft" => expression.mouth_press_left = category.score,
                "mouthPressRight" => expression.mouth_press_right = category.score,
                "mouthPucker" => expression.mouth_pucker = category.score,
                "mouthRight" => expression.mouth_right = category.score,
                "mouthRollLower" => expression.mouth_roll_lower = category.score,
                "mouthRollUpper" => expression.mouth_roll_upper = category.score,
                "mouthShrugLower" => expression.mouth_shrug_lower = category.score,
                "mouthShrugUpper" => expression.mouth_shrug_upper = category.score,
                "mouthSmileLeft" => expression.mouth_smile_left = category.score,
                "mouthSmileRight" => expression.mouth_smile_right = category.score,
                "mouthStretchLeft" => expression.mouth_stretch_left = category.score,
                "mouthStretchRight" => expression.mouth_stretch_right = category.score,
                "mouthUpperUpLeft" => expression.mouth_upper_up_left = category.score,
                "mouthUpperUpRight" => expression.mouth_upper_up_right = category.score,
                "noseSneerLeft" => expression.nose_sneer_left = category.score,
                "noseSneerRight" => expression.nose_sneer_right = category.score,
                _ => {} // Ignore any unknown categories
            }
        }

        // Y-axis (Vertical): Average of "up" scores minus average of "down" scores.
        expression.look_y = (expression.eye_look_up_left + expression.eye_look_up_right) / 2.0
            - (expression.eye_look_down_left + expression.eye_look_down_right) / 2.0;

        // X-axis (Horizontal): Looking right (eyeLookInLeft, eyeLookOutRight) minus looking left.
        let look_right_score = (expression.eye_look_in_left + expression.eye_look_out_right) / 2.0;
        let look_left_score = (expression.eye_look_out_left + expression.eye_look_in_right) / 2.0;
        expression.look_x = look_right_score - look_left_score;

        expression
    }
}

pub async fn set_face(Json(payload): Json<FaceLandmarkerResult>) -> impl IntoResponse {
    set_face_hot(payload)
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize, PartialEq)]

pub struct CurrentFace {
    pub expression: Option<FaceExpression>,
}

#[hot]
fn set_face_hot(payload: FaceLandmarkerResult) -> impl IntoResponse {
    // println!("face: {:?}", payload);

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
            // use crate::api::api_server::internal_error;

            let message = format!("Error accessing CurrentPose: {}", err);
            tracing::error!(message);
            return internal_error(&message);
        }
    };

    match AsyncWorld
        .resource::<CurrentFace>()
        .get_mut(|face: &mut CurrentFace| {
            if let Some(first_face_blendshapes) =
                payload.face_landmarker_result.face_blendshapes.first()
            {
                // Convert the list of categories into our new, flat struct.
                let expressions =
                    FaceExpression::from(first_face_blendshapes.categories.as_slice());

                // Now you can access each blendshape by its name!
                // println!("Jaw Open: {}", expressions.jaw_open);
                // println!("Mouth Smile Left: {}", expressions.mouth_smile_left);
                // println!("Eye Blink Right: {}", expressions.eye_blink_right);
                // println!("Mouth Stretch Right: {}", expressions.mouth_stretch_right);

                // println!(
                //     "Eye Look Direction (X, Y): ({:.2}, {:.2})",
                //     expressions.look_x, expressions.look_y
                // );

                face.expression = Some(expressions);
            }
        }) {
        Ok(_) => return StatusCode::OK.into_response(),
        Err(err) => {
            let message = format!("Error accessing CurrentPose: {}", err);
            tracing::error!(message);
            return internal_error(&message);
        }
    }
}


pub async fn get_face() -> impl IntoResponse {
    get_face_hot()
}

#[hot]
async fn get_face_hot() -> impl IntoResponse {
    match AsyncWorld
        .resource::<CurrentFace>()
        .get(|face: &_| face.clone())
    {
        Ok(face) => (StatusCode::OK, Json(face)).into_response(),
        Err(err) => {
            let message = format!("Failed to retrieve CurrentPose: {}", err);
            internal_error(&message)
        }
    }
}
