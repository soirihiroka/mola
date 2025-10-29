use bevy::color::palettes::css::RED;
use bevy::core_pipeline::prepass::{DepthPrepass, NormalPrepass};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat};
use bevy::render::{camera::RenderTarget, render_resource::TextureUsages};
use bevy::{
    app::{App, Plugin},
    ecs::system::ResMut,
};
use bevy_egui::{EguiPlugin, EguiUserTextures, egui};

use crate::camera_controller::CameraController;
use crate::character_control::mouth::MouthOverlay;
// use crate::gizmos_plugin::MouthOverlay;
use crate::material::post_processing_moebius::MoebiusPostProcessSettings;
// use crate::material::post_processing_plugin::PostProcessSettings;
use crate::ui::slider::AdwSlider;
use crate::ui::theme::adw_colors;
use crate::ui::toggle_switch::toggle;
use bevy_egui::egui::Stroke;
use bevy_egui::egui::{FontId, RichText, TextStyle};
use bevy_egui::{
    EguiContextPass, EguiContexts,
    egui::{CornerRadius, FontData, FontDefinitions, FontFamily, Visuals},
};
use bevy_simple_subsecond_system::prelude::*;

use crate::ui::state::GuiState;

#[derive(Resource, Default)]
pub struct SceneViewFocus(pub bool);

#[derive(Event)]
struct ResizeScenePreview {
    size: Extent3d,
}

#[derive(Deref, Resource)]
pub struct ScenePreviewImage(pub Handle<Image>);

pub struct GuiControllerPlugin;

impl Plugin for GuiControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<GuiState>(GuiState::builder().build())
            .init_resource::<SceneViewFocus>();
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        });

        app.add_event::<ResizeScenePreview>();
        app.add_systems(EguiContextPass, (ui_system, render_to_image_system));
        app.add_systems(PostUpdate, resize_scene_texture_system);
        app.add_systems(Startup, setup_system);
    }
}

fn resize_scene_texture_system(
    mut resize_events: EventReader<ResizeScenePreview>,
    scene_preview_image: Res<ScenePreviewImage>,
    mut images: ResMut<Assets<Image>>,
) {
    // We only care about the last resize event, as it's the most recent size.
    if let Some(event) = resize_events.read().last() {
        if let Some(image) = images.get_mut(&scene_preview_image.0) {
            // This re-allocates the texture on the GPU with the new size.
            // The camera's RenderTarget will automatically use the new size.
            image.resize(event.size);
            // We also need to re-register the image with egui's texture manager
            // This part is often missed, but crucial!
            // However, `bevy_egui` is smart enough to handle this automatically
            // when the asset is modified, so no extra code is needed here.
        }
    }
}

// This function will be used as a run condition
pub fn scene_view_is_focused(focus_state: Res<SceneViewFocus>) -> bool {
    focus_state.0
}

#[hot]
fn render_to_image_system(
    scene_preview_image: Res<ScenePreviewImage>,
    images: Res<Assets<Image>>,
    mut resize_writer: EventWriter<ResizeScenePreview>,
    mut focus_state: ResMut<SceneViewFocus>,
    mut contexts: EguiContexts,
) -> Result {
    let texture_id = contexts.image_id(&scene_preview_image).unwrap();
    egui::Window::new("Scene Preview").show(contexts.ctx_mut(), |ui| {
        let available_size = ui.available_size();
        let desired_size = egui::Vec2::new(available_size.x.max(1.0), available_size.y.max(1.0));

        // --- Resize logic remains the same ---
        if let Some(image) = images.get(&scene_preview_image.0) {
            let current_size = image.texture_descriptor.size;
            let new_size = Extent3d {
                width: desired_size.x as u32,
                height: desired_size.y as u32,
                depth_or_array_layers: 1,
            };

            if current_size != new_size {
                resize_writer.write(ResizeScenePreview { size: new_size });
            }
        }

        // --- Start of new logic for a focusable image ---

        // 1. Define the sense: we want both clicks and focus.
        let sense = egui::Sense::click().union(egui::Sense::FOCUSABLE);

        // 2. Allocate the space for our widget and make it interactive.
        // This replaces the ImageButton.
        let (rect, response) = ui.allocate_exact_size(desired_size, sense);

        // 3. Manually draw the image into the allocated rectangle.
        ui.painter().image(
            texture_id,
            rect,
            // The UV mapping of the texture (use the whole texture)
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            // Tint color
            egui::Color32::WHITE,
        );

        // 4. (Recommended) Add a visual indicator when the widget has focus.
        if response.has_focus() {
            use bevy_egui::egui::StrokeKind;

            ui.painter().rect_stroke(
                rect.expand(2.0), // Expand slightly for visibility
                0.0,              // No rounding
                ui.style().visuals.widgets.hovered.fg_stroke,
                StrokeKind::Inside,
            );
        }

        // --- End of new logic ---

        // Now you can check the response for focus and clicks.
        focus_state.0 = response.has_focus();

        if response.clicked() {
            // When clicked, explicitly request focus. This allows tabbing away and back.
            response.request_focus();
            println!("Scene view clicked!");
        }

        if response.gained_focus() {
            debug!("Gained focus!");
        }

        if response.lost_focus() {
            debug!("Lost focus!");
        }
    });
    Ok(())
}

