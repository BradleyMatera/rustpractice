use gpui::*;

use crate::ui::theme::{APP_BG, ASSET_ICON_ACTION_MUTE, ASSET_ICON_ACTION_UNMUTE, SHELL_BG, SUCCESS, WARN};

use super::chat_panel::build_chat_panel;
use super::left_panel::build_left_panel;
use super::right_panel::build_right_panel;
use super::CrabCordShell;

impl Render for CrabCordShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mic_state = if self.muted { "Muted" } else { "Live" };
        let mic_color = if self.muted { WARN } else { SUCCESS };
        let mic_icon = if self.muted {
            ASSET_ICON_ACTION_MUTE
        } else {
            ASSET_ICON_ACTION_UNMUTE
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
                    .child(build_left_panel(mic_state, mic_color, mic_icon, cx))
                    .child(build_chat_panel(self, cx))
                    .child(build_right_panel(self, cx)),
            )
    }
}
