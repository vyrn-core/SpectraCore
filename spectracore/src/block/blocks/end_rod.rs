use crate::block::BlockFuture;
use crate::block::{BlockBehaviour, OnPlaceArgs};
use spectracore_data::Block;
use spectracore_data::block_properties::BlockProperties;
use spectracore_data::block_properties::EndRodLikeProperties;
use spectracore_macros::spectracore_block;
use spectracore_world::BlockStateId;

#[spectracore_block("minecraft:end_rod")]
pub struct EndRodBlock;

impl BlockBehaviour for EndRodBlock {
    fn on_place<'a>(&'a self, args: OnPlaceArgs<'a>) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            let mut props = EndRodLikeProperties::default(args.block);

            let blockstate = args
                .world
                .get_block_state_id(&args.position.offset(args.direction.to_offset()))
                .await;

            if Block::from_state_id(blockstate).eq(args.block)
                && EndRodLikeProperties::from_state_id(blockstate, args.block).facing
                    == args.direction.to_facing().opposite()
            {
                props.facing = args.direction.to_facing();
            } else {
                props.facing = args.direction.to_facing().opposite();
            }

            props.to_state_id(args.block)
        })
    }
}