#[derive(Component)]
pub struct RenderingCamera;

/// Resource that tracks all possible mouth textures
#[derive(Resource)]
pub struct MouthTextures {
    pub closed: Handle<Image>,
    pub open_small: Handle<Image>,
    pub open_big: Handle<Image>,
    // add more as needed
}

fn setup_system(
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // preload all mouth textures into a resource
    let textures = MouthTextures {
        closed: asset_server.load("mouth/3.png"),
        open_small: asset_server.load("mouth/o.png"),
        open_big: asset_server.load("mouth/O.png"),
    };

    commands.insert_resource(textures);

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("Scene Preview Texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    // fill image.data with zeroes
    image.resize(size);
    let image_handle = images.add(image);
    egui_user_textures.add_image(image_handle.clone());
    commands.insert_resource(ScenePreviewImage(image_handle.clone()));

    let rendering_camera = commands
        .spawn((
            Camera3d::default(),
            Camera {
                order: -1,
                hdr: true,
                target: RenderTarget::Image(image_handle.into()),
                clear_color: ClearColorConfig::Custom(Color::srgba(1.0, 1.0, 1.0, 0.0)),
                ..default()
            },
            Tonemapping::None,
            RenderingCamera,
            DepthPrepass,
            NormalPrepass,
            Transform::from_xyz(3., 1.5, -6.).looking_at(Vec3::ZERO, Vec3::Y),
            CameraController::default(),
            MoebiusPostProcessSettings { ..default() },
            Msaa::Off, // NormalPrepass, DepthPrepass
        ))
        .id();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            UiTargetCamera(rendering_camera),
        ))
        .with_children(|p| {
            // Spawn mouth overlay as PNG instead of colored box
            // let mouth_texture = asset_server.load("mouth.png");
            p.spawn((
                ImageNode::new(asset_server.load("mouth/3.png")),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
                MouthOverlay,
            ));
        });
}

fn customize_egui_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark(); // Use light theme as base

    // Borders & rounding
    visuals.widgets.active.bg_fill = adw_colors::DARK_2;
    visuals.widgets.hovered.bg_fill = adw_colors::DARK_3;
    visuals.widgets.hovered.expansion = 0.5;
    visuals.widgets.open.bg_fill = adw_colors::ACCENT_BG_COLOR;
    visuals.selection.bg_fill = adw_colors::ACCENT_BG_COLOR;
    visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(4);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(4); // For hovered scrollbar handle/track
    visuals.widgets.active.corner_radius = CornerRadius::same(4); // For active (dragged) scrollbar handle

    visuals.window_fill = adw_colors::WINDOW_BG_COLOR;
    visuals.window_stroke = Stroke::NONE;
    visuals.override_text_color = Some(adw_colors::WINDOW_FB_COLOR);
    visuals.window_corner_radius = CornerRadius::same(16);
    visuals.window_shadow.blur = 16;
    visuals.window_shadow.offset = [0, 1];
    visuals.faint_bg_color = adw_colors::VIEW_BG_COLOR;
    visuals.extreme_bg_color = adw_colors::VIEW_BG_COLOR;

    ctx.set_visuals(visuals);
}

