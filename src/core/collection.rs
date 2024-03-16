use borsh::{BorshSerialize, BorshDeserialize};
pub use super::Document;


#[derive(BorshSerialize, BorshDeserialize)]
pub struct Collection {
    name: String,
    auto_increment: u64,
    data: Vec<Document>,
    document_max_size: usize,
}

impl Collection {
    pub fn init(name: &str) -> Self {
        Self {
            name: String::from(name),
            auto_increment: 0,
            data: Vec::new(),
            document_max_size: 1024 * 1024, // 1 MB
        }
    }
}
