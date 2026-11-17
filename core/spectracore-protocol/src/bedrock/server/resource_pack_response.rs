use crate::serial::PacketRead;
use spectracore_macros::packet;

#[derive(PacketRead)]
#[packet(8)]
pub struct SResourcePackResponse {
    pub response: u8,
    pub download_size: u16,
}
