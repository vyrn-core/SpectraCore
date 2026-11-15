use spectracore_data::packet::serverbound::LOGIN_LOGIN_ACKNOWLEDGED;
use spectracore_macros::packet;
use serde::Serialize;

/// Acknowledgement to the `CLoginSuccess` packet sent by the server.
#[derive(Serialize)]
#[packet(LOGIN_LOGIN_ACKNOWLEDGED)]
pub struct SLoginAcknowledged;
