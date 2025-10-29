#[allow(dead_code)]
pub mod adw_colors {
    use bevy_egui::egui::Color32;

    pub const BLUE_1: Color32 = Color32::from_rgb(153, 193, 241);
    pub const BLUE_2: Color32 = Color32::from_rgb(98, 160, 234);
    pub const BLUE_3: Color32 = Color32::from_rgb(53, 132, 228);
    pub const BLUE_4: Color32 = Color32::from_rgb(28, 113, 216);
    pub const BLUE_5: Color32 = Color32::from_rgb(26, 95, 180);

    pub const GREEN_1: Color32 = Color32::from_rgb(143, 240, 164);
    pub const GREEN_2: Color32 = Color32::from_rgb(87, 227, 137);
    pub const GREEN_3: Color32 = Color32::from_rgb(51, 209, 122);
    pub const GREEN_4: Color32 = Color32::from_rgb(46, 194, 126);
    pub const GREEN_5: Color32 = Color32::from_rgb(38, 162, 105);

    pub const YELLOW_1: Color32 = Color32::from_rgb(249, 240, 107);
    pub const YELLOW_2: Color32 = Color32::from_rgb(248, 228, 92);
    pub const YELLOW_3: Color32 = Color32::from_rgb(246, 211, 45);
    pub const YELLOW_4: Color32 = Color32::from_rgb(245, 194, 17);
    pub const YELLOW_5: Color32 = Color32::from_rgb(229, 165, 10);

    pub const ORANGE_1: Color32 = Color32::from_rgb(255, 190, 111);
    pub const ORANGE_2: Color32 = Color32::from_rgb(255, 163, 72);
    pub const ORANGE_3: Color32 = Color32::from_rgb(255, 120, 0);
    pub const ORANGE_4: Color32 = Color32::from_rgb(230, 97, 0);
    pub const ORANGE_5: Color32 = Color32::from_rgb(198, 70, 0);

    pub const RED_1: Color32 = Color32::from_rgb(246, 97, 81);
    pub const RED_2: Color32 = Color32::from_rgb(237, 51, 59);
    pub const RED_3: Color32 = Color32::from_rgb(224, 27, 36);
    pub const RED_4: Color32 = Color32::from_rgb(192, 28, 40);
    pub const RED_5: Color32 = Color32::from_rgb(165, 29, 45);

    pub const PURPLE_1: Color32 = Color32::from_rgb(220, 138, 221);
    pub const PURPLE_2: Color32 = Color32::from_rgb(192, 97, 203);
    pub const PURPLE_3: Color32 = Color32::from_rgb(145, 65, 172);
    pub const PURPLE_4: Color32 = Color32::from_rgb(129, 61, 156);
    pub const PURPLE_5: Color32 = Color32::from_rgb(97, 53, 131);

    pub const BROWN_1: Color32 = Color32::from_rgb(205, 171, 143);
    pub const BROWN_2: Color32 = Color32::from_rgb(181, 131, 90);
    pub const BROWN_3: Color32 = Color32::from_rgb(152, 106, 68);
    pub const BROWN_4: Color32 = Color32::from_rgb(134, 94, 60);
    pub const BROWN_5: Color32 = Color32::from_rgb(99, 69, 44);

    pub const LIGHT_1: Color32 = Color32::from_rgb(255, 255, 255);
    pub const LIGHT_2: Color32 = Color32::from_rgb(246, 245, 244);
    pub const LIGHT_3: Color32 = Color32::from_rgb(222, 221, 218);
    pub const LIGHT_4: Color32 = Color32::from_rgb(192, 191, 188);
    pub const LIGHT_5: Color32 = Color32::from_rgb(154, 153, 150);

    pub const DARK_1: Color32 = Color32::from_rgb(119, 118, 123);
    pub const DARK_2: Color32 = Color32::from_rgb(94, 92, 100);
    pub const DARK_3: Color32 = Color32::from_rgb(61, 56, 70);
    pub const DARK_4: Color32 = Color32::from_rgb(36, 31, 49);
    pub const DARK_5: Color32 = Color32::from_rgb(0, 0, 0);

    pub const WINDOW_BG_COLOR: Color32 = Color32::from_rgb(36, 36, 36);
    pub const WINDOW_FB_COLOR: Color32 = Color32::WHITE;

    pub const VIEW_BG_COLOR: Color32 = Color32::from_rgb(30, 30, 30);
    pub const VIEW_FB_COLOR: Color32 = Color32::WHITE;

    // Accent Colors
    pub const ACCENT_COLOR: Color32 = Color32::from_rgb(120, 174, 237);
    pub const ACCENT_BG_COLOR: Color32 = self::BLUE_3;
    pub const ACCENT_FG_COLOR: Color32 = Color32::WHITE;

    pub const SHADE_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 64);

    // Card Colors (Dark)
    pub const CARD_BG_COLOR: Color32 = Color32::from_rgb(62, 62, 66);
    pub const CARD_FG_COLOR: Color32 = Color32::WHITE;
    pub const CARD_SHADE_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 92); // rgba(0,0,0,0.36)

    // Thumbnail Colors (Dark)
    pub const THUMBNAIL_BG_COLOR: Color32 = Color32::from_rgb(56, 56, 56); // #383838
    pub const THUMBNAIL_FG_COLOR: Color32 = Color32::WHITE;

    // Dialog Colors (Dark)
    pub const DIALOG_BG_COLOR: Color32 = Color32::from_rgb(56, 56, 56); // #383838
    pub const DIALOG_FG_COLOR: Color32 = Color32::WHITE;

    // Popover Colors (Dark)
    pub const POPOVER_BG_COLOR: Color32 = Color32::from_rgb(56, 56, 56); // #383838
    pub const POPOVER_FG_COLOR: Color32 = Color32::WHITE;
    pub const POPOVER_SHADE_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 64); // rgba(0,0,0,0.25)
}
