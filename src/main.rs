use std::collections::{BTreeMap, HashMap};
use collection::{Collection, Type};
use dados_pessoas::dados_pessoas;
mod dados_pessoas;
mod collection;

fn main() {
    let mut pessoas = Collection::init("pessoas");

    let _ = pessoas.load_from_disk();

    //pessoas.save_on_disk().unwrap();

    let mut dados0 = &mut dados_pessoas(0);

    for i in dados0.drain(0..10) {
        println!("tamanho de d: {}", i.len());
        
    }

    println!("tamanho original de dados: {}", dados0.len());

  //  let d = dados0.drain(0..10);




    // let dados1 = dados_pessoas(3);
    // let dados2 = dados_pessoas(1);
    // let dados2 = dados_pessoas(2);
    // let dados3 = dados_pessoas(4);

    // dados0.extend(dados1);

    // dados0.extend(dados2);

    // dados0.extend(dados3);

    // pessoas.insert_many(dados0.clone());

    let dados = pessoas.get_data();
    println!("Quantidade de pessoas: {}", dados.len());

    //pessoas.inserir_muitos(dados1, dados2);
}
