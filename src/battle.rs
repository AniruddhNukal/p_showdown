use std::sync::Arc;

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

pub struct Battle {
    pokemon1: Pokemon,
    pokemon2: Pokemon,
    field: Field,
    handler1: PlayerHandler,
    handler2: PlayerHandler,
}

impl Battle {
    pub fn new(mut player1: Arc<dyn Player>, mut player2: Arc<dyn Player>) -> Self {
        let side1 = Side::new();
        let side2 = Side::opposite(side1);

        let handler1 = PlayerHandler::new(player1, side1);
        let handler2 = PlayerHandler::new(player2, side2);

        let field = Field::new(side1, side2);

        let pokemon1 = handler1.first();
        let pokemon2 = handler2.first();

        Self {
            pokemon1,
            pokemon2,
            field,
            handler1,
            handler2,
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
    fn get_team(&self) -> Team;
}

struct PlayerHandler {
    player: Arc<dyn Player>,
    team: Team,
    side: Side,
}

impl PlayerHandler {
    fn new(player: Arc<dyn Player>, side: Side) -> PlayerHandler {
        PlayerHandler {
            player: player.clone(),
            team: player.get_team(),
            side,
        }
    }

    fn first(&self) -> Pokemon {
        self.team.get(0)
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    team: [PokeMini; 6],
}

impl Team {
    fn get(&self, i: usize) -> Pokemon {
        Pokemon::from_mini(&self.team[i])
    }
}
