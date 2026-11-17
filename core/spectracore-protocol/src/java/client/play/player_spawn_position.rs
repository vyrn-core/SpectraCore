use serde::Serialize;

use spectracore_data::packet::clientbound::PLAY_SET_DEFAULT_SPAWN_POSITION;
use spectracore_macros::packet;
use spectracore_util::math::position::BlockPos;

#[derive(Serialize)]
#[packet(PLAY_SET_DEFAULT_SPAWN_POSITION)]
pub struct CPlayerSpawnPosition {
    pub location: BlockPos,
    pub angle: f32,
}

impl CPlayerSpawnPosition {
    pub fn new(location: BlockPos, angle: f32) -> Self {
        Self { location, angle }
    }
}
