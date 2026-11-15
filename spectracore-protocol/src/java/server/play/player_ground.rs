use spectracore_data::packet::serverbound::PLAY_MOVE_PLAYER_STATUS_ONLY;
use spectracore_macros::packet;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[packet(PLAY_MOVE_PLAYER_STATUS_ONLY)]
pub struct SSetPlayerGround {
    pub on_ground: bool,
}
