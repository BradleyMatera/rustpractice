use std::sync::Arc;

use gpui::*;

use crate::backend::{
    fake_workspace, BackendCommand, BackendSink, ConversationId, DmSeed, MemberSeed, MessageSeed, WorkspaceSeed,
};

mod chat_panel;
mod left_panel;
mod render;
mod right_panel;

#[derive(Clone, Debug)]
pub struct UiMessage {
    pub target: ConversationId,
    pub author: String,
    pub avatar_png: &'static str,
    pub body: String,
    pub time: String,
    pub is_bot: bool,
}

impl UiMessage {
    fn from_seed(seed: &MessageSeed) -> Self {
        Self {
            target: seed.target,
            author: seed.author.to_string(),
            avatar_png: seed.avatar_png,
            body: seed.body.to_string(),
            time: seed.time.to_string(),
            is_bot: seed.is_bot,
        }
    }
}

pub struct CrabCordShell {
    pub workspace: WorkspaceSeed,
    pub active_guild: usize,
    pub active_conversation: ConversationId,
    pub muted: bool,
    pub online_count: usize,
    pub show_asset_desk: bool,
    pub compose_index: usize,
    pub messages: Vec<UiMessage>,
    backend: Arc<dyn BackendSink>,
}

impl CrabCordShell {
    pub fn new_with_backend(backend: Arc<dyn BackendSink>) -> Self {
        let workspace = fake_workspace();
        let online_count = workspace.members.iter().filter(|member| member.online).count();
        let messages = workspace.messages.iter().map(UiMessage::from_seed).collect();
        Self {
            workspace,
            active_guild: 0,
            active_conversation: ConversationId::General,
            muted: false,
            online_count,
            show_asset_desk: false,
            compose_index: 0,
            messages,
            backend,
        }
    }

    fn select_guild(&mut self, index: usize, cx: &mut Context<Self>) {
        self.active_guild = index.min(self.workspace.guilds.len().saturating_sub(1));
        self.backend.send(BackendCommand::GuildSelected {
            index: self.active_guild,
        });
        cx.notify();
    }

    fn select_channel(&mut self, id: ConversationId, cx: &mut Context<Self>) {
        self.active_conversation = id;
        self.backend.send(BackendCommand::ChannelSelected { id });
        cx.notify();
    }

    fn select_dm(&mut self, id: ConversationId, cx: &mut Context<Self>) {
        self.active_conversation = id;
        self.backend.send(BackendCommand::DmSelected { id });
        cx.notify();
    }

    fn toggle_mic(&mut self, cx: &mut Context<Self>) {
        self.muted = !self.muted;
        self.backend.send(BackendCommand::MicToggled { muted: self.muted });
        cx.notify();
    }

    fn invite_member(&mut self, cx: &mut Context<Self>) {
        self.online_count = (self.online_count + 1).min(99);
        self.backend.send(BackendCommand::MemberInvited {
            online_count: self.online_count,
        });
        cx.notify();
    }

    fn toggle_asset_desk(&mut self, cx: &mut Context<Self>) {
        self.show_asset_desk = !self.show_asset_desk;
        self.backend.send(BackendCommand::AssetDeskToggled {
            open: self.show_asset_desk,
        });
        cx.notify();
    }

    fn send_quick_message(&mut self, cx: &mut Context<Self>) {
        const QUICK_REPLIES: [&str; 4] = [
            "Wired a real event path for this action.",
            "UI lane looks clean now. Keep this shape.",
            "Axiom actor boundary is ready for live workers.",
            "Shipping this pass with fake data and real interactions.",
        ];
        let message_body = QUICK_REPLIES[self.compose_index % QUICK_REPLIES.len()];
        self.compose_index += 1;

        self.messages.push(UiMessage {
            target: self.active_conversation,
            author: "you".to_string(),
            avatar_png: "ui/avatars/avatar-crab-6.png",
            body: message_body.to_string(),
            time: "now".to_string(),
            is_bot: false,
        });
        self.backend.send(BackendCommand::MessageSent {
            id: self.active_conversation,
            body: message_body.to_string(),
        });
        cx.notify();
    }

    fn active_name(&self) -> &'static str {
        if let Some(channel) = self
            .workspace
            .channels
            .iter()
            .find(|channel| channel.id == self.active_conversation)
        {
            channel.name
        } else if let Some(dm) = self
            .workspace
            .dms
            .iter()
            .find(|dm| dm.id == self.active_conversation)
        {
            dm.name
        } else {
            "general"
        }
    }

    fn active_topic(&self) -> &'static str {
        if let Some(channel) = self
            .workspace
            .channels
            .iter()
            .find(|channel| channel.id == self.active_conversation)
        {
            channel.topic
        } else if let Some(dm) = self
            .workspace
            .dms
            .iter()
            .find(|dm| dm.id == self.active_conversation)
        {
            dm.subtitle
        } else {
            "CrabCord chat"
        }
    }

    fn active_placeholder(&self) -> String {
        format!("Message @{}", self.active_name())
    }

    fn visible_messages(&self) -> Vec<&UiMessage> {
        self.messages
            .iter()
            .filter(|message| message.target == self.active_conversation)
            .collect()
    }

    fn is_dm(&self, id: ConversationId) -> bool {
        self.workspace.dms.iter().any(|dm| dm.id == id)
    }

    fn members_by_role<'a>(&'a self, role: &'a str) -> impl Iterator<Item = &'a MemberSeed> {
        self.workspace.members.iter().filter(move |member| member.role == role)
    }

    fn active_dm(&self) -> Option<&DmSeed> {
        self.workspace
            .dms
            .iter()
            .find(|dm| dm.id == self.active_conversation)
    }
}
