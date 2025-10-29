use std::sync::Arc;

use crate::{
    ptype::PTypePair,
    stat::{BaseStat, StageVec, StatVec},
};

pub struct Pokemon {
    level: u8,
    stats: StatVec<BaseStat>,
    species_name: Arc<str>,
    nickname: Arc<str>,
    ptypes: PTypePair,
    // moves: MoveSet,
    curr_hp: BaseStat,
    stages: StageVec,
}

pub struct PokeMini {
    level: u8,
    stats: StatVec<BaseStat>,
    species_name: Arc<str>,
    nickname: Arc<str>,
    ptypes: PTypePair,
    // moves: MoveSet,
    curr_hp: BaseStat,
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
