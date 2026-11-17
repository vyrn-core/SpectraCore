use spectracore_data::packet::clientbound::PLAY_DISCONNECT;
use spectracore_util::text::TextComponent;

use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(PLAY_DISCONNECT)]
pub struct CPlayDisconnect<'a> {
    pub reason: &'a TextComponent,
}

impl<'a> CPlayDisconnect<'a> {
    pub fn new(reason: &'a TextComponent) -> Self {
        Self { reason }
    }
}
