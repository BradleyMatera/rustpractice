use gpui::*;
use gpui::StatefulInteractiveElement;
use gpui::prelude::*;

use crate::ui::elements::{build_asset_gallery, mono_icon};
use crate::ui::theme::{
    ACCENT, ACCENT_SOFT, ALL_PNG_ASSETS, ASSET_ICON_ACTION_INVITE, ASSET_ICON_NAV_DISCOVER, MEMBERS_BG,
    PANEL_ACTIVE_BG, PANEL_ALT_BG, SUCCESS, TEXT_MUTED, TEXT_PRIMARY,
};

use super::CrabCordShell;

pub(super) fn build_right_panel(shell: &CrabCordShell, cx: &mut Context<CrabCordShell>) -> AnyElement {
    let right_width = if shell.show_asset_desk { 390.0 } else { 300.0 };
    let members_title = format!("CREW — {}", shell.online_count);
    let asset_title = format!("ASSET LIBRARY ({})", ALL_PNG_ASSETS.len());

    if shell.show_asset_desk {
        div()
            .w(px(right_width))
            .h_full()
            .bg(rgb(MEMBERS_BG))
            .rounded_md()
            .px_3()
            .py_3()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("ASSET DESK"))
            .child(
                div()
                    .id("toggle-asset-desk")
                    .h(px(34.0))
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
                    .child("Back to Crew")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_asset_desk(cx);
                    })),
            )
            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(asset_title))
            .child(
                div()
                    .h_full()
                    .id("asset-desk-scroll")
                    .overflow_y_scroll()
                    .pr_1()
                    .child(build_asset_gallery()),
            )
            .into_any_element()
    } else {
        let mut members = div()
            .h_full()
            .id("member-scroll")
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .gap_2();
        for role in ["Senior Dev", "Journeyman Dev", "Online", "Offline"] {
            let mut role_group = div()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(role));
            for member in shell.members_by_role(role) {
                let status_color = if member.online { SUCCESS } else { TEXT_MUTED };
                role_group = role_group.child(
                    div()
                        .h(px(38.0))
                        .rounded_md()
                        .bg(rgb(PANEL_ALT_BG))
                        .px_2()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .relative()
                                .child(img(member.avatar_png).w(px(22.0)).h(px(22.0)))
                                .child(
                                    div()
                                        .absolute()
                                        .right(px(-2.0))
                                        .bottom(px(-2.0))
                                        .w(px(8.0))
                                        .h(px(8.0))
                                        .rounded_lg()
                                        .bg(rgb(status_color)),
                                ),
                        )
                        .child(
                            div()
                                .w_full()
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child(member.name),
                        )
                        .when(member.is_bot, |row| {
                            row.child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .bg(rgb(PANEL_ACTIVE_BG))
                                    .text_sm()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child("BOT"),
                            )
                        }),
                );
            }
            members = members.child(role_group);
        }

        div()
            .w(px(right_width))
            .h_full()
            .bg(rgb(MEMBERS_BG))
            .rounded_md()
            .px_3()
            .py_3()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(members_title))
            .child(
                div()
                    .id("toggle-asset-desk")
                    .h(px(34.0))
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
                    .child("Open Asset Desk")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_asset_desk(cx);
                    })),
            )
            .child(members)
            .child(
                div()
                    .id("invite-member")
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
                    .child(mono_icon(ASSET_ICON_ACTION_INVITE, 14.0, TEXT_PRIMARY))
                    .child("Invite")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.invite_member(cx);
                    })),
            )
            .into_any_element()
    }
}
