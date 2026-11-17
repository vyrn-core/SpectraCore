use crate::serial::PacketRead;
use spectracore_macros::packet;

#[derive(PacketRead)]
#[packet(129)]
pub struct SClientCacheStatus {
    // https://mojang.github.io/bedrock-protocol-docs/html/ClientCacheStatusPacket.html
    pub cache_supported: bool,
}
