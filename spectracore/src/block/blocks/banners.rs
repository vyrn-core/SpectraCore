use crate::block::{BlockBehaviour, BlockFuture, OnPlaceArgs};
use crate::entity::EntityBase;
use spectracore_data::block_properties::{BlockProperties, WhiteBannerLikeProperties};
use spectracore_data::tag::{RegistryKey, get_tag_values};
use spectracore_macros::spectracore_block_from_tag;
use spectracore_world::BlockStateId;

#[spectracore_block_from_tag("minecraft:banners")]
pub struct BannerBlock;

impl BlockBehaviour for BannerBlock {
    fn on_place<'a>(&'a self, args: OnPlaceArgs<'a>) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            let mut props = WhiteBannerLikeProperties::default(args.block);
            props.rotation = args.player.get_entity().get_flipped_rotation_16();
            props.to_state_id(args.block)
        })
    }
}
