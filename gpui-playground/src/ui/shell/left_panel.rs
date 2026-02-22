use gpui::*;
use gpui::StatefulInteractiveElement;
use gpui::prelude::*;

use crate::ui::elements::mono_icon;
use crate::ui::theme::{
    ACCENT, ACCENT_ALT, ASSET_ICON_ACTION_ADD, ASSET_ICON_ACTION_MUTE, ASSET_ICON_ACTION_UNMUTE,
    ASSET_ICON_NAV_DISCOVER, ASSET_ICON_NAV_SEARCH, ASSET_ICON_NAV_SETTINGS, ASSET_ICON_STATUS_ONLINE,
    CHANNELS_BG, GUILD_RAIL_BG, PANEL_ACTIVE_BG, PANEL_ALT_BG, SHELL_BG, SUCCESS, WARN, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};

use super::CrabCordShell;

pub(super) fn build_left_panel(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> AnyElement {
    div()
        .child(build_guild_rail(shell, cx))
        .child(build_workspace_panel(shell, cx))
        .into_any_element()
}

fn build_guild_rail(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> impl IntoElement {
    let mut rail = div()
        .w(px(170.0))
        .h_full()
        .bg(rgb(GUILD_RAIL_BG))
        .rounded_md()
        .py_3()
        .px_2()
        .flex()
        .flex_col()
        .items_start()
        .gap_2();

    for (index, guild) in shell.workspace.guilds.iter().enumerate() {
        let selected = index == shell.active_guild;
        let row_bg = if selected { PANEL_ACTIVE_BG } else { SHELL_BG };
        let badge_text = if guild.unread > 99 {
            "99+".to_string()
        } else {
            guild.unread.to_string()
        };
        rail = rail.child(
            div()
                .w_full()
                .h(px(56.0))
                .id(format!("guild-{index}"))
                .rounded_lg()
                .bg(rgb(row_bg))
                .cursor_pointer()
                .relative()
                .flex()
                .items_center()
                .gap_2()
                .px_2()
                .child(img(guild.icon_png).w(px(36.0)).h(px(36.0)))
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child(guild.name),
                )
                .when(guild.unread > 0, |tile| {
                    tile.child(
                        div()
                            .absolute()
                            .right(px(4.0))
                            .top(px(4.0))
                            .min_w(px(20.0))
                            .h(px(20.0))
                            .px_1()
                            .rounded_lg()
                            .bg(rgb(ACCENT))
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(badge_text),
                    )
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.select_guild(index, cx);
                })),
        );
    }

    rail.child(
        div()
            .w_full()
            .h(px(56.0))
            .rounded_lg()
            .bg(rgb(PANEL_ALT_BG))
            .cursor_pointer()
            .flex()
            .items_center()
            .gap_2()
            .px_2()
            .child(mono_icon(ASSET_ICON_ACTION_ADD, 20.0, TEXT_PRIMARY))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Add Server"),
            ),
    )
}

