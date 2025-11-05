use std::sync::Arc;

use thiserror::Error;

use crate::{
    ptype::{PType, PTypePair},
    species::{self, Species, SpeciesError},
    stat::{EVSpread, IVSpread, StageVec},
};

#[derive(Debug, Error)]
pub enum PokemonError {
    #[error("Species failed to generate: {0}")]
    SpeciesFail(SpeciesError),
}

pub struct Pokemon {
    level: u8,
    stats: [u16; 6],
    species_name: Arc<str>,
    nickname: Arc<str>,
    ptypes: PTypePair,
    // moves: MoveSet,
    curr_hp: u16,
    stages: StageVec,
}

#[derive(Debug, Clone)]
pub struct PokeMini {
    level: u8,
    stats: [u16; 6],
    species_name: Arc<str>,
    nickname: Arc<str>,
    ptypes: PTypePair,
    // moves: MoveSet,
    curr_hp: u16,
    // pers_effects: (Some effect storing type)
}

impl Pokemon {
    pub fn from_mini(mini: &PokeMini) -> Self {
        let PokeMini {
            level,
            stats,
            species_name,
            nickname,
            ptypes,
            // moves: MoveSet,
            curr_hp,
            // pers_effects: (Some effect storing type)
        } = mini;

        Pokemon {
            level: *level,
            stats: stats.clone(),
            species_name: species_name.clone(),
            nickname: nickname.clone(),
            ptypes: ptypes.clone(),
            curr_hp: *curr_hp,
            // moves: MoveSet,
            stages: StageVec::new(),
            // pers_effects: ...
        }
    }
}

impl PokeMini {
    pub fn new(
        nickname: Arc<str>,
        level: u8,
        species: Arc<str>,
        ivs: IVSpread,
        evs: EVSpread,
        /* moveset: MoveSet */
    ) -> Result<PokeMini, PokemonError> {
        let species = Species::from_name(species).map_err(PokemonError::SpeciesFail)?;
        let Species {
            name: species_name,
            basestats,
            ptype_pair,
        } = species;
        let stats = calculate_stats(level, basestats, ivs.get_stats(), evs.get_stats());
        let curr_hp = stats[0];

        Ok(PokeMini {
            level,
            stats,
            species_name,
            nickname,
            ptypes: ptype_pair,
            curr_hp,
        })
    }
}

fn calculate_stats(level: u8, basestats: [u16; 6], ivs: [u16; 6], evs: [u16; 6]) -> [u16; 6] {
    let l = u16::from(level);
    let b = basestats;
    let iv = ivs;
    let ev = evs;

    let hp1 = ev[0] / 4;
    let hp2 = (2 * b[0] + iv[0] + hp1 * l) / 100;
    let hp = hp2 + l + 10;
    let mut o = [hp, 0, 0, 0, 0, 0];
    for i in 1..=5 {
        let s1 = ev[i] / 4;
        let s2 = 2 * b[i] + iv[i] + s1;
        let s3 = s2 * l / 100;
        let s = s3 + 5;
        o[i] = s;
    }

    o
}
