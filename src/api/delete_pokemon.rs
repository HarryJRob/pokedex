use rocket::State;
use rocket::http::Status;
use rocket::serde::Serialize;
use crate::domain::delete_pokemon;
use crate::api::RepositoryState;

#[derive(Serialize)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>
}

#[delete("/pokemons/<number>")]
pub fn serve(number: u16, state: &State<RepositoryState>) -> Status {
    let req = delete_pokemon::Request { number };

    let res = delete_pokemon::execute(state.repo.clone(), req);

    match res {
        Ok(()) => Status::Ok,
        Err(delete_pokemon::Error::NotFound) => Status::NotFound,
        Err(delete_pokemon::Error::BadRequest) => Status::BadRequest,
        Err(delete_pokemon::Error::Unknown) => Status::InternalServerError
    }
}