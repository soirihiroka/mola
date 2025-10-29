use axum::{Json, http::StatusCode, response::IntoResponse};
use bevy::app::{App, Plugin};

use bevy_simple_subsecond_system::hot;
use bevy_webserver::{RouterAppExt, WebServerConfig};
use serde_json::json;
use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::api::{
    face_api::{FaceExpression, get_face, set_face},
    hands_api::{CurrentHands, LastHandsUpdateTime, get_hands, set_hands},
    pose_api::LastPoseUpdateTime,
    face_api::CurrentFace,
};

use super::pose_api::{CurrentPose, get_pose, set_pose};
pub struct MocapApiPlugin;

impl Plugin for MocapApiPlugin {
    fn build(&self, app: &mut App) {
        let cors = CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any) // Allow any method
            .allow_headers(tower_http::cors::Any); // Allow any header
        // .allow_origin(["http://localhost:5000".parse::<HeaderValue>().unwrap()]) // Or whatever origin your client is on
        // .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        // .allow_headers([CONTENT_TYPE, AUTHORIZATION]); // This is critical!
        app.insert_resource(WebServerConfig {
            ip: IpAddr::V4(Ipv4Addr::from_str("0.0.0.0").unwrap()),
            port: 8088,
        });

        app.init_resource::<CurrentPose>();
        app.init_resource::<CurrentHands>();
        app.init_resource::<CurrentFace>();
        app.init_resource::<LastPoseUpdateTime>();
        app.init_resource::<LastHandsUpdateTime>();
        app.init_resource::<FaceExpression>();

        app.add_plugins(bevy_webserver::BevyWebServerPlugin);
        app.route("/", axum::routing::get(index))
            .route("/set_pose", axum::routing::post(set_pose))
            .route("/get_pose", axum::routing::get(get_pose))
            .route("/set_hands", axum::routing::post(set_hands))
            .route("/get_hands", axum::routing::get(get_hands))
            .route("/set_face", axum::routing::post(set_face))
            .route("/get_face", axum::routing::get(get_face))
            .route("/pair", axum::routing::get(pair));

        app.layer(TraceLayer::new_for_http());
        app.layer(cors.clone());
    }
}

async fn index() -> axum::response::Html<String> {
    hot_index()
}

#[hot]
fn hot_index() -> axum::response::Html<String> {
    axum::response::Html(
        r#"
        <html>
            <body>
                <p>Rendering server is running...</p>
                <p><a href="/get_pose">Current Pose</a></p>
            </body>
        </html>
        "#
        .to_string(),
    )
}

async fn pair() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "is_mocap": true,
            "version":1,
        })),
    )
}

#[hot]
pub fn internal_error(message: &str) -> axum::response::Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": message })),
    )
        .into_response()
}
