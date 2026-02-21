use gpui::*;
use std::time::Duration;

const APP_BG: u32 = 0x232428;
const SHELL_BG: u32 = 0x313338;
const GUILD_RAIL_BG: u32 = 0x1E1F22;
const CHANNELS_BG: u32 = 0x2B2D31;
const CHAT_BG: u32 = 0x313338;
const MEMBERS_BG: u32 = 0x2B2D31;
const PANEL_ALT_BG: u32 = 0x383A40;
const INPUT_BG: u32 = 0x383A40;
const TEXT_PRIMARY: u32 = 0xF2F3F5;
const TEXT_SECONDARY: u32 = 0xB5BAC1;
const TEXT_MUTED: u32 = 0x949BA4;
const ACCENT: u32 = 0x5865F2;
const SUCCESS: u32 = 0x3BA55D;
const WARN: u32 = 0xFAA61A;

const ASSET_BRAND_MARK_56: &str = "brand/crabcord-mark-56x56.svg";
const ASSET_BRAND_WORDMARK_360: &str = "brand/crabcord-wordmark-360x72.svg";
const ASSET_BRAND_MASCOT_56_SVG: &str = "brand/crabcord-mascot-56x56.svg";

const ASSET_ICON_NAV_HOME: &str = "ui/icons/navigation/home.svg";
const ASSET_ICON_NAV_FRIENDS: &str = "ui/icons/navigation/friends.svg";
const ASSET_ICON_NAV_DISCOVER: &str = "ui/icons/navigation/discover.svg";
const ASSET_ICON_NAV_SEARCH: &str = "ui/icons/navigation/search.svg";
const ASSET_ICON_NAV_SETTINGS: &str = "ui/icons/navigation/settings.svg";

const ASSET_ICON_CHANNEL_TEXT: &str = "ui/icons/channels/channel-text.svg";
const ASSET_ICON_CHANNEL_ANNOUNCEMENTS: &str = "ui/icons/channels/channel-announcements.svg";
const ASSET_ICON_CHANNEL_FORUM: &str = "ui/icons/channels/channel-forum.svg";
const ASSET_ICON_CHANNEL_VOICE: &str = "ui/icons/channels/channel-voice.svg";

const ASSET_ICON_ACTION_ADD: &str = "ui/icons/actions/add.svg";
const ASSET_ICON_ACTION_SEND: &str = "ui/icons/actions/send.svg";
const ASSET_ICON_ACTION_MUTE: &str = "ui/icons/actions/mute.svg";
const ASSET_ICON_ACTION_UNMUTE: &str = "ui/icons/actions/unmute.svg";
const ASSET_ICON_ACTION_INVITE: &str = "ui/icons/actions/invite.svg";

const ASSET_ICON_STATUS_ONLINE: &str = "ui/icons/status/online.svg";

const ASSET_AVATAR_1: &str = "ui/avatars/avatar-crab-1.svg";
const ASSET_AVATAR_2: &str = "ui/avatars/avatar-crab-2.svg";
const ASSET_AVATAR_3: &str = "ui/avatars/avatar-crab-3.svg";

const ASSET_BADGE_BOT: &str = "ui/badges/bot.svg";
const ASSET_BADGE_VERIFIED: &str = "ui/badges/verified.svg";

const ASSET_ILLUSTRATION_BANNER: &str = "ui/illustrations/server-banner.svg";

