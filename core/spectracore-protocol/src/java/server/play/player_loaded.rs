use spectracore_data::packet::serverbound::PLAY_PLAYER_LOADED;
use spectracore_macros::packet;

#[packet(PLAY_PLAYER_LOADED)]
pub struct SPlayerLoaded;
