use crate::block::registry::BlockActionResult;
use crate::block::{BlockBehaviour, BlockFuture, NormalUseArgs};

use spectracore_inventory::crafting::crafting_screen_handler::CraftingTableScreenHandler;
use spectracore_inventory::player::player_inventory::PlayerInventory;
use spectracore_inventory::screen_handler::{
    BoxFuture, InventoryPlayer, ScreenHandlerFactory, SharedScreenHandler,
};
use spectracore_macros::spectracore_block;
use spectracore_util::text::TextComponent;
use std::sync::Arc;
use tokio::sync::Mutex;

#[spectracore_block("minecraft:crafting_table")]
pub struct CraftingTableBlock;

impl BlockBehaviour for CraftingTableBlock {
    fn normal_use<'a>(&'a self, args: NormalUseArgs<'a>) -> BlockFuture<'a, BlockActionResult> {
        Box::pin(async move {
            args.player
                .open_handled_screen(&CraftingTableScreenFactory)
                .await;

            BlockActionResult::Success
        })
    }
}

struct CraftingTableScreenFactory;

impl ScreenHandlerFactory for CraftingTableScreenFactory {
    fn create_screen_handler<'a>(
        &'a self,
        sync_id: u8,
        player_inventory: &'a Arc<PlayerInventory>,
        _player: &'a dyn InventoryPlayer,
    ) -> BoxFuture<'a, Option<SharedScreenHandler>> {
        Box::pin(async move {
            let handler = CraftingTableScreenHandler::new(sync_id, player_inventory).await;
            let concrete_arc = Arc::new(Mutex::new(handler));

            Some(concrete_arc as SharedScreenHandler)
        })
    }

    fn get_display_name(&self) -> TextComponent {
        TextComponent::translate("container.crafting", &[])
    }
}
