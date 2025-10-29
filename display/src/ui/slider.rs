use bevy_egui::egui::{self, *};

use crate::ui::theme::adw_colors;

/// A custom slider widget
pub struct AdwSlider<'a> {
    value: &'a mut f32,
    range: std::ops::RangeInclusive<f32>,
}

impl<'a> AdwSlider<'a> {
    pub fn new(value: &'a mut f32, range: std::ops::RangeInclusive<f32>) -> Self {
        Self { value, range }
    }
}

impl<'a> Widget for AdwSlider<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = vec2(ui.available_width(), 32.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        let hovered = response.hovered();
        let active = response.dragged();

        let range = self.range;
        // let normalized = (*self.value - *range.start()) / (range.end() - range.start());
        let normalized = (*self.value - *range.start()) / (*range.end() - *range.start());

        let visuals = ui.style().interact(&response);
        let painter = ui.painter();

        let track_height = 4.0;
        let track_radius = track_height / 2.0;

        // Draw track
        let track_rect = Rect::from_center_size(rect.center(), vec2(rect.width(), track_height));
        let filled_x = remap_clamp(normalized, 0.0..=1.0, rect.left()..=rect.right());

        // Colors
        let filled_color = if hovered {
            adw_colors::BLUE_2
        } else {
            adw_colors::BLUE_3
        };
        let circle_color = Color32::WHITE;
        // Draw filled portion
        let filled_rect = Rect::from_min_max(
            pos2(track_rect.left(), track_rect.top()),
            pos2(filled_x, track_rect.bottom()),
        );
        painter.rect_filled(filled_rect, track_radius, filled_color);

        // Colors
        let filled_track_bg = if hovered {
            adw_colors::DARK_1
        } else {
            visuals.bg_fill
        };
        // Draw remaining track
        let unfilled_rect = Rect::from_min_max(
            pos2(filled_x, track_rect.top()),
            pos2(track_rect.right(), track_rect.bottom()),
        );
        painter.rect_filled(unfilled_rect, track_radius, filled_track_bg);

        // Circle (thumb)
        let circle_radius = 10.0;
        let x = remap_clamp(normalized, 0.0..=1.0, rect.left()..=rect.right());
        let center = pos2(x, rect.center().y);

        // Hover effect
        let final_circle_color = if hovered || active {
            circle_color.linear_multiply(1.2)
        } else {
            circle_color
        };

        // Draw thumb
        painter.circle_filled(center, circle_radius, final_circle_color);

        if response.dragged() {
            let mouse_x = ui.input(|i| i.pointer.interact_pos()).map(|p| p.x);
            if let Some(mouse_x) = mouse_x {
                let t = remap_clamp(mouse_x, rect.left()..=rect.right(), 0.0..=1.0);
                *self.value = lerp((*range.start())..=(*range.end()), t);
            }
        }

        response
    }
}
