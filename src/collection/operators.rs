use std::collections::{BTreeMap, HashMap};
use super::{Collection, Type};


impl Collection {
    pub fn get_data(&self) -> &Vec<BTreeMap<u8, Type>> {
        &self.data
    }

    pub fn get_columns(&self) -> &BTreeMap<String, u8> {
        &self.columns
    }

    pub fn get_nome(&self) -> &str {
        &self.name
    }

    fn generate_columns_and_document(&mut self, dados: &mut BTreeMap<&str, Type>) -> BTreeMap<String, Type> {
        let mut new_data: BTreeMap<u8, Type> = BTreeMap::new();

        self.auto_increment += 1;

        new_data.insert(0, Type::U64(self.auto_increment));

        for (column, value) in dados {
            let value = value.clone();

            let column = column.to_lowercase().leak();

            if self.columns.contains_key(column) {
                new_data.insert(
                    self.columns[column],
                    value,
                );

                self.data.push(new_data.clone());

                continue;
            }

            self.number_of_columns += 1;

            self.columns.insert(
                column.to_string(),
                self.number_of_columns,
            );

            new_data.insert(
                self.columns[column],
                value,
            );

            self.data.push(new_data.clone());
        }

        BTreeMap::from([
            ("id".to_string(), new_data.get(&0).unwrap().clone()),
        ])
    }

    pub fn insert_one(&mut self, data: BTreeMap<&str, Type>) -> BTreeMap<String, Type> {
        let mut data = data;

        let document = self.generate_columns_and_document(&mut data);

        let _ = self.save_on_disk();

        document
    }

    pub fn insert_many(&mut self, data: Vec<BTreeMap<&str, Type>>) -> Vec<BTreeMap<String, Type>> {
        let mut documents: Vec<BTreeMap<String, Type>> = Vec::new();

        for d in data {
            let mut d = d;
            let document = self.generate_columns_and_document(&mut d);

            documents.push(document);
        }

        let _ = self.save_on_disk();

        documents
    }

    pub fn find_one(&self, filters: HashMap<&str, Type>) -> BTreeMap<u8, Type> {
        let mut found: BTreeMap<u8, Type> = BTreeMap::new();

        for (column, value) in filters {
            let column = column.to_lowercase().leak();

            if let Some(id) = self.columns.get(column) {
                self.data[*id as usize].get(&0).unwrap();
            }
        }

        found
    }
}
