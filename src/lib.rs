pub mod battle;
pub mod pokemon;
mod ptype;
mod species;
pub mod stat;

pub mod prelude {
    pub use crate::battle::{Battle, Player, Team};
    pub use crate::pokemon::{PokeMini, Pokemon};
}
