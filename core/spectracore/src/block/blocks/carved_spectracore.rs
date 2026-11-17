use spectracore_data::{
    Block,
    block_properties::{BlockProperties, WallTorchLikeProperties},
};
use spectracore_world::BlockStateId;

use crate::block::{BlockBehaviour, BlockFuture, BlockMetadata, OnPlaceArgs};

pub struct CarvedSpectraCoreBlock;

impl BlockMetadata for CarvedSpectraCoreBlock {
    fn namespace(&self) -> &'static str {
        "minecraft"
    }

    fn ids(&self) -> &'static [&'static str] {
        &[Block::JACK_O_LANTERN.name, Block::CARVED_PUMPKIN.name]
    }
}

impl BlockBehaviour for CarvedSpectraCoreBlock {
    fn on_place<'a>(&'a self, args: OnPlaceArgs<'a>) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            let mut props = WallTorchLikeProperties::default(args.block);
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
