use std::fs:: create_dir;
use super::super::paths;
use std::path::Path;
use std::io::Result;

pub fn create_dirs(data_dir: &String, colection_name: &String) -> Result<()>{
    if Path::new(&data_dir).exists() == false {
        create_dir(&data_dir)?;
    }

    let collection_dir = format!(
        "{}/{}",
        data_dir,
        colection_name,
    );

    if Path::new(&collection_dir).exists() == false {
        create_dir(&collection_dir)?;
    }

    let documents_dir = format!(
        "{}/{}",
        collection_dir,
        paths::DOCUMENTS,
    );

    if Path::new(&documents_dir).exists() == false {
        create_dir(&documents_dir)?;
    }

    let indexes_dir = format!(
        "{}/{}",
        collection_dir,
        paths::INDEXES,
    );

    if Path::new(&indexes_dir).exists() == false {
        create_dir(&indexes_dir)?;
    }

    Ok(())
}