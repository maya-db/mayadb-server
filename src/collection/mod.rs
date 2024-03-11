use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
pub use types::Type;
mod operators;
mod types;
mod load;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Collection {
    name: String,
    number_of_columns: u8,
    auto_increment: u64,
    columns: BTreeMap<String, u8>,
    data: Vec<BTreeMap<u8, Type>>,
}

impl Collection {
    pub fn init(name: &str) -> Self {
        Self {
            name: String::from(name),
            columns: BTreeMap::from([("id".to_string(), 0)]),
            number_of_columns: 0,
            auto_increment: 0,
            data: Vec::new(),
        }
    }
}
