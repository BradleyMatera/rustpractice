use gpui::*;

const NAVY_950: u32 = 0x071426;
const NAVY_900: u32 = 0x0E2138;
const NAVY_800: u32 = 0x163759;
const OCEAN_500: u32 = 0x2A9DD6;
const CRAB_500: u32 = 0xF2554C;
const GOLD_400: u32 = 0xF2C65C;
const SAND_200: u32 = 0xF3DFC2;
const FOAM_50: u32 = 0xF3FAFF;
const INK_900: u32 = 0x112438;

const ASSET_BRAND_WORDMARK_360: &str = "brand/crabcord-wordmark-360x72.svg";
const ASSET_BRAND_MASCOT_56: &str = "brand/crabcord-mascot-56x56.png";

const FEATURE_CARDS: [(&str, &str, u32); 3] = [
    (
        "Feature Parity",
        "Same core social/chat capabilities as Discord, rebuilt with a stability-first architecture.",
        CRAB_500,
    ),
    (
        "Local Live Streams",
        "Host live streams from your local device to communities. Current state: X (planned).",
        GOLD_400,
    ),
    (
        "Embedded Games",
        "Run lightweight social games inside the client process for shared sessions.",
        OCEAN_500,
    ),
];

const STATUS_ROTATION: [&str; 4] = [
    "early build shell",
    "channel list refactor completed",
    "streaming feature planned (X)",
    "games embed architecture draft",
];

pub struct CrabCordShell {
    status_line: SharedString,
    click_count: u32,
    status_index: usize,
}

impl CrabCordShell {
    pub fn new() -> Self {
        Self {
            status_line: "Status: early build shell".into(),
            click_count: 0,
            status_index: 0,
        }
    }

    fn cycle_status_message(&mut self, cx: &mut Context<Self>) {
        self.status_index = (self.status_index + 1) % STATUS_ROTATION.len();
        self.status_line = format!("Status: {}", STATUS_ROTATION[self.status_index]).into();
        self.click_count += 1;
        cx.notify();
    }

    fn show_stream_status(&mut self, cx: &mut Context<Self>) {
        self.status_line = "Status: local live streams are planned (X)".into();
        cx.notify();
    }
}

fn nav_tile(label: &'static str, background: u32, foreground: u32) -> impl IntoElement {
    div()
        .w(px(56.0))
        .h(px(56.0))
        .bg(rgb(background))
        .rounded_md()
        .text_color(rgb(foreground))
        .text_sm()
        .flex()
        .justify_center()
        .items_center()
        .child(label)
}

fn feature_card(title: &'static str, summary: &'static str, accent: u32) -> impl IntoElement {
    div()
        .w(px(240.0))
        .h(px(175.0))
        .bg(rgb(0xffffff))
        .border_1()
        .border_color(rgb(0xd5e8f5))
        .rounded_md()
        .px_4()
        .py_4()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().w(px(44.0)).h(px(8.0)).bg(rgb(accent)).rounded_md())
        .child(div().text_xl().text_color(rgb(INK_900)).child(title))
        .child(div().text_sm().text_color(rgb(0x33506a)).child(summary))
}

fn action_button(
    id: &'static str,
    background: u32,
    foreground: u32,
    label: &'static str,
) -> Stateful<Div> {
    div()
        .id(id)
        .h(px(38.0))
        .px_4()
        .bg(rgb(background))
        .text_color(rgb(foreground))
        .rounded_md()
        .border_1()
        .border_color(rgb(0xa7bfd1))
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .child(label)
}

impl Render for CrabCordShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(NAVY_950))
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .w(px(1220.0))
                    .h(px(760.0))
                    .bg(rgb(NAVY_900))
                    .border_1()
                    .border_color(rgb(0x1f3a5a))
                    .rounded_md()
                    .flex()
                    .child(
                        div()
                            .w(px(92.0))
                            .h_full()
                            .bg(rgb(NAVY_950))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_3()
                            .py_4()
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(CRAB_500))
                                    .rounded_md()
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .child(img(ASSET_BRAND_MASCOT_56).w(px(42.0)).h(px(42.0))),
                            )
                            .child(nav_tile("Crab", NAVY_800, SAND_200))
                            .child(nav_tile("Live", OCEAN_500, NAVY_950)),
                    )
                    .child(
                        div()
                            .w(px(290.0))
                            .h_full()
                            .bg(rgb(NAVY_800))
                            .px_4()
                            .py_4()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_xl().text_color(rgb(SAND_200)).child("CrabCord"))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xc0daee))
                                    .child("Discord, but with stability in mind."),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(GOLD_400))
                                    .child("Rust, of course."),
                            )
                            .child(
                                div()
                                    .bg(rgb(0x112d48))
                                    .rounded_md()
                                    .px_3()
                                    .py_3()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0xe5f4ff))
                                            .child("Channel: #project-brief"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0xa5c6df))
                                            .child(self.status_line.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0xa5c6df))
                                            .child(format!("Clicks: {}", self.click_count)),
                                    ),
                            )
                            .child(
                                div()
                                    .bg(rgb(0x112d48))
                                    .rounded_md()
                                    .px_3()
                                    .py_3()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0xe5f4ff))
                                            .child("Voice: Crab Lounge"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0xa5c6df))
                                            .child("Streaming roadmap active"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .bg(rgb(FOAM_50))
                            .px_5()
                            .py_5()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .bg(rgb(0xffffff))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xd5e8f5))
                                    .px_4()
                                    .py_3()
                                    .child(
                                        svg()
                                            .path(ASSET_BRAND_WORDMARK_360)
                                            .w(px(240.0))
                                            .h(px(48.0)),
                                    )
                                    .child(
                                        div()
                                            .text_2xl()
                                            .text_color(rgb(INK_900))
                                            .child("CrabCord"),
                                    )
                                    .child(
                                        div().text_sm().text_color(rgb(0x3c5c77)).child(
                                            "Discord replacement focused on reliability, performance, and clean Rust architecture.",
                                        ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(feature_card(
                                        FEATURE_CARDS[0].0,
                                        FEATURE_CARDS[0].1,
                                        FEATURE_CARDS[0].2,
                                    ))
                                    .child(feature_card(
                                        FEATURE_CARDS[1].0,
                                        FEATURE_CARDS[1].1,
                                        FEATURE_CARDS[1].2,
                                    ))
                                    .child(feature_card(
                                        FEATURE_CARDS[2].0,
                                        FEATURE_CARDS[2].1,
                                        FEATURE_CARDS[2].2,
                                    )),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(
                                        action_button(
                                            "cycle-status",
                                            OCEAN_500,
                                            NAVY_950,
                                            "Cycle Status",
                                        )
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.cycle_status_message(cx);
                                        })),
                                    )
                                    .child(
                                        action_button(
                                            "stream-action",
                                            GOLD_400,
                                            INK_900,
                                            "Try Stream Action",
                                        )
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.show_stream_status(cx);
                                        })),
                                    ),
                            )
                            .child(
                                div()
                                    .bg(rgb(SAND_200))
                                    .rounded_md()
                                    .px_4()
                                    .py_3()
                                    .text_color(rgb(0x3f2f11))
                                    .child("Project codename: CrabCord")
                                    .child("Theme source: crab mascot palette (navy, ocean blue, crab red, gold, sand)."),
                            ),
                    ),
            )
    }
}
