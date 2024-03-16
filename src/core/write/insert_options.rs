use super::super::Document;


pub struct InsertOptions {
    pub collection_name: String,
    pub document_max_size: usize,
    pub auto_increment: u64,
    pub data: Vec<Document>,
}

impl InsertOptions {
    pub fn new(collection_name: String, document_max_size: usize, initial_capacity: usize) -> Self {
        Self {
            collection_name,
            document_max_size,
            auto_increment: 0,
            data: Vec::with_capacity(initial_capacity),
        }
    }
}