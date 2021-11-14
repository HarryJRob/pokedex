#[macro_use]
extern crate clap;

use core::repositories::in_memory_repository::InMemoryRepository;
mod commands;

use std::sync::Arc;
use clap::{App, Arg};
use commands::create_pokemon;
use dialoguer::{Select, theme::ColorfulTheme};

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let choices = [
        "Fetch all pokemon",
        "Fetch a pokemon",
        "Create a pokemon",
        "Delete a pokemon",
        "Exit"
    ];

    loop {
        let command = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Make your choice!")
            .items(&choices)
            .default(0)
            .interact()
        {
            Ok(index) => index,
            _ => continue
        };

        match command {
            2 => create_pokemon::run(repo.clone()),
            4 => break,
            _ => continue
        }
    }
}
