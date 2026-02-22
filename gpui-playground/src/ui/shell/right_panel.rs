use gpui::*;
use gpui::StatefulInteractiveElement;

use crate::ui::elements::{build_svg_gallery, mono_icon};
use crate::ui::theme::{
    ACCENT, ACCENT_ALT, ACCENT_SOFT, ALL_SVG_ASSETS, ASSET_BADGE_BOT, ASSET_BRAND_MASCOT_56_SVG,
    ASSET_ICON_ACTION_INVITE, ASSET_ICON_NAV_DISCOVER, ASSET_ICON_STATUS_ONLINE, MEMBERS_BG,
    PANEL_ACTIVE_BG, PANEL_ALT_BG, SUCCESS, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

use super::CrabCordShell;

pub(super) fn build_right_panel(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> AnyElement {
    let right_panel_width = if shell.show_asset_desk { 420.0 } else { 304.0 };
    let asset_toggle_label = if shell.show_asset_desk {
        "Back to Crew"
    } else {
        "Open Asset Desk"
    };
    let members_title = format!("CREW — {}", shell.online_count);
    let svg_library_title = format!("SVG LIBRARY ({})", ALL_SVG_ASSETS.len());

    if shell.show_asset_desk {
        div()
            .w(px(right_panel_width))
            .h_full()
            .bg(rgb(MEMBERS_BG))
            .overflow_hidden()
            .text_color(rgb(TEXT_SECONDARY))
            .px_4()
            .py_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("ASSET DESK"))
            .child(
                div()
                    .id("toggle-asset-desk")
                    .h(px(36.0))
                    .rounded_md()
                    .bg(rgb(PANEL_ALT_BG))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(mono_icon(ASSET_ICON_NAV_DISCOVER, 14.0, ACCENT_SOFT))
                    .child(asset_toggle_label)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_asset_desk(cx);
                    })),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(TEXT_MUTED))
                    .child(svg_library_title),
            )
            .child(
                div()
                    .id("asset-desk-scroll")
                    .h_full()
                    .pr_1()
                    .overflow_y_scroll()
                    .child(build_svg_gallery()),
            )
            .into_any_element()
    } else {
        div()
            .w(px(right_panel_width))
            .h_full()
            .bg(rgb(MEMBERS_BG))
            .overflow_hidden()
            .text_color(rgb(TEXT_SECONDARY))
            .px_4()
            .py_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(TEXT_MUTED))
                    .child(members_title),
            )
            .child(
                div()
                    .id("toggle-asset-desk")
                    .h(px(36.0))
                    .rounded_md()
                    .bg(rgb(PANEL_ALT_BG))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(mono_icon(ASSET_ICON_NAV_DISCOVER, 14.0, ACCENT_SOFT))
                    .child(asset_toggle_label)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_asset_desk(cx);
                    })),
            )
            .child(
                div()
                    .h(px(42.0))
                    .rounded_md()
                    .bg(rgb(PANEL_ALT_BG))
                    .px_3()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        svg()
                            .path(ASSET_ICON_STATUS_ONLINE)
                            .w(px(12.0))
                            .h(px(12.0))
                            .text_color(rgb(SUCCESS)),
                    )
                    .child(
                        svg()
                            .path(ASSET_BRAND_MASCOT_56_SVG)
                            .w(px(22.0))
                            .h(px(22.0))
                            .text_color(rgb(ACCENT_ALT)),
                    )
                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("brad")),
            )
            .child(
                div()
                    .h(px(42.0))
                    .rounded_md()
                    .bg(rgb(PANEL_ALT_BG))
                    .px_3()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(mono_icon(ASSET_ICON_STATUS_ONLINE, 12.0, SUCCESS))
                    .child(
                        div()
                            .w(px(22.0))
                            .h(px(22.0))
                            .rounded_sm()
                            .bg(rgb(PANEL_ACTIVE_BG))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path(ASSET_BADGE_BOT)
                                    .w(px(14.0))
                                    .h(px(14.0))
                                    .text_color(rgb(ACCENT_SOFT)),
                            ),
                    )
                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("design-bot"))
                    .child(
                        div()
                            .px_1()
                            .rounded_sm()
                            .bg(rgb(0x355672))
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .child("bot"),
                    ),
            )
            .child(
                div()
                    .h(px(42.0))
                    .rounded_md()
                    .bg(rgb(PANEL_ALT_BG))
                    .px_3()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(mono_icon(ASSET_ICON_STATUS_ONLINE, 12.0, SUCCESS))
                    .child(
                        svg()
                            .path(ASSET_BRAND_MASCOT_56_SVG)
                            .w(px(22.0))
                            .h(px(22.0))
                            .text_color(rgb(ACCENT)),
                    )
                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("rustacean")),
            )
            .child(div().h_full())
            .child(
                div()
                    .id("invite-member")
                    .h(px(34.0))
                    .rounded_md()
                    .bg(rgb(ACCENT))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_1()
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(mono_icon(ASSET_ICON_ACTION_INVITE, 14.0, TEXT_PRIMARY))
                    .child("Invite")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.invite_member(cx);
                    })),
            )
            .into_any_element()
    }
}
