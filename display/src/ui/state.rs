use bevy::prelude::*;
use bevy::{ecs::resource::Resource, reflect::Reflect};
use typed_builder::TypedBuilder;

#[derive(Reflect)]
pub struct SliderRange(pub f32, pub f32);

#[derive(Reflect)]
pub struct Separator;

#[derive(Resource, Debug, TypedBuilder, Default, Reflect)]
#[reflect(Resource)]
pub struct GuiState {
    #[builder(default = false)]
    pub show_pose_landmarks: bool,
    #[builder(default = true)]
    pub show_pose_world_landmarks: bool,

    #[builder(default = false)]
    pub show_hand_landmarks: bool,
    #[builder(default = true)]
    pub show_hand_world_landmarks: bool,

    #[reflect(@Separator)]
    #[builder(default = true)]
    pub update_pose_data: bool,
    #[builder(default = true)]
    pub update_hands_data: bool,

    #[reflect(@Separator)]
    #[builder(default = true)]
    pub show_grid: bool,

    #[reflect(@Separator)]
    #[builder(default = true)]
    pub move_root: bool,

    #[reflect(@SliderRange(-5.0, 5.0))]
    #[builder(default = 1.)]
    pub move_scale: f32,

     #[reflect(@SliderRange(0.0, 1.0))]
    #[builder(default = 0.2)]
    pub move_eyes_scale: f32,

    #[reflect(@SliderRange(0., 1.0))]
    #[builder(default = 0.9)]
    pub edge: f32,

    #[reflect(@Separator)]
    #[builder(default = true)]
    pub rotate_root: bool,
    #[builder(default = true)]
    pub rotate_neck: bool,
    #[builder(default = true)]
    pub rotate_left_upper_arm: bool,
    #[builder(default = true)]
    pub rotate_right_upper_arm: bool,
    #[builder(default = true)]
    pub rotate_left_lower_arm: bool,
    #[builder(default = true)]
    pub rotate_right_lower_arm: bool,
    #[builder(default = true)]
    pub rotate_left_lower_arm_r: bool,
    #[builder(default = true)]
    pub rotate_right_lower_arm_r: bool,

    #[reflect(@Separator)]
    #[builder(default = true)]
    pub rotate_left_upper_leg: bool,
    #[builder(default = true)]
    pub rotate_right_upper_leg: bool,

    #[builder(default = true)]
    pub rotate_left_lower_leg: bool,
    #[builder(default = true)]
    pub rotate_right_lower_leg: bool,

    #[reflect(@Separator)]
    #[builder(default = true)]
    pub rotate_thumb_cmp: bool,
    #[builder(default = true)]
    pub rotate_index_cmp: bool,
}
