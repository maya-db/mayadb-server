use std::fs::{OpenOptions};
use super::super::Document;
use borsh::to_writer;
use std::io;


pub fn save_on_disk(document: &Document, path: String) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    let writer = io::BufWriter::new(file);

    to_writer(writer, document)?;

    Ok(())
}