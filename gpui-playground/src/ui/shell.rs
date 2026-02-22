use gpui::*;

use super::theme::STATUS_LINES;

mod chat_panel;
mod left_panel;
mod render;
mod right_panel;

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
