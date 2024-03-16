use crate::core::{Type as Tipo, Document};
use std::collections::BTreeMap;

pub fn dados_pessoas(campos: u8) -> Vec<Document> {
    let mut valores: Vec<Document> = vec![BTreeMap::new()];

    (for i in 0..1_000_000 {
        let mut p: Document  = BTreeMap::new();
        p.insert("nome".to_string(), Tipo::Str("Cosmo André".to_string()));

        if campos > 2 {
            p.insert("idade".to_string(), Tipo::U8(10));
        }

        if campos > 3 {
            p.insert("salario".to_string(), Tipo::U32(1500));
        }

        if campos > 4 {
            p.insert("email".to_string(), Tipo::Str("cosmo@mail.com".to_string()));
        }

        valores.push(p);
    });

    valores
}