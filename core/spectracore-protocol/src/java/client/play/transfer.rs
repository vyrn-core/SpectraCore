use crate::VarInt;
use spectracore_data::packet::clientbound::PLAY_TRANSFER;
use spectracore_macros::packet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[packet(PLAY_TRANSFER)]
pub struct CTransfer<'a> {
    pub host: &'a str,
    pub port: VarInt,
}

impl<'a> CTransfer<'a> {
    pub fn new(host: &'a str, port: VarInt) -> Self {
        Self { host, port }
    }
}
