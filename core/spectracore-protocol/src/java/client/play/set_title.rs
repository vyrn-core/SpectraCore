use spectracore_data::packet::clientbound::PLAY_SET_TITLE_TEXT;
use spectracore_util::text::TextComponent;

use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(PLAY_SET_TITLE_TEXT)]
pub struct CTitleText<'a> {
    pub title: &'a TextComponent,
}

impl<'a> CTitleText<'a> {
    pub fn new(title: &'a TextComponent) -> Self {
        Self { title }
    }
}
