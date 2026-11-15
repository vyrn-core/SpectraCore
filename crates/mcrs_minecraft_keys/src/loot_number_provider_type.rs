// Written by `cargo run -p mcrs_minecraft_update -- keys`; do not edit.

use mcrs_minecraft_core::{StaticResourceLocation, rl};

pub const CONSTANT: StaticResourceLocation = rl!("minecraft:constant");
pub const UNIFORM: StaticResourceLocation = rl!("minecraft:uniform");
pub const BINOMIAL: StaticResourceLocation = rl!("minecraft:binomial");
pub const SCORE: StaticResourceLocation = rl!("minecraft:score");
pub const STORAGE: StaticResourceLocation = rl!("minecraft:storage");
pub const ENCHANTMENT_LEVEL: StaticResourceLocation = rl!("minecraft:enchantment_level");

pub const ENTRIES: &[StaticResourceLocation] = &[
    CONSTANT,
    UNIFORM,
    BINOMIAL,
    SCORE,
    STORAGE,
    ENCHANTMENT_LEVEL,
];
