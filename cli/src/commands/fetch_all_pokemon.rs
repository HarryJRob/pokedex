use core::{fetch_all_pokemon, repositories::Repository};
use std::sync::Arc;

#[derive(Debug)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>
}


pub fn run(repo: Arc<dyn Repository>) {
    match fetch_all_pokemon::execute(repo) {
        Ok(res) => res.into_iter().for_each(|pokemon| {
            println!(
                "{:?}",
                Response {
                    number: pokemon.number,
                    name: pokemon.name,
                    types: pokemon.types,
                }
            )
        }),
        Err(fetch_all_pokemon::Error::Unknown) => println!("An unknown error occurred")
    }
}