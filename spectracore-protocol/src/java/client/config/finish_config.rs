use spectracore_data::packet::clientbound::CONFIG_FINISH_CONFIGURATION;
use spectracore_macros::packet;
use serde::Serialize;

#[derive(Serialize)]
#[packet(CONFIG_FINISH_CONFIGURATION)]
pub struct CFinishConfig;
