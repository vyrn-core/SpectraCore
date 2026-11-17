use spectracore_data::packet::serverbound::PLAY_CUSTOM_PAYLOAD;
use spectracore_macros::packet;

#[packet(PLAY_CUSTOM_PAYLOAD)]
pub struct SCustomPayload {
    // TODO
}
