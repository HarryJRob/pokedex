use std::sync::Arc;
use crate::repositories::pokemon::{InMemoryRepository, Repository};

mod health;
mod create_pokemon;
mod fetch_all_pokemon;
mod fetch_pokemon;
mod delete_pokemon;

pub struct RepositoryState {
    repo: Arc<dyn Repository>
}

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    let repo = Arc::new(InMemoryRepository::new());

    rocket::build()
        .manage(RepositoryState { repo })
        .mount("/", routes![
            health::serve, 
            create_pokemon::serve,
            fetch_pokemon::serve,
            fetch_all_pokemon::serve,
            delete_pokemon::serve])
}