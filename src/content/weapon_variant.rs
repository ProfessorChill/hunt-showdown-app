use std::fmt;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WeaponVariant {
    Alamo,
    Aperture,
    Avtomat,
    Bayonet,
    Brawler,
    Carbine,
    CarbineDeadeye,
    Chain,
    Claw,
    Compact,
    CompactStriker,
    CompactDeadeye,
    Deadeye,
    Extended,
    Handcannon,
    Hatchet,
    Marksman,
    Match,
    MusketBayonet,
    Obrez,
    ObrezDrum,
    ObrezMace,
    Precision,
    Riposte,
    Silencer,
    Sniper,
    Spitfire,
    Swift,
    Talon,
    Vandal,
    VandalStriker,
    VandalDeadeye,
}

impl fmt::Display for WeaponVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alamo => "Alamo",
                Self::Aperture => "Aperture",
                Self::Avtomat => "Avtomat",
                Self::Bayonet => "Bayonet",
                Self::Brawler => "Brawler",
                Self::Carbine => "Carbine",
                Self::CarbineDeadeye => "Carbine Deadeye",
                Self::Claw => "Claw",
                Self::Chain => "Chain",
                Self::Compact => "Compact",
                Self::CompactStriker => "Compact Striker",
                Self::CompactDeadeye => "Compact Deadeye",
                Self::Deadeye => "Deadeye",
                Self::Extended => "Extended",
                Self::Handcannon => "Handcannon",
                Self::Hatchet => "Hatchet",
                Self::Marksman => "Marksman",
                Self::Match => "Match",
                Self::MusketBayonet => "Musket Bayonet",
                Self::Obrez => "Obrez",
                Self::ObrezMace => "Obrez Mace",
                Self::ObrezDrum => "Obrez Drum",
                Self::Precision => "Precision",
                Self::Riposte => "Riposte",
                Self::Silencer => "Silencer",
                Self::Sniper => "Sniper",
                Self::Spitfire => "Spitfire",
                Self::Swift => "Swift",
                Self::Talon => "Talon",
                Self::Vandal => "Vandal",
                Self::VandalStriker => "Vandal Striker",
                Self::VandalDeadeye => "Valdal Deadeye",
            }
        )
    }
}