fn build_workspace_panel(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> impl IntoElement {
    let mut channels = div().flex().flex_col().gap_1();
    for channel in &shell.workspace.channels {
        let selected = shell.active_conversation == channel.id;
        let row_bg = if selected { PANEL_ACTIVE_BG } else { CHANNELS_BG };
        let row_text = if selected { TEXT_PRIMARY } else { TEXT_SECONDARY };
        let channel_id = channel.id;
        channels = channels.child(
            div()
                .h(px(34.0))
                .id(format!("channel-{}", channel.name))
                .rounded_md()
                .bg(rgb(row_bg))
                .px_2()
                .cursor_pointer()
                .flex()
                .items_center()
                .gap_2()
                .child(mono_icon(channel.icon_png, 14.0, row_text))
                .child(div().text_sm().text_color(rgb(row_text)).child(channel.name))
                .child(div().w_full())
                .when(channel.unread > 0, |row| {
                    row.child(
                        div()
                            .w(px(18.0))
                            .h(px(18.0))
                            .rounded_lg()
                            .bg(rgb(ACCENT))
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(channel.unread.to_string()),
                    )
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.select_channel(channel_id, cx);
                })),
        );
    }

    let mut dms = div().flex().flex_col().gap_1();
    for dm in &shell.workspace.dms {
        let selected = shell.active_conversation == dm.id;
        let row_bg = if selected { PANEL_ACTIVE_BG } else { CHANNELS_BG };
        let status_color = if dm.online { SUCCESS } else { TEXT_MUTED };
        let dm_id = dm.id;
        dms = dms.child(
            div()
                .id(format!("dm-{}", dm.name))
                .rounded_md()
                .bg(rgb(row_bg))
                .px_2()
                .py_2()
                .cursor_pointer()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .relative()
                        .child(img(dm.avatar_png).w(px(28.0)).h(px(28.0)))
                        .child(
                            div()
                                .absolute()
                                .right(px(-2.0))
                                .bottom(px(-2.0))
                                .w(px(10.0))
                                .h(px(10.0))
                                .rounded_lg()
                                .bg(rgb(status_color)),
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
                                .child(dm.name),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED))
                                .child(dm.subtitle),
                        ),
                )
                .when(dm.is_bot, |row| {
                    row.child(
                        div()
                            .px_1()
                            .rounded_sm()
                            .bg(rgb(PANEL_ALT_BG))
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .child("BOT"),
                    )
                })
                .when(dm.unread > 0, |row| {
                    row.child(
                        div()
                            .w(px(18.0))
                            .h(px(18.0))
                            .rounded_lg()
                            .bg(rgb(ACCENT))
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(dm.unread.to_string()),
                    )
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.select_dm(dm_id, cx);
                })),
        );
    }

    let mic_state = if shell.muted { "Muted" } else { "Live" };
    let mic_color = if shell.muted { WARN } else { SUCCESS };
    let mic_icon = if shell.muted {
        ASSET_ICON_ACTION_MUTE
    } else {
        ASSET_ICON_ACTION_UNMUTE
    };

    div()
        .w(px(320.0))
        .h_full()
        .bg(rgb(CHANNELS_BG))
        .rounded_md()
        .px_3()
        .py_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .h(px(34.0))
                .rounded_md()
                .bg(rgb(SHELL_BG))
                .px_2()
                .flex()
                .items_center()
                .gap_2()
                .child(mono_icon(ASSET_ICON_NAV_SEARCH, 14.0, TEXT_MUTED))
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child("Find or start a conversation"),
                ),
        )
        .child(
            div()
                .rounded_md()
                .bg(rgb(PANEL_ALT_BG))
                .border_1()
                .border_color(rgb(0x2A6D95))
                .px_3()
                .py_2()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child(shell.workspace.title),
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
                                .h(px(20.0))
                                .px_2()
                                .rounded_md()
                                .bg(rgb(PANEL_ACTIVE_BG))
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(mono_icon(ASSET_ICON_STATUS_ONLINE, 10.0, TEXT_PRIMARY))
                                .child(format!("{} online", shell.online_count)),
                        )
                        .child(
                            div()
                                .h(px(20.0))
                                .px_2()
                                .rounded_md()
                                .bg(rgb(PANEL_ACTIVE_BG))
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(mono_icon(ASSET_ICON_NAV_DISCOVER, 10.0, TEXT_PRIMARY))
                                .child("asset desk ready"),
                        ),
                ),
        )
        .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("CHANNELS"))
        .child(channels)
        .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("DIRECT MESSAGES"))
        .child(dms)
        .child(div().h_full())
        .child(
            div()
                .rounded_md()
                .bg(rgb(SHELL_BG))
                .px_2()
                .py_2()
                .flex()
                .items_center()
                .gap_2()
                .child(img("ui/avatars/avatar-crab-6.png").w(px(30.0)).h(px(30.0)))
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
                        .h(px(30.0))
                        .px_2()
                        .rounded_md()
                        .bg(rgb(PANEL_ALT_BG))
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_sm()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child(mono_icon(mic_icon, 13.0, TEXT_PRIMARY))
                        .child("Mic")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.toggle_mic(cx);
                        })),
                )
                .child(
                    div()
                        .h(px(30.0))
                        .px_2()
                        .rounded_md()
                        .bg(rgb(PANEL_ALT_BG))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(mono_icon(ASSET_ICON_NAV_SETTINGS, 14.0, ACCENT_ALT)),
                ),
        )
}
