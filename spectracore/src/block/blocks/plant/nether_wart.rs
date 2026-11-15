use spectracore_data::Block;
use spectracore_macros::spectracore_block;
use spectracore_util::math::position::BlockPos;
use spectracore_world::BlockStateId;
use spectracore_world::world::BlockAccessor;

use crate::block::blocks::plant::PlantBlockBase;
use crate::block::{BlockBehaviour, BlockFuture, CanPlaceAtArgs, GetStateForNeighborUpdateArgs};

#[spectracore_block("minecraft:nether_wart")]
pub struct NetherWartBlock;

impl BlockBehaviour for NetherWartBlock {
    fn can_place_at<'a>(&'a self, args: CanPlaceAtArgs<'a>) -> BlockFuture<'a, bool> {
        Box::pin(async move {
            <Self as PlantBlockBase>::can_place_at(self, args.block_accessor, args.position).await
        })
    }

    fn get_state_for_neighbor_update<'a>(
        &'a self,
        args: GetStateForNeighborUpdateArgs<'a>,
    ) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            <Self as PlantBlockBase>::get_state_for_neighbor_update(
                self,
                args.world,
                args.position,
                args.state_id,
            )
            .await
        })
    }
}

impl PlantBlockBase for NetherWartBlock {
    async fn can_plant_on_top(&self, block_accessor: &dyn BlockAccessor, pos: &BlockPos) -> bool {
        let block = block_accessor.get_block(pos).await;
        block == &Block::SOUL_SAND
    }
}
