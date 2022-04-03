use super::RepositoryState;
use core::use_cases::create_pokemon;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

#[derive(Deserialize, Clone)]
pub struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[derive(Serialize)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[post("/pokemons", data = "<request>")]
pub fn serve(
    request: Json<Request>,
    state: &State<RepositoryState>,
) -> Result<Json<Response>, Status> {
    let req = create_pokemon::Request {
        number: request.number,
        name: request.name.clone(),
        types: request.types.clone(),
    };

    let res = create_pokemon::execute(state.repo.clone(), req);

    match res {
        Ok(create_pokemon::Response {
            number,
            name,
            types,
        }) => Ok(Json(Response {
            number,
            name,
            types,
        })),
        Err(create_pokemon::Error::BadRequest) => Err(Status::BadRequest),
        Err(create_pokemon::Error::Conflict) => Err(Status::BadRequest),
        Err(create_pokemon::Error::Unknown) => Err(Status::InternalServerError),
    }
}
