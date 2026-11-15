use spectracore_data::packet::serverbound::STATUS_PING_REQUEST;
use spectracore_macros::packet;
use serde::Serialize;

#[derive(serde::Deserialize, Serialize)]
#[packet(STATUS_PING_REQUEST)]
pub struct SStatusPingRequest {
    pub payload: i64,
}
