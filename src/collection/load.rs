use std::io::{self, BufReader, BufWriter};
use std::fs::{File, OpenOptions};
use super::Collection;
use std::path::Path;


impl Collection {
    fn path(&self) -> String {
        format!(
            "/home/cosmo9able/Desktop/workspace/maya-db/data/{}.maya",
            self.name,
        )
    }

    pub fn save_on_disk(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(self.path())?;

        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    // Método para ler do arquivo, desserializar e atualizar a struct
    pub fn load_from_disk(&mut self) -> io::Result<()> {
        if !Path::exists(self.path().as_str().as_ref()) {
            // se o arquivo não existir, crie um
            self.save_on_disk()?;
        }

        let file = File::open(self.path())?;
        let reader = BufReader::new(file);
        *self = serde_json::from_reader(reader)?;
        Ok(())
    }
}