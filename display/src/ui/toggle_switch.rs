use bevy_egui::egui;
use egui::{Pos2, Rect, Sense, TextStyle, Vec2, WidgetText};

use super::theme::adw_colors;

pub struct ToggleSwitch<'a> {
    on: &'a mut bool,
    text: Option<WidgetText>,
}

impl<'a> ToggleSwitch<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self { on, text: None }
    }
}

impl<'a> egui::Widget for ToggleSwitch<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (label_galley, label_size) = if let Some(widget_text) = self.text {
            let galley = widget_text.into_galley(
                ui,
                Some(egui::TextWrapMode::Wrap), // No wrap for a simple label next to a switch
                ui.available_width(),           // Max width for galley, though it should be small
                TextStyle::Button,              // Use button text style for consistency
            );
            let size = galley.size();
            (Some(galley), size)
        } else {
            (None, Vec2::ZERO)
        };

        // 2. Define Switch Size (based on original logic)
        let switch_height_factor = 1.4;
        let switch_width_factor = 2.5;
        let switch_height = ui.spacing().interact_size.y * switch_height_factor;
        let switch_width = ui.spacing().interact_size.y * switch_width_factor;
        let switch_visual_size = Vec2::new(switch_width, switch_height);

        // 3. Calculate Total Desired Size for the combined widget
        let spacing_x = if label_size.x > 0.0 {
            ui.spacing().item_spacing.x
        } else {
            0.0
        };
        let total_desired_width = label_size.x + spacing_x + switch_visual_size.x;
        let total_desired_height = label_size.y.max(switch_visual_size.y);
        let total_desired_size = Vec2::new(total_desired_width, total_desired_height);

        // 4. Allocate Space for the whole widget
        let (full_rect, mut response) = ui.allocate_exact_size(total_desired_size, Sense::click());

        // 5. Handle Interaction
        if response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }

        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Checkbox,
                ui.is_enabled(),
                *self.on,
                label_galley
                    .as_ref()
                    .map_or_else(String::new, |g| g.text().to_owned()),
            )
        });

        // 6. Paint
        if ui.is_rect_visible(full_rect) {
            let mut visuals = ui.style().interact_selectable(&response, *self.on);
            visuals.fg_stroke = egui::Stroke::NONE;
            visuals.bg_stroke = egui::Stroke::NONE;

            let track_color: egui::Color32 = if response.hovered() {
                if *self.on {
                    adw_colors::BLUE_2
                } else {
                    adw_colors::DARK_2
                }
            } else {
                visuals.bg_fill // Default background fill
            };

            // Paint Label
            if let Some(ref galley) = label_galley {
                if label_size.x > 0.0 {
                    let label_pos_y = full_rect.top() + (total_desired_height - label_size.y) / 2.0;
                    let label_pos = Pos2::new(full_rect.left(), label_pos_y);
                    ui.painter()
                        .galley(label_pos, galley.clone(), visuals.text_color());
                }
            }

            // Paint Switch
            let switch_pos_x = full_rect.left() + label_size.x + spacing_x;
            let switch_pos_y =
                full_rect.top() + (total_desired_height - switch_visual_size.y) / 2.0;
            let switch_rect =
                Rect::from_min_size(Pos2::new(switch_pos_x, switch_pos_y), switch_visual_size);

            let switch_paint_rect = switch_rect.expand(visuals.expansion);
            let radius = 0.5 * switch_paint_rect.height();

            // Paint the track
            ui.painter().rect(
                switch_paint_rect,
                radius,
                track_color,
                egui::Stroke::NONE,
                egui::StrokeKind::Inside,
            );

            // Paint the knob
            let how_on = ui.ctx().animate_bool_responsive(response.id, *self.on);
            let circle_x = egui::lerp(
                (switch_paint_rect.left() + radius)..=(switch_paint_rect.right() - radius),
                how_on,
            );
            let center = Pos2::new(circle_x, switch_paint_rect.center().y);
            ui.painter().circle(
                center,
                0.75 * radius,               // Knob slightly smaller than track height
                adw_colors::ACCENT_FG_COLOR, // Fill with foreground color
                egui::Stroke::NONE,          // No stroke for the knob
            );
        }
        response
    }
}

impl ToggleSwitch<'_> {
    pub fn text(mut self, text: impl Into<WidgetText>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl ToggleSwitch<'_> {
    pub fn on(self) -> Self {
        *self.on = true;
        self
    }
    pub fn is_on(&self) -> bool {
        *self.on
    }
}

pub fn toggle(on: &mut bool) -> ToggleSwitch<'_> {
    ToggleSwitch::new(on)
}
