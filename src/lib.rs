mod battle;
mod pokemon;
mod ptype;
mod species;
mod stat;

pub mod prelude {
    pub use crate::battle::{Battle, Player, Team};
    pub use crate::pokemon::Pokemon;
}
