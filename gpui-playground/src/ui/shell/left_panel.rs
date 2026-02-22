use gpui::*;
use gpui::StatefulInteractiveElement;

use crate::ui::elements::mono_icon;
use crate::ui::theme::{
    ACCENT, ACCENT_ALT, ACCENT_SOFT, ASSET_AVATAR_1, ASSET_BRAND_CRAB_PNG_120, ASSET_BRAND_CRAB_PNG_56,
    ASSET_BRAND_CRAB_PNG_96, ASSET_BRAND_MASCOT_56_PNG, ASSET_ICON_ACTION_ADD, ASSET_ICON_CHANNEL_ANNOUNCEMENTS,
    ASSET_ICON_CHANNEL_FORUM, ASSET_ICON_CHANNEL_TEXT, ASSET_ICON_CHANNEL_VOICE, ASSET_ICON_NAV_DISCOVER,
    ASSET_ICON_NAV_FRIENDS, ASSET_ICON_NAV_SETTINGS, ASSET_ICON_STATUS_ONLINE, CHANNELS_BG, GUILD_RAIL_BG,
    PANEL_ACTIVE_BG, PANEL_ALT_BG, SHELL_BG, SUCCESS, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

use super::CrabCordShell;

pub(super) fn build_left_panel(
    mic_state: &'static str,
    mic_color: u32,
    mic_icon: &'static str,
    cx: &mut Context<CrabCordShell>,
) -> AnyElement {
    div()
        .child(build_guild_rail())
        .child(build_channels_panel(mic_state, mic_color, mic_icon, cx))
        .into_any_element()
}

fn build_guild_rail() -> impl IntoElement {
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
        )
}

fn build_channels_panel(
    mic_state: &'static str,
    mic_color: u32,
    mic_icon: &'static str,
    cx: &mut Context<CrabCordShell>,
) -> impl IntoElement {
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
                .child(img(ASSET_BRAND_MASCOT_56_PNG).w(px(18.0)).h(px(18.0)))
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
                                        .child(mono_icon(ASSET_ICON_STATUS_ONLINE, 10.0, SUCCESS))
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
                                        .child(mono_icon(ASSET_ICON_NAV_DISCOVER, 10.0, ACCENT_SOFT))
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
        .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("CHANNELS"))
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
        .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("VOICE"))
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
                .child(img(ASSET_AVATAR_1).w(px(38.0)).h(px(38.0)))
                .child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("brad"))
                        .child(div().text_sm().text_color(rgb(mic_color)).child(mic_state)),
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
                        .child(mono_icon(mic_icon, 14.0, TEXT_PRIMARY))
                        .child("Mic")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.toggle_mic(cx);
                        })),
                ),
        )
}
