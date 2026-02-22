use std::sync::mpsc;

#[cfg_attr(feature = "axiom-backend", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConversationId {
    General,
    Announcements,
    DesignReview,
    DmBrad,
    DmDesignBot,
    DmRustacean,
}

#[derive(Clone, Debug)]
pub struct GuildSeed {
    #[allow(dead_code)]
    pub name: &'static str,
    pub icon_png: &'static str,
    pub unread: u16,
}

#[derive(Clone, Debug)]
pub struct ChannelSeed {
    pub id: ConversationId,
    pub name: &'static str,
    pub topic: &'static str,
    pub icon_png: &'static str,
    pub unread: u8,
}

#[derive(Clone, Debug)]
pub struct DmSeed {
    pub id: ConversationId,
    pub name: &'static str,
    pub subtitle: &'static str,
    pub avatar_png: &'static str,
    pub online: bool,
    pub unread: u8,
    pub is_bot: bool,
}

#[derive(Clone, Debug)]
pub struct MemberSeed {
    pub name: &'static str,
    pub role: &'static str,
    pub avatar_png: &'static str,
    pub online: bool,
    pub is_bot: bool,
}

#[derive(Clone, Debug)]
pub struct MessageSeed {
    pub target: ConversationId,
    pub author: &'static str,
    pub avatar_png: &'static str,
    pub body: &'static str,
    pub time: &'static str,
    pub is_bot: bool,
}

#[derive(Clone, Debug)]
pub struct WorkspaceSeed {
    pub title: &'static str,
    pub guilds: Vec<GuildSeed>,
    pub channels: Vec<ChannelSeed>,
    pub dms: Vec<DmSeed>,
    pub members: Vec<MemberSeed>,
    pub messages: Vec<MessageSeed>,
}

