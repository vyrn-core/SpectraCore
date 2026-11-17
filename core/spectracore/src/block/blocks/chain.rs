use crate::block::BlockFuture;
use crate::block::{BlockBehaviour, OnPlaceArgs};
use spectracore_data::BlockDirection;
use spectracore_data::block_properties::Axis;
use spectracore_data::block_properties::BlockProperties;
use spectracore_macros::spectracore_block;
use spectracore_world::BlockStateId;

#[spectracore_block("minecraft:iron_chain")]
pub struct ChainBlock;

impl BlockBehaviour for ChainBlock {
    fn on_place<'a>(&'a self, args: OnPlaceArgs<'a>) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            let mut props =
                spectracore_data::block_properties::IronChainLikeProperties::default(args.block);
            props.r#waterlogged = args.replacing.water_source();
            props.r#axis = match args.direction {
                BlockDirection::East | BlockDirection::West => Axis::X,
                BlockDirection::Up | BlockDirection::Down => Axis::Y,
                BlockDirection::North | BlockDirection::South => Axis::Z,
            };

            props.to_state_id(args.block)
        })
    }
}
