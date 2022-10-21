use serde::Deserialize;
use std::fmt;

use crate::content::{BulletSize, GenericItem};

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BulletVariant {
    ChaosBolt,
    ChokeBolt,
    ConcertinaArrow,
    DragonBreath,
    DumDum,
    Explosive,
    ExplosiveBolt,
    Flechette,
    FragArrow,
    FullMetalJacket,
    HighVelocity,
    Incendiary,
    PennyShot,
    Poison,
    PoisonArrow,
    PoisonBolt,
    ShotBolt,
    Shredder,
    Slug,
    Spitzer,
    Starshell,
}

impl fmt::Display for BulletVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ChaosBolt => "Chaos Bolt",
                Self::ChokeBolt => "Choke Bolt",
                Self::ConcertinaArrow => "Concertina Arrow",
                Self::DragonBreath => "Dragon Breath",
                Self::DumDum => "Dum Dum",
                Self::Explosive => "Explosive",
                Self::ExplosiveBolt => "Explosive Bolt",
                Self::Flechette => "Flechette",
                Self::FragArrow => "Frag Arrow",
                Self::FullMetalJacket => "Full Metal Jacket",
                Self::HighVelocity => "High Velocity",
                Self::Incendiary => "Incendiary",
                Self::PennyShot => "Penny Shot",
                Self::Poison => "Poison",
                Self::PoisonArrow => "Poison Arrow",
                Self::PoisonBolt => "Poison Bolt",
                Self::ShotBolt => "Shotbolt",
                Self::Shredder => "Shredder",
                Self::Slug => "Slug",
                Self::Spitzer => "Spitzer",
                Self::Starshell => "Starshell",
            }
        )
    }
}

impl BulletVariant {
    pub fn to_svg_path(&self, weapon: Option<&GenericItem>, size: &BulletSize) -> String {
        weapon.map_or_else(
            || {
                format!(
                    "/images/bullets/{size}{}.svg",
                    self.to_string().replace(' ', "")
                )
            },
            |weapon| match &*weapon.name {
                // Special exceptions for some special weapons.
                "Crossbow" => format!(
                    "/images/bullets/Crossbow{}.svg",
                    self.to_string().replace(' ', "").replace("Bolt", "")
                ),
                "Hand Crossbow" => format!(
                    "/images/bullets/Bolt{}.svg",
                    self.to_string().replace(' ', "").replace("Bolt", ""),
                ),
                "Hunting Bow" => format!(
                    "/images/bullets/Arrow{}.svg",
                    self.to_string().replace(' ', "").replace("Arrow", "")
                ),
                "Nitro Express Rifle" => format!(
                    "/images/bullets/Nitro{}.svg",
                    self.to_string().replace(' ', "")
                ),
                _ => format!(
                    "/images/bullets/{}{}.svg",
                    size,
                    self.to_string().replace(' ', "")
                ),
            },
        )
    }
}
