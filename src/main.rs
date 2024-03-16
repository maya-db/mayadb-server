use core::{Collection, write::{Insert, InsertOptions}};
use dados_pessoas::dados_pessoas;
use std::time::Instant;
mod dados_pessoas;
mod core;

fn main() {
    let mut pessoas = Collection::init("pessoas3");

    //let _ = pessoas.load_from_disk(p);

    let now = Instant::now();

    let dados0 = dados_pessoas(5);

    println!("Finalização montagem de dados: {}", now.elapsed().as_secs());

    println!("Existe {}", dados0.len());

    let mut options = InsertOptions::new(
        "pessoas4".to_string(),
        1024 * 10, // 10kb
        dados0.len(),
    );

    let _ = Insert::insert_many(dados0, &mut options);

    println!("Finalização do insert de dados: {}", now.elapsed().as_secs());
}
