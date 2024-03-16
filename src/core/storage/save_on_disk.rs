use std::{fs::OpenOptions, io::Write};
use super::super::Document;
use borsh::to_writer;
use std::io;


pub fn save_on_disk(document: &Document, path: String) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    let mut writer = io::BufWriter::new(file);

    to_writer(&mut writer, document)?;

    writer.flush()?;

    Ok(())
}