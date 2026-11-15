use crate::entity::player::Player;
use crate::item::ItemBehaviour;
use crate::item::ItemMetadata;
use crate::server::Server;
use crate::world::World;
use spectracore_data::Block;
use spectracore_data::BlockDirection;
use spectracore_data::item::Item;
use spectracore_util::math::position::BlockPos;
use spectracore_world::item::ItemStack;
use spectracore_world::world::BlockFlags;
use std::pin::Pin;
use std::sync::Arc;

use crate::item::items::ignite::ignition::Ignition;

pub struct FlintAndSteelItem;

impl ItemMetadata for FlintAndSteelItem {
    fn ids() -> Box<[u16]> {
        [Item::FLINT_AND_STEEL.id].into()
    }
}

impl ItemBehaviour for FlintAndSteelItem {
    fn use_on_block<'a>(
        &'a self,
        _item: &'a mut ItemStack,
        player: &'a Player,
        location: BlockPos,
        face: BlockDirection,
        block: &'a Block,
        _server: &'a Server,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            Ignition::ignite_block(
                |world: Arc<World>, pos: BlockPos, new_state_id: u16| async move {
                    world
                        .set_block_state(&pos, new_state_id, BlockFlags::NOTIFY_ALL)
                        .await;
                },
                player,
                location,
                face,
                block,
            )
            .await;
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
