use spectracore_data::packet::clientbound::PLAY_CHUNK_BATCH_START;
use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(PLAY_CHUNK_BATCH_START)]
pub struct CChunkBatchStart;
