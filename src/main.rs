use std::sync::Arc;

use p_showdown::prelude::*;
use p_showdown::stat::{EV, IV, StatVec};

fn main() {
    let ivs = StatVec::<IV>::from_int(31, 31, 31, 31, 31, 31).unwrap();
    let evs = StatVec::<EV>::from_int(0, 0, 0, 0, 0, 0).unwrap();
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
