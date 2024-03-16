use super::super::types::Document;
use std::collections::BTreeMap;

pub struct Find;

impl Find {
    pub fn find_one(&self, filters: Document) -> Document {
        let mut found: Document = BTreeMap::new();

        for (column, value) in filters {
            let column = column.to_lowercase();

            // falta escrever a lógica de busca
        }

        found
    }
}