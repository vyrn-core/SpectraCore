use std::sync::Arc;

use spectracore_macros::spectracore_block;
use spectracore_world::block::entities::mob_spawner::MobSpawnerBlockEntity;

use crate::block::{BlockBehaviour, BlockFuture, PlacedArgs};

#[spectracore_block("minecraft:spawner")]
pub struct SpawnerBlock;

impl BlockBehaviour for SpawnerBlock {
    fn placed<'a>(&'a self, args: PlacedArgs<'a>) -> BlockFuture<'a, ()> {
        Box::pin(async move {
            let hopper_block_entity = MobSpawnerBlockEntity::new(*args.position);
            args.world
                .add_block_entity(Arc::new(hopper_block_entity))
                .await;
        })
    }
}
