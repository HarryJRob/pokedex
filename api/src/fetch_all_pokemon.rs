use crate::RepositoryState;
use core::fetch_all_pokemon;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::State;

#[derive(Serialize)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[get("/pokemons")]
pub fn serve(state: &State<RepositoryState>) -> Result<Json<Vec<Response>>, Status> {
    let res = fetch_all_pokemon::execute(state.repo.clone());

    match res {
        Ok(pokemons) => Ok(Json(
            pokemons
                .into_iter()
                .map(|p| Response {
                    number: p.number,
                    name: p.name,
                    types: p.types,
                })
                .collect::<Vec<Response>>(),
        )),
        Err(fetch_all_pokemon::Error::Unknown) => Err(Status::InternalServerError),
    }
}
