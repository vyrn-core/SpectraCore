use crate::Link;
use spectracore_data::packet::clientbound::PLAY_SERVER_LINKS;
use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(PLAY_SERVER_LINKS)]
pub struct CPlayServerLinks<'a> {
    pub links: &'a [Link<'a>],
}

impl<'a> CPlayServerLinks<'a> {
    pub fn new(links: &'a [Link<'a>]) -> Self {
        Self { links }
    }
}
