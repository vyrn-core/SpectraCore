use spectracore_data::packet::serverbound::PLAY_COMMAND_SUGGESTION;
use spectracore_macros::packet;
use serde::Deserialize;

use crate::VarInt;

#[derive(Deserialize)]
#[packet(PLAY_COMMAND_SUGGESTION)]
pub struct SCommandSuggestion {
    pub id: VarInt,
    pub command: String,
}
