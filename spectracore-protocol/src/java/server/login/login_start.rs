use std::io::Read;

use spectracore_data::packet::serverbound::LOGIN_HELLO;
use spectracore_macros::packet;
use serde::Serialize;

use crate::{
    ServerPacket,
    ser::{NetworkReadExt, ReadingError},
};

#[packet(LOGIN_HELLO)]
#[derive(Serialize)]
pub struct SLoginStart {
    pub name: String, // 16
    pub uuid: uuid::Uuid,
}

impl ServerPacket for SLoginStart {
    fn read(read: impl Read) -> Result<Self, ReadingError> {
        let mut read = read;
        Ok(Self {
            name: read.get_string_bounded(16)?,
            uuid: read.get_uuid()?,
        })
    }
}
