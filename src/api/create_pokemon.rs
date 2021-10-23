use std::sync::Arc;
use rocket::State;
use rocket::serde::{Serialize, Deserialize, json::Json};
use crate::repositories::pokemon::InMemoryRepository;
use crate::domain::create_pokemon;

#[derive(Deserialize)]
pub struct Request {
    number: u16,
    name: String,
    types: Vec<String>
}

#[post("/pokemons", data = "<request>")]
pub fn serve(request: Json<Request>, repo: &State<Arc<InMemoryRepository>>) -> String {
    let res = create_pokemon::execute(repo, create_pokemon::Request {
        number: request.number,
        name: request.name,
        types: request.types
    });

    "".to_string()
}