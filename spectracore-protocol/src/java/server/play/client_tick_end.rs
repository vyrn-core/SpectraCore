use spectracore_data::packet::serverbound::PLAY_CLIENT_TICK_END;
use spectracore_macros::packet;

#[packet(PLAY_CLIENT_TICK_END)]
pub struct SClientTickEnd;
