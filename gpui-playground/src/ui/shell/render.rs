use gpui::*;

use crate::ui::theme::{APP_BG, SHELL_BG};

use super::chat_panel::build_chat_panel;
use super::left_panel::build_left_panel;
use super::right_panel::build_right_panel;
use super::CrabCordShell;

impl Render for CrabCordShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(APP_BG))
            .flex()
            .p_4()
            .child(
                div()
                    .w_full()
                    .h_full()
                    .bg(rgb(SHELL_BG))
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(0x224B6A))
                    .flex()
                    .child(build_left_panel(self, cx))
                    .child(build_chat_panel(self, cx))
                    .child(build_right_panel(self, cx)),
            )
    }
}
