use spectracore_data::packet::clientbound::CONFIG_DISCONNECT;
use spectracore_macros::packet;
use serde::Deserialize;

#[derive(serde::Serialize, Deserialize)]
#[packet(CONFIG_DISCONNECT)]
pub struct CConfigDisconnect<'a> {
    pub reason: &'a str,
}

impl<'a> CConfigDisconnect<'a> {
    pub fn new(reason: &'a str) -> Self {
        Self { reason }
    }
}