pub fn fake_workspace() -> WorkspaceSeed {
    WorkspaceSeed {
        title: "CrabCord Dev",
        guilds: vec![
            GuildSeed {
                name: "CrabCord",
                icon_png: "brand/crab-56.png",
                unread: 0,
            },
            GuildSeed {
                name: "Design",
                icon_png: "ui/avatars/avatar-crab-2.png",
                unread: 30,
            },
            GuildSeed {
                name: "Build",
                icon_png: "ui/avatars/avatar-crab-3.png",
                unread: 16,
            },
            GuildSeed {
                name: "Ops",
                icon_png: "ui/avatars/avatar-crab-4.png",
                unread: 9,
            },
        ],
        channels: vec![
            ChannelSeed {
                id: ConversationId::General,
                name: "general",
                topic: "CrabCord ship lane",
                icon_png: "ui/icons/channels/channel-text.png",
                unread: 0,
            },
            ChannelSeed {
                id: ConversationId::Announcements,
                name: "announcements",
                topic: "release notes and status",
                icon_png: "ui/icons/channels/channel-announcements.png",
                unread: 2,
            },
            ChannelSeed {
                id: ConversationId::DesignReview,
                name: "design-review",
                topic: "layout, spacing, interaction",
                icon_png: "ui/icons/channels/channel-forum.png",
                unread: 0,
            },
        ],
        dms: vec![
            DmSeed {
                id: ConversationId::DmBrad,
                name: "thisisbrad",
                subtitle: "studying",
                avatar_png: "ui/avatars/avatar-crab-1.png",
                online: true,
                unread: 1,
                is_bot: false,
            },
            DmSeed {
                id: ConversationId::DmDesignBot,
                name: "design-bot",
                subtitle: "layout helper",
                avatar_png: "ui/badges/bot.png",
                online: true,
                unread: 0,
                is_bot: true,
            },
            DmSeed {
                id: ConversationId::DmRustacean,
                name: "rustacean",
                subtitle: "shipping hard",
                avatar_png: "ui/avatars/avatar-crab-5.png",
                online: true,
                unread: 0,
                is_bot: false,
            },
        ],
        members: vec![
            MemberSeed {
                name: "thisisbrad",
                role: "Senior Dev",
                avatar_png: "ui/avatars/avatar-crab-1.png",
                online: true,
                is_bot: false,
            },
            MemberSeed {
                name: "design-bot",
                role: "Senior Dev",
                avatar_png: "ui/badges/bot.png",
                online: true,
                is_bot: true,
            },
            MemberSeed {
                name: "Chunkywizard",
                role: "Journeyman Dev",
                avatar_png: "ui/avatars/avatar-crab-4.png",
                online: true,
                is_bot: false,
            },
            MemberSeed {
                name: "Testuser1",
                role: "Online",
                avatar_png: "ui/avatars/avatar-crab-2.png",
                online: true,
                is_bot: false,
            },
            MemberSeed {
                name: "Testuser2",
                role: "Offline",
                avatar_png: "ui/avatars/avatar-crab-3.png",
                online: false,
                is_bot: false,
            },
        ],
        messages: vec![
            MessageSeed {
                target: ConversationId::General,
                author: "thisisbrad",
                avatar_png: "ui/avatars/avatar-crab-1.png",
                body: "I started to look into this and it is what he is using now: https://docs.rs/axiom/latest/axiom/",
                time: "8:34 PM",
                is_bot: false,
            },
            MessageSeed {
                target: ConversationId::General,
                author: "design-bot",
                avatar_png: "ui/badges/bot.png",
                body: "Build the UI lane first. Keep channels left, timeline center, crew right.",
                time: "8:42 PM",
                is_bot: true,
            },
            MessageSeed {
                target: ConversationId::General,
                author: "Chunkywizard",
                avatar_png: "ui/avatars/avatar-crab-4.png",
                body: "All converted to PNGs. Next step is backend events wired cleanly.",
                time: "9:12 PM",
                is_bot: false,
            },
            MessageSeed {
                target: ConversationId::Announcements,
                author: "design-bot",
                avatar_png: "ui/badges/bot.png",
                body: "Release lane: UI shell moved to split modules and backend command boundary added.",
                time: "7:05 PM",
                is_bot: true,
            },
            MessageSeed {
                target: ConversationId::DesignReview,
                author: "thisisbrad",
                avatar_png: "ui/avatars/avatar-crab-1.png",
                body: "Spacing pass: aim for readable lanes and less cramped controls.",
                time: "6:44 PM",
                is_bot: false,
            },
            MessageSeed {
                target: ConversationId::DmBrad,
                author: "thisisbrad",
                avatar_png: "ui/avatars/avatar-crab-1.png",
                body: "Ship the shell first, then plug in real actor workers.",
                time: "6:30 PM",
                is_bot: false,
            },
            MessageSeed {
                target: ConversationId::DmDesignBot,
                author: "design-bot",
                avatar_png: "ui/badges/bot.png",
                body: "I can post lint and status events once the Axiom actor is live.",
                time: "6:31 PM",
                is_bot: true,
            },
            MessageSeed {
                target: ConversationId::DmRustacean,
                author: "rustacean",
                avatar_png: "ui/avatars/avatar-crab-5.png",
                body: "Fast path: command channel + actor mailbox + deterministic UI updates.",
                time: "6:35 PM",
                is_bot: false,
            },
        ],
    }
}

#[cfg_attr(feature = "axiom-backend", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum BackendCommand {
    GuildSelected { index: usize },
    ChannelSelected { id: ConversationId },
    DmSelected { id: ConversationId },
    MessageSent { id: ConversationId, body: String },
    MicToggled { muted: bool },
    MemberInvited { online_count: usize },
    AssetDeskToggled { open: bool },
}

pub trait BackendSink: Send + Sync {
    fn send(&self, command: BackendCommand);
}

pub struct ChannelBackend {
    tx: mpsc::Sender<BackendCommand>,
}

impl ChannelBackend {
    pub fn new(tx: mpsc::Sender<BackendCommand>) -> Self {
        Self { tx }
    }
}

impl BackendSink for ChannelBackend {
    fn send(&self, command: BackendCommand) {
        let _ = self.tx.send(command);
    }
}

#[cfg(feature = "axiom-backend")]
pub fn to_axiom_message(command: BackendCommand) -> axiom::prelude::Message {
    axiom::prelude::Message::new(command)
}