const ALL_SVG_ASSETS: [&str; 65] = [
    "brand/crabcord-mark-120x120.svg",
    "brand/crabcord-mark-56x56.svg",
    "brand/crabcord-mascot-120x120.svg",
    "brand/crabcord-mascot-256x256.svg",
    "brand/crabcord-mascot-56x56.svg",
    "brand/crabcord-wordmark-360x72.svg",
    "mock/crabcord-shell-1280x800.svg",
    "ui/avatars/avatar-crab-1.svg",
    "ui/avatars/avatar-crab-2.svg",
    "ui/avatars/avatar-crab-3.svg",
    "ui/avatars/avatar-crab-4.svg",
    "ui/avatars/avatar-crab-5.svg",
    "ui/avatars/avatar-crab-6.svg",
    "ui/badges/booster.svg",
    "ui/badges/bot.svg",
    "ui/badges/mod.svg",
    "ui/badges/nitro.svg",
    "ui/badges/owner.svg",
    "ui/badges/verified.svg",
    "ui/icons/actions/add.svg",
    "ui/icons/actions/attach.svg",
    "ui/icons/actions/check.svg",
    "ui/icons/actions/close.svg",
    "ui/icons/actions/deafen.svg",
    "ui/icons/actions/delete.svg",
    "ui/icons/actions/edit.svg",
    "ui/icons/actions/emoji.svg",
    "ui/icons/actions/gif.svg",
    "ui/icons/actions/invite.svg",
    "ui/icons/actions/mute.svg",
    "ui/icons/actions/pin.svg",
    "ui/icons/actions/remove.svg",
    "ui/icons/actions/send.svg",
    "ui/icons/actions/stream.svg",
    "ui/icons/actions/undeafen.svg",
    "ui/icons/actions/unmute.svg",
    "ui/icons/actions/video.svg",
    "ui/icons/channels/channel-announcements.svg",
    "ui/icons/channels/channel-events.svg",
    "ui/icons/channels/channel-forum.svg",
    "ui/icons/channels/channel-media.svg",
    "ui/icons/channels/channel-rules.svg",
    "ui/icons/channels/channel-stage.svg",
    "ui/icons/channels/channel-text.svg",
    "ui/icons/channels/channel-voice.svg",
    "ui/icons/navigation/bookmark.svg",
    "ui/icons/navigation/discover.svg",
    "ui/icons/navigation/friends.svg",
    "ui/icons/navigation/help.svg",
    "ui/icons/navigation/home.svg",
    "ui/icons/navigation/inbox.svg",
    "ui/icons/navigation/notifications.svg",
    "ui/icons/navigation/search.svg",
    "ui/icons/navigation/settings.svg",
    "ui/icons/navigation/threads.svg",
    "ui/icons/status/dnd.svg",
    "ui/icons/status/idle.svg",
    "ui/icons/status/mobile.svg",
    "ui/icons/status/offline.svg",
    "ui/icons/status/online.svg",
    "ui/icons/status/streaming.svg",
    "ui/illustrations/empty-chat.svg",
    "ui/illustrations/empty-friends.svg",
    "ui/illustrations/onboarding.svg",
    "ui/illustrations/server-banner.svg",
];

const STATUS_LINES: [&str; 4] = [
    "Voice stable, 42ms",
    "Sync OK, no dropped frames",
    "Push notifications connected",
    "All core services nominal",
];

pub struct CrabCordShell {
    status_index: usize,
    muted: bool,
    online_count: usize,
}

fn mono_icon(path: &'static str, size: f32, color: u32) -> impl IntoElement {
    svg()
        .path(path)
        .w(px(size))
        .h(px(size))
        .text_color(rgb(color))
}

