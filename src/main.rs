use std::sync::Arc;

use p_showdown::prelude::*;
use p_showdown::stat::{EVSpread, IVSpread};

fn main() {
    let ivs = IVSpread::new(&[31u16, 31, 31, 31, 31, 31]).unwrap();
    let evs = EVSpread::new(&[4u16, 0, 0, 252, 0, 252]).unwrap();
    let p = PokeMini::new(Arc::from("pikapew"), 100, Arc::from("Pikachu"), ivs, evs).unwrap();
    dbg!(p);
}

// struct PrototypePlayer {
//     team: Team,
// }

// impl PrototypePlayer {
//     fn new(team: Team) -> Self {
//         Self { team }
//     }
// }

// impl Player for PrototypePlayer {
//     fn get_team(&self) -> Team {
//         self.team.clone()
//     }
// }
