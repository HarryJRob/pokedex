use std::sync::Arc;
use rocket::State;
use rocket::serde::{Serialize, Deserialize, json::Json};
use crate::repositories::pokemon::Repository;
use crate::domain::create_pokemon;

#[derive(Deserialize, Clone)]
pub struct Request {
    number: u16,
    name: String,
    types: Vec<String>
}

#[post("/pokemons", data = "<request>")]
pub fn serve(request: Json<Request>, state: &State<Arc<dyn Repository>>) -> String {
    let req = create_pokemon::Request {
        number: request.number,
        name: request.name.clone(),
        types: request.types.clone()
    };

    let res = create_pokemon::execute(state.inner().clone(), req);

    match res {
        create_pokemon::Response::Ok(id) => id.to_string(),
        create_pokemon::Response::BadRequest => "BadRequest".to_string(),
        create_pokemon::Response::Conflict => "Conflict".to_string(),
        create_pokemon::Response::Error => "An unexpected error".to_string()
    }
}