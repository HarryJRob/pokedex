use std::convert::TryFrom;

use mysql::{params, prelude::Queryable, Opts};

use super::*;
use crate::entities::*;

pub struct MySqlRepository {
    pool: mysql::Pool,
}

impl MySqlRepository {
    pub fn try_new(connection_string: String) -> Self {
        let opts = match Opts::from_url(&connection_string) {
            Ok(opts) => opts,
            Err(error) => panic!("{:?}", error),
        };

        let pool = match mysql::Pool::new(opts) {
            Ok(pool) => pool,
            Err(error) => panic!("{:?}", error),
        };

        Self { pool }
    }
}

impl Repository for MySqlRepository {
    fn insert(&self, pokemon: Pokemon) -> Result<Pokemon, InsertError> {
        let mut connection = match self.pool.get_conn() {
            Ok(connection) => connection,
            Err(_) => return Err(InsertError::Unknown),
        };

        let pokemon_id = u16::from(pokemon.number.clone());
        let pokemon_name = String::from(pokemon.name.clone());
        let pokemon_types = Vec::<String>::from(pokemon.types.clone())
            .iter()
            .map(|t| format!("'{}'", t))
            .collect::<Vec<String>>()
            .join(",");

        let stmt =
            match connection.prep(r"INSERT INTO pokemons VALUES (:pokemon_id, :pokemon_name)") {
                Ok(stmt) => stmt,
                Err(_) => return Err(InsertError::Unknown),
            };

        match connection.exec_drop(stmt, params! { pokemon_id, pokemon_name }) {
            Ok(_) => (),
            Err(_) => return Err(InsertError::Conflict),
        };

        let type_ids: Vec<u16> = match connection.query_map(
            format!(
                "SELECT type_id FROM types WHERE type_name IN ({})",
                pokemon_types
            ),
            |id: u16| id,
        ) {
            Ok(result) => result,
            Err(_) => return Err(InsertError::Unknown),
        };

        let stmt =
            match connection.prep(r"INSERT INTO pokemon_types VALUES (:pokemon_id, :type_id)") {
                Ok(stmt) => stmt,
                Err(_) => return Err(InsertError::Unknown),
            };

        match connection.exec_batch(
            stmt,
            type_ids.iter().map(|type_id| {
                params! {
                    pokemon_id,
                    type_id
                }
            }),
        ) {
            Ok(_) => Ok(pokemon),
            Err(_) => Err(InsertError::Unknown),
        }
    }

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, FetchOneError> {
        let mut connection = match self.pool.get_conn() {
            Ok(connection) => connection,
            Err(_) => return Err(FetchOneError::Unknown),
        };

        let pokemon_id = u16::from(number);

        let pokemon = match connection.prep(
            r"SELECT pokemon_id, pokemon_name 
        FROM pokemons
        WHERE pokemon_id = :pokemon_id",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Err(FetchOneError::NotFound),
        };

        let pokemon: Option<(u16, String)> =
            match connection.exec_first(&pokemon, params! { pokemon_id }) {
                Ok(result) => result,
                Err(_) => return Err(FetchOneError::Unknown),
            };

        let pokemon = match pokemon {
            Some(pokemon) => pokemon,
            None => return Err(FetchOneError::NotFound),
        };

        let pokemon_types = match connection.prep(
            r"SELECT type_name 
        FROM pokemon_types pt
        INNER JOIN types t ON t.type_id = pt.type_id
        WHERE pt.pokemon_id = :pokemon_id",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Err(FetchOneError::Unknown),
        };

        let pokemon_types: Vec<String> =
            match connection.exec_map(pokemon_types, params! { pokemon_id }, |result: String| {
                result
            }) {
                Ok(pokemon_types) => pokemon_types,
                Err(_) => return Err(FetchOneError::Unknown),
            };

        let pokemon_number = match PokemonNumber::try_from(pokemon.0) {
            Ok(number) => number,
            Err(_) => return Err(FetchOneError::Unknown),
        };
        let pokemon_name = match PokemonName::try_from(pokemon.1) {
            Ok(name) => name,
            Err(_) => return Err(FetchOneError::Unknown),
        };

        let pokemon_types = match PokemonTypes::try_from(pokemon_types) {
            Ok(types) => types,
            Err(_) => return Err(FetchOneError::Unknown),
        };

        Ok(Pokemon {
            number: pokemon_number,
            name: pokemon_name,
            types: pokemon_types,
        })
    }

    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError> {
        let mut connection = match self.pool.get_conn() {
            Ok(connection) => connection,
            Err(_) => return Err(FetchAllError::Unknown),
        };

        let res = match connection.query_map(
            r"SELECT p.pokemon_id, p.pokemon_name
            FROM pokemons p",
            |p: (u16, String)| p,
        ) {
            Ok(result) => result,
            Err(_) => return Err(FetchAllError::Unknown),
        };

        let pokemons = res
            .iter()
            .map(|p| {
                let pokemon_id = PokemonNumber::try_from(p.0).unwrap();
                let pokemon_name = PokemonName::try_from(p.1.clone()).unwrap();

                let pokemon_types = connection
                    .prep(
                        r"SELECT type_name 
                        FROM pokemon_types pt
                        INNER JOIN types t ON t.type_id = pt.type_id
                        WHERE pt.pokemon_id = :pokemon_id",
                    )
                    .unwrap();

                let pokemon_types: Vec<String> = connection
                    .exec_map(
                        pokemon_types,
                        params! { "pokemon_id" => p.0 },
                        |result: String| result,
                    )
                    .unwrap();

                let pokemon_types = PokemonTypes::try_from(pokemon_types).unwrap();

                Pokemon {
                    number: pokemon_id,
                    name: pokemon_name,
                    types: pokemon_types,
                }
            })
            .collect();

        Ok(pokemons)
    }

    fn delete(&self, number: PokemonNumber) -> Result<(), DeleteError> {
        let mut connection = match self.pool.get_conn() {
            Ok(connection) => connection,
            Err(_) => return Err(DeleteError::Unknown),
        };

        let pokemon_id = u16::from(number);

        let stmt = match connection.prep(r"DELETE FROM pokemons WHERE pokemon_id = :pokemon_id") {
            Ok(stmt) => stmt,
            Err(_) => return Err(DeleteError::Unknown),
        };

        match connection.exec_drop(stmt, params! { pokemon_id }) {
            Ok(_) => Ok(()),
            Err(_) => return Err(DeleteError::Unknown),
        }
    }
}
