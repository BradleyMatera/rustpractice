use gpui::*;
use gpui::StatefulInteractiveElement;
use gpui::prelude::*;

use crate::ui::elements::{mono_icon, rounded_image};
use crate::ui::theme::{
    ACCENT, ACCENT_SOFT, ASSET_ICON_ACTION_ADD, ASSET_ICON_ACTION_SEND, ASSET_ICON_CHANNEL_TEXT,
    ASSET_ICON_NAV_SEARCH, CHAT_BG, INPUT_BG, PANEL_ACTIVE_BG, PANEL_ALT_BG, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};

use super::CrabCordShell;

pub(super) fn build_chat_panel(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> AnyElement {
    div()
        .w_full()
        .h_full()
        .bg(rgb(CHAT_BG))
        .rounded_md()
        .flex()
        .flex_col()
        .child(build_chat_header(shell))
        .child(build_chat_timeline(shell))
        .child(build_chat_input(shell, cx))
        .into_any_element()
}

fn build_chat_header(shell: &CrabCordShell) -> impl IntoElement {
    let active_name = shell.active_name();
    let active_topic = shell.active_topic();
    let lane_label = if shell.is_dm(shell.active_conversation) {
        format!("@{}", active_name)
    } else {
        format!("#{}", active_name)
    };
    div()
        .h(px(52.0))
        .px_4()
        .bg(rgb(PANEL_ALT_BG))
        .border_b_1()
        .border_color(rgb(0x225575))
        .flex()
        .items_center()
        .gap_2()
        .child(mono_icon(ASSET_ICON_CHANNEL_TEXT, 15.0, ACCENT_SOFT))
        .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(lane_label))
        .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(active_topic))
        .child(div().w_full())
        .child(mono_icon("ui/icons/actions/video.png", 14.0, TEXT_MUTED))
        .child(mono_icon("ui/icons/actions/pin.png", 14.0, TEXT_MUTED))
        .child(mono_icon("ui/icons/navigation/notifications.png", 14.0, TEXT_MUTED))
        .child(
            div()
                .w(px(230.0))
                .h(px(30.0))
                .rounded_md()
                .bg(rgb(CHAT_BG))
                .px_2()
                .flex()
                .items_center()
                .gap_2()
                .child(mono_icon(ASSET_ICON_NAV_SEARCH, 14.0, TEXT_MUTED))
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child(format!("Search {}", active_name)),
                ),
        )
}

fn build_chat_timeline(shell: &CrabCordShell) -> impl IntoElement {
    let mut timeline = div()
        .h_full()
        .px_4()
        .py_3()
        .id("timeline-scroll")
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .gap_3();

    for message in shell.visible_messages() {
        let author_line = if message.is_bot {
            format!("{}  BOT   {}", message.author, message.time)
        } else {
            format!("{}   {}", message.author, message.time)
        };
        timeline = timeline.child(
            div()
                .rounded_md()
                .px_2()
                .py_1()
                .flex()
                .gap_3()
                .child(rounded_image(message.avatar_png, 38.0))
                .child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child(author_line),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY))
                                .child(message.body.clone()),
                        )
                        .when(message.body.contains("http"), |thread| {
                            thread.child(
                                div()
                                    .rounded_md()
                                    .bg(rgb(PANEL_ALT_BG))
                                    .border_1()
                                    .border_color(rgb(0x2A607F))
                                    .px_3()
                                    .py_2()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child("Link Preview"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child("Axiom docs and backend actor model notes"),
                                    ),
                            )
                        }),
                ),
        );
    }

    timeline.child(
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
                    .child("Keep interaction obvious and lanes readable."),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child("Fake backend data now, real actor backend next."),
            ),
    )
}

fn build_chat_input(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> impl IntoElement {
    div()
        .h(px(64.0))
        .mx_4()
        .mb_4()
        .rounded_md()
        .bg(rgb(INPUT_BG))
        .px_3()
        .flex()
        .items_center()
        .gap_2()
        .child(mono_icon(ASSET_ICON_ACTION_ADD, 16.0, ACCENT_SOFT))
        .child(
            div()
                .w_full()
                .text_sm()
                .text_color(rgb(TEXT_MUTED))
                .child(shell.active_placeholder()),
        )
        .child(mono_icon("ui/icons/actions/gif.png", 16.0, TEXT_MUTED))
        .child(mono_icon("ui/icons/actions/emoji.png", 16.0, TEXT_MUTED))
        .child(
            div()
                .id("send-message")
                .h(px(36.0))
                .px_4()
                .rounded_md()
                .bg(rgb(ACCENT))
                .cursor_pointer()
                .flex()
                .items_center()
                .gap_1()
                .text_sm()
                .text_color(rgb(TEXT_PRIMARY))
                .child(mono_icon(ASSET_ICON_ACTION_SEND, 14.0, TEXT_PRIMARY))
                .child("Send")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.send_quick_message(cx);
                })),
        )
        .when(shell.active_dm().is_some(), |row| {
            row.child(
                div()
                    .h(px(24.0))
                    .px_2()
                    .rounded_md()
                    .bg(rgb(PANEL_ACTIVE_BG))
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .flex()
                    .items_center()
                    .child("DM"),
            )
        })
}
