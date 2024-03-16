use std::io::{self, BufReader};
use borsh::from_reader;
use std::fs::File;
use super::{
    mount_path,
    super::Document,
};


pub fn load_from_disk(path: &str, document: &mut Document, collection_name: &str) -> io::Result<()> {
    let path = mount_path(
        format!("{collection_name}").as_str(),
    );

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    *document = from_reader(&mut reader)?;

    Ok(())
}