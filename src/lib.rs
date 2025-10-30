mod battle;
mod pokemon;
mod ptype;
mod stat;

pub mod prelude {
    pub use crate::battle::{Player, Side, Team};
    pub use crate::pokemon::Pokemon;
}