#[hot]
fn ui_system(mut contexts: EguiContexts, mut state: ResMut<GuiState>) {
    let ctx = contexts.ctx_mut();

    // Only initialize fonts once
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let mut fonts = FontDefinitions::default();

        // Load custom font from assets
        let font_bytes = std::fs::read("assets/fonts/AdwaitaSans-Regular.ttf")
            .expect("Failed to read font file");

        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_owned(font_bytes).into(),
        );

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "my_font".to_owned());

        ctx.set_fonts(fonts);
        customize_egui_theme(ctx);
    });
    // egui::Window::Tit
    egui::Window::new(RichText::from("Settings").size(20.).strong())
        .frame(
            egui::Frame::new()
                .inner_margin(8.)
                .fill(adw_colors::WINDOW_BG_COLOR)
                .corner_radius(16.)
                .shadow(egui::Shadow {
                    offset: [0, 1],
                    blur: 16,
                    spread: 0,
                    color: adw_colors::SHADE_COLOR,
                }),
        )
        .show(contexts.ctx_mut(), |ui| {
            let style = ui.style_mut();

            style.spacing.scroll.bar_width = 8.;
            style.spacing.scroll.floating_width = 4.;
            // style.scrollbar.
            style.text_styles = [
                (TextStyle::Small, FontId::new(9.0, FontFamily::Proportional)),
                (TextStyle::Body, FontId::new(12.5, FontFamily::Proportional)),
                (
                    TextStyle::Button,
                    FontId::new(14., FontFamily::Proportional),
                ),
                (
                    TextStyle::Heading,
                    FontId::new(20.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Monospace,
                    FontId::new(12.0, FontFamily::Monospace),
                ),
            ]
            .into();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::default()
                        .inner_margin(16.0) // uniform padding
                        .show(ui, |ui| {
                            let style = ui.style_mut();
                            style.spacing.item_spacing = [16., 16.].into();

                            let mut s = state.as_mut().to_dynamic_struct();

                            // Iterate over each field in the struct
                            for i in 0..s.field_len() {
                                // Get the name of the field (e.g., "show_grid")

                                use crate::ui::state::Separator;
                                let field_name =
                                    s.name_at(i).unwrap_or("Unnamed Field").to_string();
                                let title_name = to_title_case(&field_name);
                                let attrs = s
                                    .get_represented_type_info()
                                    .unwrap()
                                    .as_struct()
                                    .unwrap()
                                    .field_at(i)
                                    .unwrap()
                                    .custom_attributes();
                                // Get a mutable reference to the field's value
                                let field_value = s.field_at_mut(i).unwrap();

                                if let Some(_) = attrs.get::<Separator>() {
                                    ui.separator();
                                }

                                ui.horizontal(|ui| {
                                    if let Some(value) = field_value.try_downcast_mut::<bool>() {
                                        label_toggle(ui, &title_name, value);
                                    }
                                    // If it's an f32, we'll show a slider.
                                    else if let Some(value) =
                                        field_value.try_downcast_mut::<f32>()
                                    {
                                        use crate::ui::state::SliderRange;

                                        let range = if let Some(range) = attrs.get::<SliderRange>()
                                        {
                                            range.0..=range.1
                                        } else {
                                            -1.0..=1.0
                                        };
                                        label_slider(ui, &title_name, value, range);
                                    }
                                    // You could add more types here (e.g., for strings, enums, etc.)
                                    else {
                                        ui.label(format!("{}: (Unsupported Type)", field_name));
                                    }
                                });
                            }

                            if let Some(updated_state) = GuiState::from_reflect(&s) {
                                *state = updated_state;
                            }

                            ui.separator();

                            ui.label(format!(
                                "\
Freecam Controls:\n\
    Mouse       - Move camera orientation\n\
    Scroll      - Adjust movement speed\n\
    Left        - Hold to grab cursor\n\
    KeyM        - Toggle cursor grab\n\
    KeyW & KeyS - Fly forward & backwards\n\
    KeyA & KeyD - Fly sideways left & right\n\
    KeyE & KeyQ - Fly up & down\n\
    ShiftLeft   - Fly faster while held\n\
        "
                            ))
                        })
                });
        });
}

fn label_toggle(ui: &mut egui::Ui, label: &str, value: &mut bool) {
    ui.horizontal(|ui| {
        ui.set_min_height(32.0);
        // ui.label(label);
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.label(label);
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(toggle(value));
        });
    });
}

fn label_slider(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) {
    ui.horizontal(|ui| {
        ui.set_min_height(32.0);
        let label_text_content = format!("{}: {:.2}", label, value);
        let fixed_label_width = 140.0; // Adjust this width as needed
        let label_height = 32.0; // Match the row's min_height

        ui.allocate_ui_with_layout(
            egui::vec2(fixed_label_width, label_height),
            egui::Layout::left_to_right(egui::Align::Center), // Original layout for the label
            |ui_for_label| {
                ui_for_label.label(label_text_content);
            },
        );

        ui.add_space(8.);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.);
            ui.add(AdwSlider::new(value, range));
        });
    });
}
fn to_title_case(s: &str) -> String {
    s.replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
