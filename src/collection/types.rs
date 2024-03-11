use serde::{Serialize, Deserialize};
use std::clone::Clone;

#[derive(Clone, Serialize, Deserialize)]
pub enum Type {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    String(String),
    Space,
}
