use gpui::*;

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
const ASSET_BRAND_MARK_120: &str = "brand/crabcord-mark-120x120.svg";
const ASSET_BRAND_WORDMARK_360: &str = "brand/crabcord-wordmark-360x72.svg";
const ASSET_BRAND_MASCOT_56_SVG: &str = "brand/crabcord-mascot-56x56.svg";
const ASSET_BRAND_MASCOT_120_SVG: &str = "brand/crabcord-mascot-120x120.svg";
const ASSET_BRAND_MASCOT_256_SVG: &str = "brand/crabcord-mascot-256x256.svg";

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
        let mic_state = if self.muted { "Mic muted" } else { "Mic live" };
        let mic_color = if self.muted { WARN } else { SUCCESS };
        let members_title = format!("MEMBERS — {}", self.online_count);

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
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(svg().path(ASSET_BRAND_MARK_56).w(px(28.0)).h(px(28.0))),
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
                                    .child(
                                        svg()
                                            .path(ASSET_BRAND_MASCOT_56_SVG)
                                            .w(px(30.0))
                                            .h(px(30.0)),
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
                                    .child(svg().path(ASSET_BRAND_MARK_120).w(px(30.0)).h(px(30.0))),
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
                                    .child("+"),
                            ),
                    )
                    .child(
                        div()
                            .w(px(260.0))
                            .h_full()
                            .bg(rgb(CHANNELS_BG))
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
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("CrabCord Dev")),
                            )
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
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("# general")),
                            )
                            .child(
                                div()
                                    .h(px(34.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("# announcements")),
                            )
                            .child(
                                div()
                                    .h(px(34.0))
                                    .rounded_md()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("# design-review")),
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
                                    .child(
                                        div()
                                            .w(px(36.0))
                                            .h(px(36.0))
                                            .rounded_md()
                                            .bg(rgb(PANEL_ALT_BG))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(
                                                svg()
                                                    .path(ASSET_BRAND_MASCOT_56_SVG)
                                                    .w(px(24.0))
                                                    .h(px(24.0)),
                                            ),
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
                                            .w(px(70.0))
                                            .h(px(30.0))
                                            .rounded_md()
                                            .bg(rgb(PANEL_ALT_BG))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child("Toggle")
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
                                    .flex()
                                    .items_center()
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("# general"))
                                            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("Discord-style GPUI shell")),
                                    )
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
                                            .text_sm()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child("Refresh Status")
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
                                            .child(
                                                div()
                                                    .w(px(40.0))
                                                    .h(px(40.0))
                                                    .rounded_md()
                                                    .bg(rgb(PANEL_ALT_BG))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        svg()
                                                            .path(ASSET_BRAND_MASCOT_120_SVG)
                                                            .w(px(30.0))
                                                            .h(px(30.0)),
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
                                                    .child(svg().path(ASSET_BRAND_MARK_120).w(px(26.0)).h(px(26.0))),
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
                                                    .w(px(90.0))
                                                    .h(px(34.0))
                                                    .rounded_md()
                                                    .bg(rgb(ACCENT))
                                                    .cursor_pointer()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_PRIMARY))
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
                                    .child(svg().path(ASSET_BRAND_MASCOT_120_SVG).w(px(24.0)).h(px(24.0)))
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
                                    .child(svg().path(ASSET_BRAND_MARK_56).w(px(20.0)).h(px(20.0)))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("design-bot")),
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
                                    .child(svg().path(ASSET_BRAND_MASCOT_256_SVG).w(px(24.0)).h(px(24.0)))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("rustacean")),
                            )
                            .child(div().h_full())
                            .child(
                                div()
                                    .child(svg().path(ASSET_BRAND_WORDMARK_360).w(px(180.0)).h(px(36.0))),
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
                                    .text_sm()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child("Invite Member")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.invite_member(cx);
                                    })),
                            ),
                    ),
            )
    }
}
