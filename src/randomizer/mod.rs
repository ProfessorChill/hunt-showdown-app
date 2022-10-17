//! This is for the loadout randomizer seen in `src/pages/loadout.rs`

pub mod budget;
pub mod config;
pub mod loadout;

pub use budget::Budget;
pub use config::Config;
pub use loadout::Loadout;

pub enum LoadoutInvalid {
    WeaponSlot(u8),
    ToolSlot(u8),
}
