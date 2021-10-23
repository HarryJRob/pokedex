use std::sync::Arc;
use crate::repositories::pokemon::InMemoryRepository;

mod health;
mod create_pokemon;

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    let mut repo = InMemoryRepository::new();

    rocket::build()
        .manage(Arc::new(repo))
        .mount("/", routes![health::serve, create_pokemon::serve])
}