use gpui::*;
use gpui::StatefulInteractiveElement;

use super::elements::{build_svg_gallery, mono_icon};
use super::theme::*;

pub struct CrabCordShell {
    status_index: usize,
    muted: bool,
    online_count: usize,
    show_asset_desk: bool,
}

impl CrabCordShell {
    pub fn new() -> Self {
        Self {
            status_index: 0,
            muted: false,
            online_count: 7,
            show_asset_desk: false,
        }
    }

    fn cycle_status(&mut self, cx: &mut Context<Self>) {
        self.status_index = (self.status_index + 1) % STATUS_LINES.len();
        cx.notify();
    }

    fn toggle_mic(&mut self, cx: &mut Context<Self>) {
        self.muted = !self.muted;
        cx.notify();
    }

    fn invite_member(&mut self, cx: &mut Context<Self>) {
        self.online_count = (self.online_count + 1).min(99);
        cx.notify();
    }

    fn toggle_asset_desk(&mut self, cx: &mut Context<Self>) {
        self.show_asset_desk = !self.show_asset_desk;
        cx.notify();
    }
}

impl Render for CrabCordShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mic_state = if self.muted { "Muted" } else { "Live" };
        let mic_color = if self.muted { WARN } else { SUCCESS };
        let mic_icon = if self.muted {
            ASSET_ICON_ACTION_MUTE
        } else {
            ASSET_ICON_ACTION_UNMUTE
        };
        let members_title = format!("CREW — {}", self.online_count);
        let svg_library_title = format!("SVG LIBRARY ({})", ALL_SVG_ASSETS.len());

        let right_panel_width = if self.show_asset_desk { 420.0 } else { 304.0 };
        let asset_toggle_label = if self.show_asset_desk {
            "Back to Crew"
        } else {
            "Open Asset Desk"
        };

        let right_panel = if self.show_asset_desk {
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
        };

        div()
            .size_full()
            .bg(rgb(APP_BG))
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .w(px(1500.0))
                    .h(px(900.0))
                    .bg(rgb(SHELL_BG))
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(0x224B6A))
                    .flex()
                    .child(
                        div()
                            .w(px(88.0))
                            .h_full()
                            .bg(rgb(GUILD_RAIL_BG))
                            .rounded_md()
                            .text_color(rgb(TEXT_SECONDARY))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_3()
                            .py_4()
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(ACCENT))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        img(ASSET_BRAND_CRAB_PNG_56)
                                            .w(px(38.0))
                                            .h(px(38.0)),
                                    ),
                            )
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(PANEL_ALT_BG))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(mono_icon(ASSET_ICON_NAV_FRIENDS, 22.0, ACCENT_ALT)),
                            )
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(PANEL_ALT_BG))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(mono_icon(ASSET_ICON_NAV_DISCOVER, 22.0, ACCENT_SOFT)),
                            )
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(PANEL_ALT_BG))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(mono_icon(ASSET_ICON_ACTION_ADD, 22.0, ACCENT)),
                            ),
                    )
                    .child(
                        div()
                            .w(px(336.0))
                            .h_full()
                            .bg(rgb(CHANNELS_BG))
                            .text_color(rgb(TEXT_SECONDARY))
                            .px_4()
                            .py_4()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .h(px(48.0))
                                    .rounded_md()
                                    .bg(rgb(SHELL_BG))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        svg()
                                            .path(ASSET_BRAND_MASCOT_56_SVG)
                                            .w(px(18.0))
                                            .h(px(18.0))
                                            .text_color(rgb(ACCENT_SOFT)),
                                    )
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("CrabCord Dev"))
                                    .child(div().w_full())
                                    .child(mono_icon(ASSET_ICON_NAV_SETTINGS, 16.0, ACCENT_ALT)),
                            )
                            .child(
                                div()
                                    .h(px(86.0))
                                    .rounded_md()
                                    .bg(rgb(PANEL_ALT_BG))
                                    .border_1()
                                    .border_color(rgb(0x2A6D95))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        img(ASSET_BRAND_CRAB_PNG_96)
                                            .w(px(44.0))
                                            .h(px(44.0)),
                                    )
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
                                                    .child("CrabCord HQ"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_SECONDARY))
                                                    .child("Design lane for UI polish and fast iteration"),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .px_2()
                                                            .h(px(22.0))
                                                            .rounded_md()
                                                            .bg(rgb(PANEL_ACTIVE_BG))
                                                            .flex()
                                                            .items_center()
                                                            .gap_1()
                                                            .child(mono_icon(
                                                                ASSET_ICON_STATUS_ONLINE,
                                                                10.0,
                                                                SUCCESS,
                                                            ))
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .text_color(rgb(TEXT_PRIMARY))
                                                                    .child("7 online"),
                                                            ),
                                                    )
                                                    .child(
                                                        div()
                                                            .px_2()
                                                            .h(px(22.0))
                                                            .rounded_md()
                                                            .bg(rgb(PANEL_ACTIVE_BG))
                                                            .flex()
                                                            .items_center()
                                                            .gap_1()
                                                            .child(mono_icon(
                                                                ASSET_ICON_NAV_DISCOVER,
                                                                10.0,
                                                                ACCENT_SOFT,
                                                            ))
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .text_color(rgb(TEXT_PRIMARY))
                                                                    .child("asset desk ready"),
                                                            ),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .h(px(96.0))
                                    .rounded_md()
                                    .bg(rgb(PANEL_ALT_BG))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        img(ASSET_BRAND_CRAB_PNG_120)
                                            .w(px(60.0))
                                            .h(px(60.0)),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .child("Crab Tide"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_MUTED))
                                                    .child("Original CrabCord art in live theme"),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("CHANNELS"),
                            )
                            .child(
                                div()
                                    .h(px(38.0))
                                    .rounded_md()
                                    .bg(rgb(PANEL_ACTIVE_BG))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_TEXT, 16.0, ACCENT_SOFT))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("general")),
                            )
                            .child(
                                div()
                                    .h(px(38.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_ANNOUNCEMENTS, 16.0, ACCENT_ALT))
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("announcements")),
                            )
                            .child(
                                div()
                                    .h(px(38.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_FORUM, 16.0, ACCENT_ALT))
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("design-review")),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("VOICE"),
                            )
                            .child(
                                div()
                                    .h(px(38.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_VOICE, 16.0, ACCENT_SOFT))
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("Voice Lounge")),
                            )
                            .child(div().h_full())
                            .child(
                                div()
                                    .h(px(64.0))
                                    .rounded_md()
                                    .bg(rgb(SHELL_BG))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        svg()
                                            .path(ASSET_AVATAR_1)
                                            .w(px(38.0))
                                            .h(px(38.0))
                                            .text_color(rgb(ACCENT_ALT)),
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .child("brad"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(mic_color))
                                                    .child(mic_state),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .id("toggle-mic")
                                            .w(px(76.0))
                                            .h(px(34.0))
                                            .rounded_md()
                                            .bg(rgb(PANEL_ALT_BG))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .gap_1()
                                            .text_sm()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child(
                                                svg()
                                                    .path(mic_icon)
                                                    .w(px(14.0))
                                                    .h(px(14.0))
                                                    .text_color(rgb(TEXT_PRIMARY)),
                                            )
                                            .child("Mic")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.toggle_mic(cx);
                                            })),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .bg(rgb(CHAT_BG))
                            .flex()
                            .flex_col()
                            .child(
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
                                            .child(STATUS_LINES[self.status_index]),
                                    ),
                            )
                            .child(
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
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(TEXT_PRIMARY))
                                                            .child("brad  Today at 10:42"),
                                                    )
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
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(TEXT_PRIMARY))
                                                            .child("design-bot  Today at 10:43"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(TEXT_SECONDARY))
                                                            .child(
                                                                "Keep this pass bold, fast, and unmistakably CrabCord.",
                                                            ),
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
                                                                    .child(STATUS_LINES[self.status_index]),
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
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_MUTED))
                                                    .child("Build focus"),
                                            )
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
                                    ),
                            ),
                    )
                    .child(right_panel),
            )
    }
}
