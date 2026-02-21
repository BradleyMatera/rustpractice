use gpui::*;

const BG: u32 = 0x071426;
const PANEL: u32 = 0x0E2138;
const SIDEBAR: u32 = 0x163759;
const TEXT_LIGHT: u32 = 0xF3FAFF;
const TEXT_DARK: u32 = 0x112438;
const BLUE: u32 = 0x2A9DD6;
const RED: u32 = 0xF2554C;
const GOLD: u32 = 0xF2C65C;

const ASSET_BRAND_WORDMARK_360: &str = "brand/crabcord-wordmark-360x72.svg";
const ASSET_BRAND_MASCOT_56: &str = "brand/crabcord-mascot-56x56.png";

const STATUS_TEXT: [&str; 4] = [
    "early build shell",
    "single-screen prototype",
    "ui simplification pass done",
    "local live streams are placeholder only",
];

pub struct CrabCordShell {
    status_index: usize,
}

impl CrabCordShell {
    pub fn new() -> Self {
        Self { status_index: 0 }
    }

    fn cycle_status(&mut self, cx: &mut Context<Self>) {
        self.status_index = (self.status_index + 1) % STATUS_TEXT.len();
        cx.notify();
    }

    fn set_stream_placeholder_status(&mut self, cx: &mut Context<Self>) {
        self.status_index = STATUS_TEXT.len() - 1;
        cx.notify();
    }
}

impl Render for CrabCordShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = format!("Status: {}", STATUS_TEXT[self.status_index]);

        div()
            .size_full()
            .bg(rgb(BG))
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .w(px(980.0))
                    .h(px(620.0))
                    .bg(rgb(PANEL))
                    .border_1()
                    .border_color(rgb(0x1D3C5A))
                    .rounded_md()
                    .flex()
                    .child(
                        div()
                            .w(px(96.0))
                            .h_full()
                            .bg(rgb(BG))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .py_5()
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(RED))
                                    .rounded_md()
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .child(img(ASSET_BRAND_MASCOT_56).w(px(42.0)).h(px(42.0))),
                            )
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(SIDEBAR))
                                    .rounded_md()
                                    .text_color(rgb(TEXT_LIGHT))
                                    .text_sm()
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .child("Crab"),
                            )
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(BLUE))
                                    .rounded_md()
                                    .text_color(rgb(BG))
                                    .text_sm()
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .child("Live"),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .px_6()
                            .py_6()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div().w(px(280.0)).h(px(56.0)).child(
                                    svg()
                                        .path(ASSET_BRAND_WORDMARK_360)
                                        .w(px(240.0))
                                        .h(px(48.0)),
                                ),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .text_color(rgb(TEXT_LIGHT))
                                    .child("CrabCord"),
                            )
                            .child(div().text_sm().text_color(rgb(0xC0DAEE)).child(
                                "UI-only prototype. Keep this screen simple and easy to edit.",
                            ))
                            .child(
                                div()
                                    .bg(rgb(SIDEBAR))
                                    .rounded_md()
                                    .px_4()
                                    .py_4()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(TEXT_LIGHT))
                                            .child("Channel: #project-brief"),
                                    )
                                    .child(div().text_sm().text_color(rgb(0xA5C6DF)).child(status)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .id("cycle-status")
                                            .h(px(38.0))
                                            .px_4()
                                            .bg(rgb(BLUE))
                                            .text_color(rgb(BG))
                                            .rounded_md()
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .child("Cycle Status")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.cycle_status(cx);
                                            })),
                                    )
                                    .child(
                                        div()
                                            .id("stream-action")
                                            .h(px(38.0))
                                            .px_4()
                                            .bg(rgb(GOLD))
                                            .text_color(rgb(TEXT_DARK))
                                            .rounded_md()
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .child("Stream Placeholder")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.set_stream_placeholder_status(cx);
                                            })),
                                    ),
                            )
                            .child(
                                div()
                                    .bg(rgb(0xF3DFC2))
                                    .rounded_md()
                                    .px_4()
                                    .py_3()
                                    .text_color(rgb(0x3f2f11))
                                    .child("Current scope: single-screen UI shell only.")
                                    .child("No routing, no persistence, no backend yet."),
                            ),
                    ),
            )
    }
}
