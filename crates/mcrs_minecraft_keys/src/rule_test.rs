// Written by `cargo run -p mcrs_minecraft_update -- keys`; do not edit.

use mcrs_minecraft_core::{StaticResourceLocation, rl};

pub const ALWAYS_TRUE: StaticResourceLocation = rl!("minecraft:always_true");
pub const BLOCK_MATCH: StaticResourceLocation = rl!("minecraft:block_match");
pub const BLOCKSTATE_MATCH: StaticResourceLocation = rl!("minecraft:blockstate_match");
pub const TAG_MATCH: StaticResourceLocation = rl!("minecraft:tag_match");
pub const RANDOM_BLOCK_MATCH: StaticResourceLocation = rl!("minecraft:random_block_match");
pub const RANDOM_BLOCKSTATE_MATCH: StaticResourceLocation = rl!("minecraft:random_blockstate_match");

pub const ENTRIES: &[StaticResourceLocation] = &[
    ALWAYS_TRUE,
    BLOCK_MATCH,
    BLOCKSTATE_MATCH,
    TAG_MATCH,
    RANDOM_BLOCK_MATCH,
    RANDOM_BLOCKSTATE_MATCH,
];
