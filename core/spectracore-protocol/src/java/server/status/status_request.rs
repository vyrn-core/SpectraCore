use spectracore_data::packet::serverbound::STATUS_STATUS_REQUEST;
use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(STATUS_STATUS_REQUEST)]
pub struct SStatusRequest;
