use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const VIRTUAL_SOL_RESERVE : u64 = 12_330_000_000u64; // 100 token in reserves
pub const VIRTUAL_TOKEN_RESERVE : u64 = 618_496_769u64; // 12 sol in reserves
pub const CONNECTOR_WEIGHT : u32 = 20;
