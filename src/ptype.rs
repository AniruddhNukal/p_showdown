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

use PType as P;

impl PType {
    fn type_ef(attacking: &PType, defending: &PType) -> f64 {
        match (attacking, defending) {
            (P::Null, _) => 1.,
            (P::Normal, t) => match t {
                P::Rock | P::Steel => 0.5,
                P::Ghost => 0.,
                _ => 1.,
            },
            (P::Fighting, t) => match t {
                P::Normal | P::Rock | P::Steel | P::Ice | P::Dark => 2.,
                P::Flying | P::Poison | P::Bug | P::Psychic | P::Fairy => 0.5,
                P::Ghost => 0.,
                _ => 1.,
            },
            (P::Flying, t) => match t {
                P::Fighting | P::Bug | P::Grass => 2.,
                P::Rock | P::Steel | P::Electric => 0.5,
                _ => 1.,
            },
            (P::Poison, t) => match t {
                P::Grass | P::Fairy => 2.,
                P::Poison | P::Ground | P::Rock | P::Ghost => 0.5,
                P::Steel => 0.,
                _ => 1.,
            },
            (P::Ground, t) => match t {
                P::Poison | P::Rock | P::Steel | P::Fire | P::Electric => 2.,
                P::Bug | P::Grass => 0.5,
                P::Flying => 0.,
                _ => 1.,
            },
            (P::Rock, t) => match t {
                P::Flying | P::Bug | P::Fire | P::Ice => 2.,
                P::Fighting | P::Ground | P::Steel => 0.5,
                _ => 1.,
            },
            (P::Bug, t) => match t {
                P::Grass | P::Psychic | P::Dark => 2.,
                P::Fighting | P::Flying | P::Poison | P::Ghost | P::Steel | P::Fire | P::Fairy => {
                    0.5
                }
                _ => 1.,
            },
            (P::Ghost, t) => match t {
                P::Ghost | P::Psychic => 2.,
                P::Dark => 0.5,
                P::Normal => 0.,
                _ => 1.,
            },
            (P::Steel, t) => match t {
                P::Rock | P::Ice | P::Fairy => 2.,
                P::Steel | P::Fire | P::Water | P::Electric => 0.5,
                _ => 1.,
            },
            (P::Fire, t) => match t {
                P::Bug | P::Steel | P::Grass | P::Ice => 2.,
                P::Rock | P::Fire | P::Water | P::Dragon => 0.5,
                _ => 1.,
            },
            (P::Water, t) => match t {
                P::Ground | P::Rock | P::Fire => 2.,
                P::Water | P::Grass | P::Dragon => 0.5,
                _ => 1.,
            },
            (P::Grass, t) => match t {
                P::Ground | P::Rock | P::Water => 2.,
                P::Flying | P::Poison | P::Bug | P::Steel | P::Fire | P::Grass | P::Dragon => 0.5,
                _ => 1.,
            },
            (P::Electric, t) => match t {
                P::Flying | P::Water => 2.,
                P::Grass | P::Electric | P::Dragon => 0.5,
                P::Ground => 0.,
                _ => 1.,
            },
            (P::Psychic, t) => match t {
                P::Fighting | P::Poison => 2.,
                P::Steel | P::Psychic => 0.5,
                P::Dark => 0.,
                _ => 1.,
            },
            (P::Ice, t) => match t {
                P::Flying | P::Ground | P::Grass | P::Dragon => 2.,
                P::Steel | P::Fire | P::Water | P::Ice => 0.5,
                _ => 1.,
            },
            (P::Dragon, t) => match t {
                P::Dragon => 2.,
                P::Steel => 0.5,
                P::Fairy => 0.,
                _ => 1.,
            },
            (P::Dark, t) => match t {
                P::Ghost | P::Psychic => 2.,
                P::Fighting | P::Dark | P::Fairy => 0.5,
                _ => 1.,
            },
            (P::Fairy, t) => match t {
                P::Fighting | P::Dragon | P::Dark => 2.,
                P::Poison | P::Steel | P::Fire => 0.5,
                _ => 1.,
            },
            (P::Stellar, _) => 1.,
        }
    }
}
