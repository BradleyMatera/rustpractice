use gpui::*;

use super::theme::{ALL_PNG_ASSETS, PANEL_ACTIVE_BG};

pub(crate) fn mono_icon(path: &'static str, size: f32, _color: u32) -> impl IntoElement {
    img(path).w(px(size)).h(px(size))
}

pub(crate) fn build_asset_gallery() -> impl IntoElement {
    let mut gallery = div().flex().flex_wrap().gap_2();
    for path in ALL_PNG_ASSETS {
        let is_brand = path.starts_with("brand/");
        let is_avatar = path.starts_with("ui/avatars/");
        let is_mock = path.starts_with("mock/");
        let is_crab_art = path.contains("crab-art");
        let is_wide_preview =
            path.contains("wordmark") || path.starts_with("mock/") || path.starts_with("ui/illustrations/");
        let is_ui_icon = path.starts_with("ui/icons/") || path.starts_with("ui/badges/");

        let (tile_w, tile_h, icon_w, icon_h) = if path.contains("crab-art") {
            (66.0, 66.0, 56.0, 56.0)
        } else if is_avatar {
            (38.0, 38.0, 24.0, 24.0)
        } else if is_mock {
            (74.0, 46.0, 68.0, 40.0)
        } else if path.contains("illustrations/") || path.contains("wordmark") {
            (62.0, 38.0, 56.0, 32.0)
        } else {
            (34.0, 34.0, 18.0, 18.0)
        };
        let preview_bg = if is_crab_art || is_wide_preview {
            0x173E5A
        } else if is_ui_icon {
            PANEL_ACTIVE_BG
        } else if is_brand {
            0x173A56
        } else {
            0x12324A
        };
        let preview_border = 0x2D709A;
        gallery = gallery.child(
            div()
                .w(px(tile_w))
                .h(px(tile_h))
                .rounded_sm()
                .bg(rgb(0x0D2B42))
                .border_1()
                .border_color(rgb(preview_border))
                .p_1()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .w_full()
                        .h_full()
                        .rounded_sm()
                        .bg(rgb(preview_bg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(img(path).w(px(icon_w)).h(px(icon_h))),
                ),
        );
    }
    gallery
}
