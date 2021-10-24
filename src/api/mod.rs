use std::sync::Arc;
use crate::repositories::pokemon::{InMemoryRepository, Repository};

mod health;
mod create_pokemon;

pub struct RepositoryState {
    repo: Arc<dyn Repository>
}

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    let repo = Arc::new(InMemoryRepository::new());

    rocket::build()
        .manage(RepositoryState { repo })
        .mount("/", routes![health::serve, create_pokemon::serve])
}