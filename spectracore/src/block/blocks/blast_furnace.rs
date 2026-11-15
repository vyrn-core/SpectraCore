use crate::block::{BlockBehaviour, BlockFuture, OnPlaceArgs};
use spectracore_data::block_properties::{BlockProperties, FurnaceLikeProperties};
use spectracore_macros::spectracore_block;
use spectracore_world::BlockStateId;

#[spectracore_block("minecraft:blast_furnace")]
pub struct BlastFurnaceBlock;

impl BlockBehaviour for BlastFurnaceBlock {
    fn on_place<'a>(&'a self, args: OnPlaceArgs<'a>) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            let mut props = FurnaceLikeProperties::default(args.block);
            props.facing = args
                .player
                .living_entity
                .entity
                .get_horizontal_facing()
                .opposite();
            props.to_state_id(args.block)
        })
    }
}
