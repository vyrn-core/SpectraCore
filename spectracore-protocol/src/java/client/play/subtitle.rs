use spectracore_data::packet::clientbound::PLAY_SET_SUBTITLE_TEXT;
use spectracore_util::text::TextComponent;

use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(PLAY_SET_SUBTITLE_TEXT)]
pub struct CSubtitle<'a> {
    pub subtitle: &'a TextComponent,
}

impl<'a> CSubtitle<'a> {
    pub fn new(subtitle: &'a TextComponent) -> Self {
        Self { subtitle }
    }
}
