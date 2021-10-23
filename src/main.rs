mod api;
mod domain;
mod repositories;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    match api::rocket().launch().await {
        Ok(_) => (),
        Err(_) => (),
    };
}
