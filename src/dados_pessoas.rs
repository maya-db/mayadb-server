use std::collections::BTreeMap;
use super::collection::Type as Tipo;

pub fn dados_pessoas(campos: u8) -> Vec<BTreeMap<&'static str, Tipo>> {
    let mut valores: Vec<BTreeMap<&str, Tipo>> = vec![BTreeMap::new()];

    (for i in 0..1_000_000 {
        let mut p: BTreeMap<&str, Tipo>  = BTreeMap::new();
        p.insert("nome", Tipo::String("Fulano".to_string()));

        if campos > 2 {
            p.insert("idade", Tipo::U8(10));
        }

        if campos > 3 {
            p.insert("salario", Tipo::U32(1500));
        }

        if campos > 4 {
            p.insert("email", Tipo::String("fulano@fulano".to_string()));
        }

        valores.push(p);
    });

    // valores.append(
    //     BTreeMap::from([
    //         ("nome", Tipo::String(f.name())),
    //         ("salario", Tipo::U32(f.random_number(1000))),
    //         ("email" , Tipo::String(f.email())),
    //     ])
    // );

    valores
}