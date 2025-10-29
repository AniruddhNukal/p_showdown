#[derive(Debug, Clone)]
pub struct PTypePair {
    primary: PType,
    secondary: PType,
}

impl PTypePair {
    pub fn new(primary: PType, secondary: PType) -> Self {
        Self { primary, secondary }
    }

    pub fn type_effective(&self, attacker: &PType) -> f64 {
        PType::type_ef(attacker, &self.primary) * PType::type_ef(attacker, &self.secondary)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PType {
    Null,
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    Stellar,
}

use PType::*;

impl PType {
    fn type_ef(attacking: &PType, defending: &PType) -> f64 {
        match (attacking, defending) {
            (Null, _) => 1.,
            (Normal, t) => match t {
                Rock | Steel => 0.5,
                Ghost => 0.,
                _ => 1.,
            },
            (Fighting, t) => match t {
                Normal | Rock | Steel | Ice | Dark => 2.,
                Flying | Poison | Bug | Psychic | Fairy => 0.5,
                Ghost => 0.,
                _ => 1.,
            },
            (Flying, t) => match t {
                Fighting | Bug | Grass => 2.,
                Rock | Steel | Electric => 0.5,
                _ => 1.,
            },
            (Poison, t) => match t {
                Grass | Fairy => 2.,
                Poison | Ground | Rock | Ghost => 0.5,
                Steel => 0.,
                _ => 1.,
            },
            (Ground, t) => match t {
                Poison | Rock | Steel | Fire | Electric => 2.,
                Bug | Grass => 0.5,
                Flying => 0.,
                _ => 1.,
            },
            (Rock, t) => match t {
                Flying | Bug | Fire | Ice => 2.,
                Fighting | Ground | Steel => 0.5,
                _ => 1.,
            },
            (Bug, t) => match t {
                Grass | Psychic | Dark => 2.,
                Fighting | Flying | Poison | Ghost | Steel | Fire | Fairy => 0.5,
                _ => 1.,
            },
            (Ghost, t) => match t {
                Ghost | Psychic => 2.,
                Dark => 0.5,
                Normal => 0.,
                _ => 1.,
            },
            (Steel, t) => match t {
                Rock | Ice | Fairy => 2.,
                Steel | Fire | Water | Electric => 0.5,
                _ => 1.,
            },
            (Fire, t) => match t {
                Bug | Steel | Grass | Ice => 2.,
                Rock | Fire | Water | Dragon => 0.5,
                _ => 1.,
            },
            (Water, t) => match t {
                Ground | Rock | Fire => 2.,
                Water | Grass | Dragon => 0.5,
                _ => 1.,
            },
            (Grass, t) => match t {
                Ground | Rock | Water => 2.,
                Flying | Poison | Bug | Steel | Fire | Grass | Dragon => 0.5,
                _ => 1.,
            },
            (Electric, t) => match t {
                Flying | Water => 2.,
                Grass | Electric | Dragon => 0.5,
                Ground => 0.,
                _ => 1.,
            },
            (Psychic, t) => match t {
                Fighting | Poison => 2.,
                Steel | Psychic => 0.5,
                Dark => 0.,
                _ => 1.,
            },
            (Ice, t) => match t {
                Flying | Ground | Grass | Dragon => 2.,
                Steel | Fire | Water | Ice => 0.5,
                _ => 1.,
            },
            (Dragon, t) => match t {
                Dragon => 2.,
                Steel => 0.5,
                Fairy => 0.,
                _ => 1.,
            },
            (Dark, t) => match t {
                Ghost | Psychic => 2.,
                Fighting | Dark | Fairy => 0.5,
                _ => 1.,
            },
            (Fairy, t) => match t {
                Fighting | Dragon | Dark => 2.,
                Poison | Steel | Fire => 0.5,
                _ => 1.,
            },
            (Stellar, _) => 1.,
        }
    }
}
