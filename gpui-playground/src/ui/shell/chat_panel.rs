use gpui::*;
use gpui::StatefulInteractiveElement;

use crate::ui::elements::mono_icon;
use crate::ui::theme::{
    ACCENT, ACCENT_ALT, ACCENT_SOFT, ASSET_BADGE_BOT, ASSET_BRAND_MASCOT_56_SVG, ASSET_ICON_ACTION_ADD,
    ASSET_ICON_ACTION_SEND, ASSET_ICON_CHANNEL_TEXT, ASSET_ICON_CHANNEL_VOICE, ASSET_ICON_NAV_SEARCH,
    CHAT_BG, INPUT_BG, PANEL_ALT_BG, STATUS_LINES, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

use super::CrabCordShell;

pub(super) fn build_chat_panel(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> AnyElement {
    div()
        .w_full()
        .h_full()
        .bg(rgb(CHAT_BG))
        .flex()
        .flex_col()
        .child(build_chat_header(shell))
        .child(build_chat_body(shell, cx))
        .into_any_element()
}

fn build_chat_header(shell: &CrabCordShell) -> impl IntoElement {
    div()
        .h(px(56.0))
        .px_5()
        .bg(rgb(PANEL_ALT_BG))
        .text_color(rgb(TEXT_SECONDARY))
        .flex()
        .items_center()
        .child(
            div()
                .w_full()
                .flex()
                .items_center()
                .gap_2()
                .child(mono_icon(ASSET_ICON_CHANNEL_TEXT, 16.0, ACCENT_SOFT))
                .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("general"))
                .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("CrabCord chat")),
        )
        .child(mono_icon(ASSET_ICON_NAV_SEARCH, 16.0, ACCENT_ALT))
        .child(
            div()
                .px_3()
                .h(px(28.0))
                .rounded_md()
                .bg(rgb(CHAT_BG))
                .flex()
                .items_center()
                .justify_center()
                .gap_1()
                .text_sm()
                .text_color(rgb(TEXT_MUTED))
                .child(mono_icon(ASSET_ICON_CHANNEL_VOICE, 12.0, ACCENT_SOFT))
                .child(STATUS_LINES[shell.status_index]),
        )
}

fn build_chat_body(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> impl IntoElement {
    div()
        .w_full()
        .h_full()
        .px_5()
        .py_5()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .flex()
                .gap_3()
                .child(
                    svg()
                        .path(ASSET_BRAND_MASCOT_56_SVG)
                        .w(px(44.0))
                        .h(px(44.0))
                        .text_color(rgb(ACCENT_SOFT)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("brad  Today at 10:42"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY))
                                .child("This is the first clean CrabCord layout pass in GPUI."),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .gap_3()
                .child(
                    div()
                        .w(px(44.0))
                        .h(px(44.0))
                        .rounded_md()
                        .bg(rgb(PANEL_ALT_BG))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            svg()
                                .path(ASSET_BADGE_BOT)
                                .w(px(26.0))
                                .h(px(26.0))
                                .text_color(rgb(ACCENT_ALT)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("design-bot  Today at 10:43"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY))
                                .child("Keep this pass bold, fast, and unmistakably CrabCord."),
                        )
                        .child(
                            div()
                                .bg(rgb(PANEL_ALT_BG))
                                .rounded_md()
                                .px_3()
                                .py_2()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .child(STATUS_LINES[shell.status_index]),
                                ),
                        ),
                ),
        )
        .child(
            div()
                .rounded_md()
                .bg(rgb(PANEL_ALT_BG))
                .border_1()
                .border_color(rgb(0x235A7A))
                .px_4()
                .py_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("Build focus"))
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child("Keep chat first, keep asset desk separate."),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_SECONDARY))
                        .child("No clutter in the main channel lane."),
                ),
        )
        .child(
            div()
                .h_full()
                .rounded_md()
                .bg(rgb(0x0E2B44))
                .border_1()
                .border_color(rgb(0x1E4E6E))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .rounded_md()
                        .bg(rgb(PANEL_ALT_BG))
                        .px_4()
                        .py_2()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(mono_icon(ASSET_ICON_ACTION_ADD, 14.0, ACCENT_SOFT))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child("Start a message, drop a file, or paste a link"),
                        ),
                ),
        )
        .child(
            div()
                .h(px(64.0))
                .bg(rgb(INPUT_BG))
                .rounded_md()
                .px_3()
                .flex()
                .items_center()
                .child(
                    div()
                        .w_full()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child("Message #general"),
                )
                .child(
                    div()
                        .id("send-message")
                        .w(px(112.0))
                        .h(px(36.0))
                        .rounded_md()
                        .bg(rgb(ACCENT))
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .text_sm()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child(mono_icon(ASSET_ICON_ACTION_SEND, 14.0, ACCENT_SOFT))
                        .child("Send")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.cycle_status(cx);
                        })),
                ),
        )
}
