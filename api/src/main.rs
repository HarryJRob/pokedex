use core::repositories::in_memory_repository::InMemoryRepository;
use core::repositories::Repository;
use std::sync::Arc;

mod create_pokemon;
mod delete_pokemon;
mod fetch_all_pokemon;
mod fetch_pokemon;
mod health;

pub struct RepositoryState {
    repo: Arc<dyn Repository>,
}

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let repo = Arc::new(InMemoryRepository::new());

    rocket::build().manage(RepositoryState { repo }).mount(
        "/",
        routes![
            health::serve,
            create_pokemon::serve,
            fetch_pokemon::serve,
            fetch_all_pokemon::serve,
            delete_pokemon::serve
        ],
    )
}
