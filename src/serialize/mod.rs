//! Serialize content to disk or across the wire.

mod datainput;
mod dataoutput;

// Implements the NBT serialization present in the alpha
// 1.2.6 of Minecraft.
pub mod nbt;

pub use datainput::*;
pub use dataoutput::*;
