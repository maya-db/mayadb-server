use crate::core::{storage::{create_dirs, mount_path, save_on_disk}, Validation};
use std::collections::BTreeMap;
use super::{
    InsertOptions,
    super::{
        paths,
        Type,
        Document,
    },
};

pub struct Insert;

impl Insert {
    fn generate_columns_and_document(document: &mut Document, options: &mut InsertOptions) -> Result<Document, &'static str> {
        let mut document_size: usize = 0;

        let mut new_data: Document = BTreeMap::new();

        let id = Type::U64(options.auto_increment + 1);

        new_data.insert("id".to_string(), id.clone());

        for (column, value) in document {
            let e = Validation::check_document_size(
                &value,
                &mut document_size,
                options.document_max_size,
            )?;

            let value = value.clone();

            let column = column.to_lowercase();

            new_data.insert(
                column,
                value,
            );
        }

        options.data.push(new_data.clone());

        options.auto_increment += 1;

        Ok(
            BTreeMap::from([("id".to_string(), id)])
        )
    }

    pub fn insert_one(data: Document, options: &mut InsertOptions) -> Result<Document, &'static str> {
        let mut data = data;

        Self::create_path(options);

        let inserted = Self::generate_columns_and_document(
            &mut data,
            options,
        )?;

        let path = format!(
            "{}{}{}",
            paths::DATA,
            paths::DOCUMENTS,
            inserted.get("id").unwrap().to_string()
        );

        let _ = save_on_disk(
            &options.data[0],
            path,
        );

        Ok(inserted)
    }

    pub fn insert_many(data: Vec<Document>, options: &mut InsertOptions) -> Result<Vec<Document>, &'static str> {
        let mut data = data;

        Self::create_path(options);

        let mut inserted_documents = Vec::with_capacity(data.len());

        let path = mount_path(paths::DATA);

        let path = format!(
            "{}/{}/{}",
            path,
            options.collection_name,
            paths::DOCUMENTS
        );

        for document in data.iter_mut() {
            let inserted = Self::generate_columns_and_document(
                document,
                options,
            );

            if inserted.is_err() {
                return Err(inserted.err().unwrap());
            }

            let inserted = inserted.unwrap();

            inserted_documents.push(inserted.clone());

            let path = format!(
                "{}/{}",
                path,
                inserted.get("id").unwrap().to_string()
            );

            save_on_disk(
                options.data.last().unwrap(),
                path,
            ).expect("Erro ao salvar o documento");
        }

        Ok(inserted_documents)
    }

    fn create_path(options: &InsertOptions) {
        let data_dir = mount_path(paths::DATA);

        create_dirs(
            &data_dir,
            &options.collection_name,
        ).unwrap();
    }
}
