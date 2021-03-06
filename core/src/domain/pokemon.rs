use std::{cmp::PartialOrd, convert::TryFrom};

#[derive(PartialEq, Clone, PartialOrd, Ord, Eq, Debug)]
pub struct PokemonNumber(u16);

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(n: PokemonNumber) -> Self {
        n.0
    }
}

#[cfg(test)]
impl PokemonNumber {
    pub fn pikachu() -> Self {
        Self(25)
    }

    pub fn charmander() -> Self {
        Self(4)
    }

    pub fn bad() -> Self {
        Self(0)
    }
}

#[derive(Clone, Debug)]
pub struct PokemonName(String);

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<PokemonName> for String {
    fn from(n: PokemonName) -> Self {
        n.0
    }
}

#[cfg(test)]
impl PokemonName {
    pub fn pikachu() -> Self {
        Self(String::from("Pikachu"))
    }

    pub fn charmander() -> Self {
        Self(String::from("Charmander"))
    }

    pub fn bad() -> Self {
        Self(String::from(""))
    }
}

#[derive(Clone, Debug)]
pub struct PokemonTypes(Vec<PokemonType>);

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match PokemonType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

impl From<PokemonTypes> for Vec<String> {
    fn from(pts: PokemonTypes) -> Self {
        let mut ts = vec![];
        for pt in pts.0.into_iter() {
            ts.push(String::from(pt));
        }
        ts
    }
}

#[cfg(test)]
impl PokemonTypes {
    pub fn pikachu() -> Self {
        Self(vec![PokemonType::Electric])
    }

    pub fn charmander() -> Self {
        Self(vec![PokemonType::Fire])
    }
}

#[derive(Clone, Debug)]
enum PokemonType {
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Fire" => Ok(Self::Fire),
            "Water" => Ok(Self::Water),
            "Grass" => Ok(Self::Grass),
            "Electric" => Ok(Self::Electric),
            "Ice" => Ok(Self::Ice),
            "Fighting" => Ok(Self::Fighting),
            "Poison" => Ok(Self::Poison),
            "Ground" => Ok(Self::Ground),
            "Flying" => Ok(Self::Flying),
            "Psychic" => Ok(Self::Psychic),
            "Bug" => Ok(Self::Bug),
            "Rock" => Ok(Self::Rock),
            "Ghost" => Ok(Self::Ghost),
            "Dragon" => Ok(Self::Dragon),
            _ => Err(()),
        }
    }
}

impl From<PokemonType> for String {
    fn from(t: PokemonType) -> String {
        String::from(match t {
            PokemonType::Fire => "Fire",
            PokemonType::Water => "Water",
            PokemonType::Grass => "Grass",
            PokemonType::Electric => "Electric",
            PokemonType::Ice => "Ice",
            PokemonType::Fighting => "Fighting",
            PokemonType::Poison => "Poison",
            PokemonType::Ground => "Ground",
            PokemonType::Flying => "Flying",
            PokemonType::Psychic => "Psychic",
            PokemonType::Bug => "Bug",
            PokemonType::Rock => "Rock",
            PokemonType::Ghost => "Ghost",
            PokemonType::Dragon => "Dragon",
        })
    }
}

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}

#[cfg(test)]
impl Pokemon {
    pub fn pikachu() -> Self {
        Self {
            number: PokemonNumber::pikachu(),
            name: PokemonName::pikachu(),
            types: PokemonTypes::pikachu(),
        }
    }

    pub fn charmander() -> Self {
        Self {
            number: PokemonNumber::charmander(),
            name: PokemonName::charmander(),
            types: PokemonTypes::charmander(),
        }
    }
}
