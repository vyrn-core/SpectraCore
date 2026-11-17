use crate::VarInt;
use spectracore_data::packet::clientbound::CONFIG_TRANSFER;
use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(CONFIG_TRANSFER)]
pub struct CTransfer<'a> {
    pub host: &'a str,
    pub port: &'a VarInt,
}

impl<'a> CTransfer<'a> {
    pub fn new(host: &'a str, port: &'a VarInt) -> Self {
        Self { host, port }
    }
}
