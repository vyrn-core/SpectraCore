use spectracore_data::packet::clientbound::PLAY_SET_ACTION_BAR_TEXT;
use spectracore_util::text::TextComponent;

use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(PLAY_SET_ACTION_BAR_TEXT)]
pub struct CActionBar<'a> {
    pub action_bar: &'a TextComponent,
}

impl<'a> CActionBar<'a> {
    pub fn new(action_bar: &'a TextComponent) -> Self {
        Self { action_bar }
    }
}
