use gpui::*;

use super::theme::{
    ACCENT, ACCENT_SOFT, ALL_SVG_ASSETS, ASSET_BRAND_CRAB_PNG_56, PANEL_ACTIVE_BG, PANEL_ALT_BG,
    SUCCESS,
};

pub(crate) fn mono_icon(path: &'static str, size: f32, color: u32) -> impl IntoElement {
    svg()
        .path(path)
        .w(px(size))
        .h(px(size))
        .text_color(rgb(color))
}

pub(crate) fn build_svg_gallery() -> impl IntoElement {
    let mut gallery = div().flex().flex_wrap().gap_2();
    for path in ALL_SVG_ASSETS {
        let is_brand = path.starts_with("brand/");
        let is_ui_icon = path.starts_with("ui/icons/") || path.starts_with("ui/badges/");
        let is_status_icon = path.starts_with("ui/icons/status/");
        let is_avatar = path.starts_with("ui/avatars/");
        let is_crab_art = path.contains("crab-art");
        let is_mock = path.starts_with("mock/");
        let is_wide_preview =
            path.contains("wordmark") || path.starts_with("mock/") || path.starts_with("ui/illustrations/");
        let is_brand_or_mock =
            path.starts_with("brand/") || path.starts_with("mock/") || path.starts_with("ui/illustrations/");

        let (tile_w, tile_h, icon_w, icon_h) = if is_crab_art {
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
            0xE7F1F8
        } else if is_brand {
            PANEL_ALT_BG
        } else if is_ui_icon {
            PANEL_ACTIVE_BG
        } else {
            0x0B2438
        };
        let preview_border = if is_crab_art || is_wide_preview {
            0x90B5CB
        } else {
            0x2D709A
        };
        let mut icon = svg().path(path).w(px(icon_w)).h(px(icon_h));
        if is_status_icon {
            icon = icon.text_color(rgb(SUCCESS));
        } else if is_ui_icon {
            icon = icon.text_color(rgb(0xEAF6FF));
        } else if is_crab_art {
            // Keep original multicolor company art untouched.
        } else if is_avatar {
            icon = icon.text_color(rgb(ACCENT_SOFT));
        } else if is_brand {
            icon = icon.text_color(rgb(ACCENT_SOFT));
        } else if is_mock {
            icon = icon.text_color(rgb(ACCENT));
        } else if is_brand_or_mock {
            icon = icon.text_color(rgb(0x1D4D6E));
        } else {
            icon = icon.text_color(rgb(0xBCEBFF));
        }
        let content = if is_crab_art {
            img(ASSET_BRAND_CRAB_PNG_56)
                .w(px(icon_w))
                .h(px(icon_h))
                .into_any_element()
        } else {
            icon.into_any_element()
        };
        gallery = gallery.child(
            div()
                .w(px(tile_w))
                .h(px(tile_h))
                .rounded_sm()
                .bg(rgb(preview_bg))
                .border_1()
                .border_color(rgb(preview_border))
                .flex()
                .items_center()
                .justify_center()
                .child(content),
        );
    }
    gallery
}
