use core::repositories::Repository;
use core::create_pokemon;
use std::sync::Arc;

use super::{prompt_name, prompt_number, prompt_type};

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();
    let name = prompt_name();
    let types = prompt_type();

    let req = match (number, name, types) {
        (Ok(number), Ok(name), Ok(types)) => create_pokemon::Request {
            number,
            name,
            types
        },
        _ => {
            println!("An error occurred while creating a pokemon");
            return;
        }
    };

    match core::create_pokemon::execute(repo, req) {
        Ok(res) => println!("Pokemon '{}' created with id '{}'", res.name, res.number),
        Err(create_pokemon::Error::BadRequest) => println!("The request is invalid"),
        Err(create_pokemon::Error::Conflict) => println!("The Pokemon already exists"),
        Err(create_pokemon::Error::Unknown) => println!("An unknown error occurred")
    }
}