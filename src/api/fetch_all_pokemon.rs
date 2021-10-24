use rocket::State;
use rocket::http::Status;
use rocket::serde::{Serialize, json::Json};
use crate::domain::fetch_all_pokemon;
use crate::api::RepositoryState;

#[derive(Serialize)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>
}

#[get("/pokemons")]
pub fn serve(state: &State<RepositoryState>) -> Result<Json<Vec<Response>>, Status> {
    let res = fetch_all_pokemon::execute(state.repo.clone());

    match res {
        Ok(pokemons) => Ok(Json(pokemons.into_iter()
            .map(|p| Response {
                number: p.number,
                name: p.name,
                types: p.types
            })
            .collect::<Vec<Response>>())),
        Err(fetch_all_pokemon::Error::Unknown) => Err(Status::InternalServerError)
    }
}