use crate::entity::EntityBase;
use crate::entity::player::Player;
use crate::server::Server;
use spectracore_data::Block;
use spectracore_data::BlockDirection;
use spectracore_data::item::Item;
use spectracore_util::math::position::BlockPos;
use spectracore_world::item::ItemStack;
use std::collections::HashMap;
use std::sync::Arc;

use super::{ItemBehaviour, ItemMetadata};

#[derive(Default)]
pub struct ItemRegistry {
    items: HashMap<&'static Item, Arc<dyn ItemBehaviour>>,
}

impl ItemRegistry {
    pub fn register<T: ItemBehaviour + ItemMetadata + 'static>(&mut self, item: T) {
        let val = Arc::new(item);
        self.items.reserve(T::ids().len());
        for i in T::ids() {
            self.items.insert(Item::from_id(i).unwrap(), val.clone());
        }
    }

    pub async fn on_use(&self, item: &Item, player: &Player) {
        let spectracore_item = self.get_spectracore_item(item);
        if let Some(spectracore_item) = spectracore_item {
            spectracore_item.normal_use(item, player).await;
        }
    }

    pub async fn use_on_block(
        &self,
        stack: &mut ItemStack,
        player: &Player,
        location: BlockPos,
        face: BlockDirection,
        block: &Block,
        server: &Server,
    ) {
        let spectracore_item = self.get_spectracore_item(stack.item);
        if let Some(spectracore_item) = spectracore_item {
            spectracore_item
                .use_on_block(stack, player, location, face, block, server)
                .await;
        }
    }

    pub async fn use_on_entity(
        &self,
        stack: &mut ItemStack,
        player: &Player,
        entity: Arc<dyn EntityBase>,
    ) {
        let spectracore_item = self.get_spectracore_item(stack.item);
        if let Some(spectracore_item) = spectracore_item {
            spectracore_item.use_on_entity(stack, player, entity).await;
        }
    }

    pub fn can_mine(&self, item: &Item, player: &Player) -> bool {
        let spectracore_block = self.get_spectracore_item(item);
        if let Some(spectracore_block) = spectracore_block {
            return spectracore_block.can_mine(player);
        }
        true
    }

    #[must_use]
    pub fn get_spectracore_item(&self, item: &Item) -> Option<&dyn ItemBehaviour> {
        self.items.get(item).map(|value| &**value)
    }
}
