use serde::Deserialize;
use std::fmt;

use crate::content::GenericItem;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BulletSize {
    Compact,
    Derringer,
    Flare,
    Long,
    Medium,
    Shell,
    Special,
}

impl fmt::Display for BulletSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Compact => "Compact",
                Self::Derringer => "Derringer",
                Self::Flare => "Flare",
                Self::Long => "Long",
                Self::Medium => "Medium",
                Self::Shell => "Shell",
                Self::Special => "Special",
            }
        )
    }
}

impl BulletSize {
    pub fn to_svg_path(&self, weapon: Option<&GenericItem>) -> String {
        match weapon {
            Some(weapon) => {
                match &*weapon.name {
                    // Special exceptions for some special weapons.
                    // Bomb Lance actually uses the same image as CrossbowBolt
                    "Crossbow" | "Bomb Lance" => "/images/bullets/CrossbowBolt.svg".to_string(),
                    "Hand Crossbow" => "/images/bullets/Bolt.svg".to_string(),
                    "Hunting Bow" => "/images/bullets/Arrow.svg".to_string(),
                    "Dolch 96" => "/images/bullets/Dolch96.svg".to_string(),
                    "Nitro Express Rifle" => "/images/bullets/Nitro.svg".to_string(),
                    _ => format!("/images/bullets/{}.svg", self).replace(' ', ""),
                }
            }
            None => format!("/images/bullets/{}.svg", self).replace(' ', ""),
        }
    }
}
