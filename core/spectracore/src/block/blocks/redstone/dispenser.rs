use crate::block::{BlockBehaviour, BlockFuture, OnPlaceArgs};
use spectracore_data::block_properties::{BlockProperties, DispenserLikeProperties};
use spectracore_macros::spectracore_block;
use spectracore_world::BlockStateId;

#[spectracore_block("minecraft:dispenser")]
pub struct DispenserBlock;

impl BlockBehaviour for DispenserBlock {
    fn on_place<'a>(&'a self, args: OnPlaceArgs<'a>) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            let mut props = DispenserLikeProperties::default(args.block);
            props.facing = args.player.living_entity.entity.get_facing().opposite();
            props.to_state_id(args.block)
        })
    }
}
