pub(crate) const APP_BG: u32 = 0x08131F;
pub(crate) const SHELL_BG: u32 = 0x0E2338;
pub(crate) const GUILD_RAIL_BG: u32 = 0x071525;
pub(crate) const CHANNELS_BG: u32 = 0x11304B;
pub(crate) const CHAT_BG: u32 = 0x0F2942;
pub(crate) const MEMBERS_BG: u32 = 0x11304B;
pub(crate) const PANEL_ALT_BG: u32 = 0x184361;
pub(crate) const PANEL_ACTIVE_BG: u32 = 0x24597E;
pub(crate) const INPUT_BG: u32 = 0x1A3F5D;
pub(crate) const TEXT_PRIMARY: u32 = 0xECF8FF;
pub(crate) const TEXT_SECONDARY: u32 = 0xB8D8EA;
pub(crate) const TEXT_MUTED: u32 = 0x84AFC8;
pub(crate) const ACCENT: u32 = 0xF2554C;
pub(crate) const ACCENT_ALT: u32 = 0x2A9DD6;
pub(crate) const ACCENT_SOFT: u32 = 0xF2C65C;
pub(crate) const SUCCESS: u32 = 0x33C07A;
pub(crate) const WARN: u32 = 0xF2C65C;

pub(crate) const ASSET_BRAND_MASCOT_56_SVG: &str = "brand/crabcord-mascot-56x56.svg";
pub(crate) const ASSET_BRAND_CRAB_ART_SVG: &str = "brand/crabcord-crab-art-1024.svg";
pub(crate) const ASSET_BRAND_CRAB_PNG_56: &str = "brand/crab-56.png";
pub(crate) const ASSET_BRAND_CRAB_PNG_96: &str = "brand/crab-96.png";
pub(crate) const ASSET_BRAND_CRAB_PNG_120: &str = "brand/crab-120.png";

pub(crate) const ASSET_ICON_NAV_FRIENDS: &str = "ui/icons/navigation/friends.svg";
pub(crate) const ASSET_ICON_NAV_DISCOVER: &str = "ui/icons/navigation/discover.svg";
pub(crate) const ASSET_ICON_NAV_SEARCH: &str = "ui/icons/navigation/search.svg";
pub(crate) const ASSET_ICON_NAV_SETTINGS: &str = "ui/icons/navigation/settings.svg";

pub(crate) const ASSET_ICON_CHANNEL_TEXT: &str = "ui/icons/channels/channel-text.svg";
pub(crate) const ASSET_ICON_CHANNEL_ANNOUNCEMENTS: &str = "ui/icons/channels/channel-announcements.svg";
pub(crate) const ASSET_ICON_CHANNEL_FORUM: &str = "ui/icons/channels/channel-forum.svg";
pub(crate) const ASSET_ICON_CHANNEL_VOICE: &str = "ui/icons/channels/channel-voice.svg";

pub(crate) const ASSET_ICON_ACTION_ADD: &str = "ui/icons/actions/add.svg";
pub(crate) const ASSET_ICON_ACTION_SEND: &str = "ui/icons/actions/send.svg";
pub(crate) const ASSET_ICON_ACTION_MUTE: &str = "ui/icons/actions/mute.svg";
pub(crate) const ASSET_ICON_ACTION_UNMUTE: &str = "ui/icons/actions/unmute.svg";
pub(crate) const ASSET_ICON_ACTION_INVITE: &str = "ui/icons/actions/invite.svg";

pub(crate) const ASSET_ICON_STATUS_ONLINE: &str = "ui/icons/status/online.svg";

pub(crate) const ASSET_AVATAR_1: &str = "ui/avatars/avatar-crab-1.svg";
pub(crate) const ASSET_BADGE_BOT: &str = "ui/badges/bot.svg";

pub(crate) const ALL_SVG_ASSETS: [&str; 66] = [
    ASSET_BRAND_CRAB_ART_SVG,
    "brand/crabcord-mark-120x120.svg",
    "brand/crabcord-mark-56x56.svg",
    "brand/crabcord-mascot-120x120.svg",
    "brand/crabcord-mascot-256x256.svg",
    "brand/crabcord-mascot-56x56.svg",
    "brand/crabcord-wordmark-360x72.svg",
    "mock/crabcord-shell-1280x800.svg",
    "ui/avatars/avatar-crab-1.svg",
    "ui/avatars/avatar-crab-2.svg",
    "ui/avatars/avatar-crab-3.svg",
    "ui/avatars/avatar-crab-4.svg",
    "ui/avatars/avatar-crab-5.svg",
    "ui/avatars/avatar-crab-6.svg",
    "ui/badges/booster.svg",
    "ui/badges/bot.svg",
    "ui/badges/mod.svg",
    "ui/badges/nitro.svg",
    "ui/badges/owner.svg",
    "ui/badges/verified.svg",
    "ui/icons/actions/add.svg",
    "ui/icons/actions/attach.svg",
    "ui/icons/actions/check.svg",
    "ui/icons/actions/close.svg",
    "ui/icons/actions/deafen.svg",
    "ui/icons/actions/delete.svg",
    "ui/icons/actions/edit.svg",
    "ui/icons/actions/emoji.svg",
    "ui/icons/actions/gif.svg",
    "ui/icons/actions/invite.svg",
    "ui/icons/actions/mute.svg",
    "ui/icons/actions/pin.svg",
    "ui/icons/actions/remove.svg",
    "ui/icons/actions/send.svg",
    "ui/icons/actions/stream.svg",
    "ui/icons/actions/undeafen.svg",
    "ui/icons/actions/unmute.svg",
    "ui/icons/actions/video.svg",
    "ui/icons/channels/channel-announcements.svg",
    "ui/icons/channels/channel-events.svg",
    "ui/icons/channels/channel-forum.svg",
    "ui/icons/channels/channel-media.svg",
    "ui/icons/channels/channel-rules.svg",
    "ui/icons/channels/channel-stage.svg",
    "ui/icons/channels/channel-text.svg",
    "ui/icons/channels/channel-voice.svg",
    "ui/icons/navigation/bookmark.svg",
    "ui/icons/navigation/discover.svg",
    "ui/icons/navigation/friends.svg",
    "ui/icons/navigation/help.svg",
    "ui/icons/navigation/home.svg",
    "ui/icons/navigation/inbox.svg",
    "ui/icons/navigation/notifications.svg",
    "ui/icons/navigation/search.svg",
    "ui/icons/navigation/settings.svg",
    "ui/icons/navigation/threads.svg",
    "ui/icons/status/dnd.svg",
    "ui/icons/status/idle.svg",
    "ui/icons/status/mobile.svg",
    "ui/icons/status/offline.svg",
    "ui/icons/status/online.svg",
    "ui/icons/status/streaming.svg",
    "ui/illustrations/empty-chat.svg",
    "ui/illustrations/empty-friends.svg",
    "ui/illustrations/onboarding.svg",
    "ui/illustrations/server-banner.svg",
];

pub(crate) const STATUS_LINES: [&str; 4] = [
    "Voice stable, 42ms",
    "Sync OK, no dropped frames",
    "Push notifications connected",
    "All core services nominal",
];
