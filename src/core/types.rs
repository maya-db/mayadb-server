use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::clone::Clone;
use std::fmt::Display;


pub type Document = BTreeMap<String, Type>;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum Type {
    // inicio tipo com prefixo u
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    // fim tipo com prefixo u

    // inicio tipo com prefixo i
    I8(u8),
    I16(u16),
    I32(u32),
    I64(u64),
    I128(u128),
    // fim tipo com prefixo i

    Str(String),
    Bool(bool),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::U8(v) | Type::I8(v) => write!(f, "{v}"),
            Type::U16(v) | Type::I16(v) => write!(f, "{v}"),
            Type::U32(v) | Type::I32(v) => write!(f, "{v}"),
            Type::U64(v) | Type::I64(v) => write!(f, "{v}"),
            Type::U128(v) | Type::I128(v) => write!(f, "{v}"),
            Type::Str(v) => write!(f, "{v}"),
            Type::Bool(v) => write!(f, "{v}"),
        }
    }
}
