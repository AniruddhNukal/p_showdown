use crate::pokemon::{PokeMini, Pokemon};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    fn new() -> Side {
        Side::Left
    }

    fn opposite(s: Side) -> Side {
        match s {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

enum Screens {
    LightScreen,
    Reflect,
    Mist,
}

enum Traps {
    Bind,
    Clamp,
    FireSpin,
    Wrap,
    LeechSeed,
}

struct Battle {
    pokemon1: Pokemon,
    pokemon2: Pokemon,
    field: Field,
    player1: Box<dyn Player>,
    player2: Box<dyn Player>,
}

impl Battle {
    pub fn new(mut player1: Box<dyn Player>, mut player2: Box<dyn Player>) -> Self {
        let side1 = Side::new();
        let side2 = Side::opposite(side1);

        player1.set_side(side1);
        player2.set_side(side2);

        let field = Field::new(side1, side2);

        let pokemon1 = player1.first();
        let pokemon2 = player2.first();

        Self {
            pokemon1,
            pokemon2,
            field,
            player1,
            player2,
        }
    }
}

struct Field {
    side1: FieldSide,
    side2: FieldSide,
}

impl Field {
    fn new(side1: Side, side2: Side) -> Self {
        let side1 = FieldSide::new(side1);
        let side2 = FieldSide::new(side2);

        Self { side1, side2 }
    }
}

struct FieldSide {
    side: Side,
    screens: Vec<Screens>,
    traps: Vec<Traps>,
}

impl FieldSide {
    fn new(side: Side) -> Self {
        Self {
            side,
            screens: vec![],
            traps: vec![],
        }
    }
}

pub trait Player {
    fn set_side(&mut self, side: Side);
    fn first(&self) -> Pokemon;
}

pub struct Team {
    team: [PokeMini; 6],
}

impl Team {
    fn get(&self, i: usize) -> Pokemon {
        Pokemon::from_mini(&self.team[i])
    }
}
