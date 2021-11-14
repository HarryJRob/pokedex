use core::repositories::in_memory_repository::InMemoryRepository;
mod commands;

use commands::{create_pokemon, delete_pokemon, fetch_all_pokemon, fetch_pokemon};
use dialoguer::{theme::ColorfulTheme, Select};
use std::sync::Arc;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let choices = [
        "Fetch all pokemon",
        "Fetch a pokemon",
        "Create a pokemon",
        "Delete a pokemon",
        "Exit",
    ];

    loop {
        let command = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Make your choice!")
            .items(&choices)
            .default(0)
            .interact()
        {
            Ok(index) => index,
            _ => continue,
        };

        match command {
            0 => fetch_all_pokemon::run(repo.clone()),
            1 => fetch_pokemon::run(repo.clone()),
            2 => create_pokemon::run(repo.clone()),
            3 => delete_pokemon::run(repo.clone()),
            4 => break,
            _ => continue,
        }
    }
}
