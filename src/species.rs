use std::sync::Arc;

use rusqlite::{Connection, params};
use thiserror::Error;

use crate::ptype::{PType, PTypeError, PTypePair};
use crate::stat::{BaseStat, Stat, StatError, StatVec};

#[derive(Debug, Error)]
pub enum SpeciesError {
    #[error("The database at {db_name} was not able to be accessed. rusqlite: {err}")]
    DBConnectionFail {
        err: rusqlite::Error,
        db_name: String,
    },
    #[error("The query for {name} was unable to be prepared. rusqlite: {err}")]
    PrepareFail { err: rusqlite::Error, name: String },
    #[error("Unable to read value at index {idx} in for query on {name}. rusqlite: {err}")]
    ReadFail {
        err: rusqlite::Error,
        name: String,
        idx: usize,
    },
    #[error("The PType does not exist: {0}")]
    BadPType(PTypeError),
    #[error("The Stat is invalide: {0}")]
    BadStat(StatError),
}

pub struct Species {
    pub name: Arc<str>,
    pub ptype_pair: PTypePair,
    pub basestats: StatVec<BaseStat>,
    // movepool: ...
}

struct RawSpecies {
    name: String,
    type_1: String,
    type_2: String,
    hp: u16,
    atk: u16,
    def: u16,
    spa: u16,
    spd: u16,
    spe: u16,
}

impl Species {
    pub fn from_name(name: Arc<str>) -> Result<Species, SpeciesError> {
        let db_path = "pokemon.db";
        let conn = Connection::open(db_path).map_err(|err| SpeciesError::DBConnectionFail {
            err,
            db_name: db_path.to_string(),
        })?;

        let mut statement = conn
            .prepare("SELECT * FROM pokemon_stats WHERE name = ?")
            .map_err(|err| SpeciesError::PrepareFail {
                err,
                name: name.to_string(),
            })?;

        let raw = statement
            .query_row(params![name], |row| {
                Ok(RawSpecies {
                    name: row.get(0)?,
                    type_1: row.get(1)?,
                    type_2: row.get(2)?,
                    hp: row.get(3)?,
                    atk: row.get(4)?,
                    def: row.get(5)?,
                    spa: row.get(6)?,
                    spd: row.get(7)?,
                    spe: row.get(8)?,
                })
            })
            .map_err(|err| SpeciesError::ReadFail {
                err,
                name: name.to_string(),
                idx: 0,
            })?;

        Species::from_raw(raw)
    }

    fn from_raw(raw: RawSpecies) -> Result<Self, SpeciesError> {
        let name: Arc<str> = Arc::from(raw.name.as_str());
        let type1 = PType::try_from(raw.type_1).map_err(SpeciesError::BadPType)?;
        let type2 = PType::try_from(raw.type_2).map_err(SpeciesError::BadPType)?;
        let ptype_pair = PTypePair::new(type1, type2);
        let stats: StatVec<BaseStat> =
            StatVec::from_int(raw.hp, raw.atk, raw.def, raw.spa, raw.spd, raw.spe)
                .map_err(SpeciesError::BadStat)?;
        Ok(Species {
            name,
            ptype_pair,
            basestats: stats,
        })
    }
}
