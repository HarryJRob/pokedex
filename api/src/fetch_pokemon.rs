use crate::RepositoryState;
use core::use_cases::fetch_pokemon;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::State;

#[derive(Serialize)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[get("/pokemons/<number>")]
pub fn serve(number: u16, state: &State<RepositoryState>) -> Result<Json<Response>, Status> {
    let req = fetch_pokemon::Request { number };

    let res = fetch_pokemon::execute(state.repo.clone(), req);

    match res {
        Ok(pokemon) => Ok(Json(Response {
            number: pokemon.number,
            name: pokemon.name,
            types: pokemon.types,
        })),
        Err(fetch_pokemon::Error::NotFound) => Err(Status::NotFound),
        Err(fetch_pokemon::Error::BadRequest) => Err(Status::BadRequest),
        Err(fetch_pokemon::Error::Unknown) => Err(Status::InternalServerError),
    }
}
