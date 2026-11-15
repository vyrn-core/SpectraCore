use spectracore_data::packet::serverbound::PLAY_CHANGE_GAME_MODE;
use spectracore_macros::packet;
use spectracore_util::GameMode;
use serde::Serialize;

#[derive(serde::Deserialize, Serialize)]
#[packet(PLAY_CHANGE_GAME_MODE)]
pub struct SChangeGameMode {
    pub game_mode: GameMode,
}
