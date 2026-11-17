use spectracore_data::packet::clientbound::PLAY_SYSTEM_CHAT;
use spectracore_util::text::TextComponent;

use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(PLAY_SYSTEM_CHAT)]
pub struct CSystemChatMessage<'a> {
    pub content: &'a TextComponent,
    pub overlay: bool,
}

impl<'a> CSystemChatMessage<'a> {
    pub fn new(content: &'a TextComponent, overlay: bool) -> Self {
        Self { content, overlay }
    }
}
