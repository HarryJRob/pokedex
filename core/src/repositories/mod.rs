use crate::domain::pokemon::{Pokemon, PokemonNumber};

pub mod in_memory_repository;
pub mod mysql_repository;

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

pub enum FetchOneError {
    NotFound,
    Unknown,
}

pub enum DeleteError {
    NotFound,
    Unknown,
}

pub trait Repository: Send + Sync {
    fn insert(&self, pokemon: Pokemon) -> Result<Pokemon, InsertError>;
    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, FetchOneError>;
    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError>;
    fn delete(&self, number: PokemonNumber) -> Result<(), DeleteError>;
}
