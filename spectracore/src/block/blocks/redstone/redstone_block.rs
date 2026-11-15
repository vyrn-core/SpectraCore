use spectracore_macros::spectracore_block;

use crate::block::{BlockBehaviour, BlockFuture, EmitsRedstonePowerArgs, GetRedstonePowerArgs};

#[spectracore_block("minecraft:redstone_block")]
pub struct RedstoneBlock;

impl BlockBehaviour for RedstoneBlock {
    fn get_weak_redstone_power<'a>(
        &'a self,
        _args: GetRedstonePowerArgs<'a>,
    ) -> BlockFuture<'a, u8> {
        Box::pin(async move { 15 })
    }

    fn emits_redstone_power<'a>(
        &'a self,
        _args: EmitsRedstonePowerArgs<'a>,
    ) -> BlockFuture<'a, bool> {
        Box::pin(async move { true })
    }
}
