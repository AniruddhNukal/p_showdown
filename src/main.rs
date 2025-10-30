use p_showdown::prelude::*;

fn main() {}

struct PrototypePlayer {
    team: Team,
}

impl PrototypePlayer {
    fn new(team: Team) -> Self {
        Self { team }
    }
}

impl Player for PrototypePlayer {
    fn get_team(&self) -> Team {
        self.team.clone()
    }
}