impl CrabCordShell {
    pub fn new() -> Self {
        Self {
            status_index: 0,
            muted: false,
            online_count: 7,
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
        let members_title = format!("MEMBERS — {}", self.online_count);
        let mut svg_gallery = div().flex().flex_wrap().gap_1();
        for path in ALL_SVG_ASSETS {
            let (tile_w, tile_h, icon_w, icon_h) = if path.contains("mock/") {
                (72.0, 46.0, 66.0, 40.0)
            } else if path.contains("illustrations/") || path.contains("wordmark") {
                (54.0, 36.0, 48.0, 28.0)
            } else {
                (28.0, 28.0, 18.0, 18.0)
            };
            svg_gallery = svg_gallery.child(
                div()
                    .w(px(tile_w))
                    .h(px(tile_h))
                    .rounded_sm()
                    .bg(rgb(PANEL_ALT_BG))
                    .border_1()
                    .border_color(rgb(0x484C56))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        svg()
                            .path(path)
                            .w(px(icon_w))
                            .h(px(icon_h))
                            .text_color(rgb(TEXT_PRIMARY)),
                    ),
            );
        }

        div()
            .size_full()
            .bg(rgb(APP_BG))
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .w(px(1260.0))
                    .h(px(780.0))
                    .bg(rgb(SHELL_BG))
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(0x1A1B1E))
                    .flex()
                    .child(
                        div()
                            .w(px(76.0))
                            .h_full()
                            .bg(rgb(GUILD_RAIL_BG))
                            .rounded_md()
                            .text_color(rgb(TEXT_SECONDARY))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .py_3()
                            .child(
                                div()
                                    .w(px(50.0))
                                    .h(px(50.0))
                                    .bg(rgb(ACCENT))
                                    .rounded_md()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        svg()
                                            .path(ASSET_ICON_NAV_HOME)
                                            .w(px(20.0))
                                            .h(px(20.0))
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .with_animation(
                                                "nav-home-float",
                                                Animation::new(Duration::from_secs(3)).repeat(),
                                                |icon, delta| {
                                                    let y = (delta * std::f32::consts::TAU).sin() * 1.6;
                                                    icon.with_transformation(Transformation::translate(
                                                        point(px(0.0), px(y)),
                                                    ))
                                                },
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .w(px(50.0))
                                    .h(px(50.0))
                                    .bg(rgb(PANEL_ALT_BG))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(mono_icon(ASSET_ICON_NAV_FRIENDS, 20.0, TEXT_SECONDARY)),
                            )
                            .child(
                                div()
                                    .w(px(50.0))
                                    .h(px(50.0))
                                    .bg(rgb(PANEL_ALT_BG))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(mono_icon(ASSET_ICON_NAV_DISCOVER, 20.0, TEXT_SECONDARY)),
                            )
                            .child(
                                div()
                                    .w(px(50.0))
                                    .h(px(50.0))
                                    .bg(rgb(PANEL_ALT_BG))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(mono_icon(ASSET_ICON_ACTION_ADD, 20.0, TEXT_SECONDARY)),
                            ),
                    )
                    .child(
                        div()
                            .w(px(260.0))
                            .h_full()
                            .bg(rgb(CHANNELS_BG))
                            .text_color(rgb(TEXT_SECONDARY))
                            .px_3()
                            .py_3()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .h(px(44.0))
                                    .rounded_md()
                                    .bg(rgb(SHELL_BG))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(svg().path(ASSET_BRAND_MASCOT_56_SVG).w(px(18.0)).h(px(18.0)))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("CrabCord Dev"))
                                    .child(div().w_full())
                                    .child(mono_icon(ASSET_ICON_NAV_SETTINGS, 16.0, TEXT_SECONDARY)),
                            )
                            .child(svg().path(ASSET_ILLUSTRATION_BANNER).w(px(236.0)).h(px(68.0)))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("TEXT CHANNELS"),
                            )
                            .child(
                                div()
                                    .h(px(34.0))
                                    .rounded_md()
                                    .bg(rgb(PANEL_ALT_BG))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_TEXT, 16.0, TEXT_PRIMARY))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("general")),
                            )
                            .child(
                                div()
                                    .h(px(34.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_ANNOUNCEMENTS, 16.0, TEXT_SECONDARY))
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("announcements")),
                            )
                            .child(
                                div()
                                    .h(px(34.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_FORUM, 16.0, TEXT_SECONDARY))
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("design-review")),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("VOICE CHANNELS"),
                            )
                            .child(
                                div()
                                    .h(px(34.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_CHANNEL_VOICE, 16.0, TEXT_SECONDARY))
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("Voice Lounge")),
                            )
                            .child(div().h_full())
                            .child(
                                div()
                                    .h(px(56.0))
                                    .rounded_md()
                                    .bg(rgb(SHELL_BG))
                                    .px_2()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(svg().path(ASSET_AVATAR_1).w(px(34.0)).h(px(34.0)))
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
                                            .w(px(70.0))
                                            .h(px(30.0))
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
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .with_animation(
                                                        "mic-state-pulse",
                                                        Animation::new(Duration::from_secs(2)).repeat(),
                                                        move |icon, delta| {
                                                            let scale = if mic_state == "Live" {
                                                                1.0
                                                                    + (delta
                                                                        * std::f32::consts::TAU)
                                                                        .sin()
                                                                        .abs()
                                                                        * 0.18
                                                            } else {
                                                                1.0
                                                            };
                                                            icon.with_transformation(
                                                                Transformation::scale(size(scale, scale)),
                                                            )
                                                        },
                                                    ),
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
                                    .h(px(52.0))
                                    .px_4()
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
                                            .child(mono_icon(ASSET_ICON_CHANNEL_TEXT, 16.0, TEXT_PRIMARY))
                                            .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("general"))
                                            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("Discord-style GPUI shell")),
                                    )
                                    .child(mono_icon(ASSET_ICON_NAV_SEARCH, 16.0, TEXT_SECONDARY))
                                    .child(
                                        div()
                                            .id("cycle-status")
                                            .w(px(122.0))
                                            .h(px(32.0))
                                            .rounded_md()
                                            .bg(rgb(ACCENT))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .gap_1()
                                            .text_sm()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child(
                                                svg()
                                                    .path(ASSET_ICON_NAV_DISCOVER)
                                                    .w(px(14.0))
                                                    .h(px(14.0))
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .with_animation(
                                                        "refresh-spin",
                                                        Animation::new(Duration::from_secs(2))
                                                            .repeat(),
                                                        |icon, delta| {
                                                            icon.with_transformation(
                                                                Transformation::rotate(percentage(delta)),
                                                            )
                                                        },
                                                    ),
                                            )
                                            .child("Refresh")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.cycle_status(cx);
                                            })),
                                    ),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .h_full()
                                    .px_4()
                                    .py_4()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .child(svg().path(ASSET_AVATAR_2).w(px(40.0)).h(px(40.0)))
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
                                                            .child(
                                                                "This is the first clean Discord-style layout pass in GPUI.",
                                                            ),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .w(px(40.0))
                                                    .h(px(40.0))
                                                    .rounded_md()
                                                    .bg(rgb(PANEL_ALT_BG))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(svg().path(ASSET_BADGE_BOT).w(px(24.0)).h(px(24.0))),
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
                                                                "Ship channels left, messages center, members right. Keep interaction obvious.",
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
                                    .child(div().h_full())
                                    .child(
                                        div()
                                            .h(px(58.0))
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
                                                    .w(px(100.0))
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
                                                    .child(mono_icon(ASSET_ICON_ACTION_SEND, 14.0, TEXT_PRIMARY))
                                                    .child("Send")
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.cycle_status(cx);
                                                    })),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .w(px(230.0))
                            .h_full()
                            .bg(rgb(MEMBERS_BG))
                            .text_color(rgb(TEXT_SECONDARY))
                            .px_3()
                            .py_3()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(members_title),
                            )
                            .child(
                                div()
                                    .h(px(38.0))
                                    .rounded_md()
                                    .bg(rgb(PANEL_ALT_BG))
                                    .px_2()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        svg()
                                            .path(ASSET_ICON_STATUS_ONLINE)
                                            .w(px(12.0))
                                            .h(px(12.0))
                                            .text_color(rgb(SUCCESS))
                                            .with_animation(
                                                "presence-pulse",
                                                Animation::new(Duration::from_secs(2)).repeat(),
                                                |icon, delta| {
                                                    let scale = 1.0
                                                        + (delta * std::f32::consts::TAU).sin().abs()
                                                            * 0.22;
                                                    icon.with_transformation(Transformation::scale(size(
                                                        scale, scale,
                                                    )))
                                                },
                                            ),
                                    )
                                    .child(svg().path(ASSET_AVATAR_1).w(px(22.0)).h(px(22.0)))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("brad")),
                            )
                            .child(
                                div()
                                    .h(px(38.0))
                                    .rounded_md()
                                    .bg(rgb(PANEL_ALT_BG))
                                    .px_2()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_STATUS_ONLINE, 12.0, SUCCESS))
                                    .child(svg().path(ASSET_AVATAR_2).w(px(22.0)).h(px(22.0)))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("design-bot"))
                                    .child(svg().path(ASSET_BADGE_VERIFIED).w(px(14.0)).h(px(14.0))),
                            )
                            .child(
                                div()
                                    .h(px(38.0))
                                    .rounded_md()
                                    .bg(rgb(PANEL_ALT_BG))
                                    .px_2()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(mono_icon(ASSET_ICON_STATUS_ONLINE, 12.0, SUCCESS))
                                    .child(svg().path(ASSET_AVATAR_3).w(px(22.0)).h(px(22.0)))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("rustacean")),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("SVG LIBRARY (65)"),
                            )
                            .child(div().h(px(280.0)).pr_1().child(svg_gallery))
                            .child(div().h_full())
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(svg().path(ASSET_BRAND_MARK_56).w(px(22.0)).h(px(22.0)))
                                    .child(svg().path(ASSET_BRAND_WORDMARK_360).w(px(160.0)).h(px(32.0))),
                            )
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
                            ),
                    ),
            )
    }
}
